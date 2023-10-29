use std::{
    net::{Ipv4Addr, Ipv6Addr, SocketAddr},
    path::PathBuf,
};

use axum::{http::HeaderValue, routing::get};
use client::{update_game_list, Config};
use common::GameId;
use rspc::{ExportConfig, Rspc};
use tower_http::cors::{AllowOrigin, CorsLayer};
use tracing_subscriber::{filter::LevelFilter, EnvFilter};

#[derive(thiserror::Error, serde::Serialize, specta::Type, Debug)]
pub enum Error {
    #[error("game not found")]
    GameNotFound,
    #[error("game already downloading")]
    GameCantDownload,
    #[error("client error: {0}")]
    Client(
        #[serde(skip)]
        #[from]
        client::ClientError,
    ),
}

const R: Rspc<Config, Error> = Rspc::new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    let config_file = Config::file();

    tracing::trace!("config file: {config_file:#?}");

    let mut config = if config_file.exists() {
        let config_file = std::fs::File::open(config_file)?;
        serde_json::from_reader(config_file)?
    } else {
        let config = Config::default();
        config.save()?;
        config
    };

    tracing::trace!("updating game list...");

    if let Err(e) = update_game_list(&mut config).await {
        tracing::warn!("failed to update game list: {e:#}");
    } else {
        config.save()?;
    };

    tracing::info!("save dir: {:#?}", config.saves_dir());
    tracing::info!("games dir: {:#?}", config.games_dir());
    tracing::info!("{} games", config.games().len());

    let router = R
        .router()
        .procedure("version", R.query(|_, _: ()| Ok(env!("CARGO_PKG_VERSION"))))
        .procedure(
            "gameState",
            R.query(|config, game: GameId| async move {
                config
                    .games()
                    .get(&game)
                    .map(|game| game.status.clone())
                    .ok_or(Error::GameNotFound)
            }),
        )
        .procedure(
            "downloadGame",
            R.mutation(|config, game: GameId| async move {
                let games = config.games();
                let game = games.get(&game).ok_or(Error::GameNotFound)?;
                if !matches!(game.status, client::GameStatus::NotDownloaded) {
                    return Err(Error::GameCantDownload);
                }

                let game_dir = config.game_dir(game.info.id);
                let _ = std::fs::create_dir_all(&game_dir);
                let game_dir = game_dir.join("Saves");
                let _ = std::fs::create_dir_all(game_dir);

                config.save()?;

                Ok(())
            }),
        )
        .procedure(
            "games",
            R.query(|config, _: ()| {
                Ok((*config.games())
                    .clone()
                    .into_iter()
                    .collect::<std::collections::HashMap<_, _>>())
            }),
        )
        .build()
        .unwrap()
        .arced(); // This function is a shortcut to wrap the router in an `Arc`.

    router
        .export_ts(ExportConfig::new(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("frontend/src/lib/bindings.ts"),
        ))
        .unwrap();

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "4000".to_string())
        .parse()
        .unwrap();

    let cors = CorsLayer::new().allow_origin(AllowOrigin::list([
        HeaderValue::from_str(&format!("http://localhost:{port}")).unwrap(),
        HeaderValue::from_str(&format!("http://127.0.0.1:{port}")).unwrap(),
    ]));

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello 'rspc'!" })) // TODO: display frontend here
        .nest(
            "/rspc",
            rspc_httpz::endpoint(router, move |req: rspc_httpz::Request| {
                tracing::info!("client requested operation '{}'", req.uri().path());
                config.clone()
            })
            .axum(),
        )
        .layer(cors);

    let addr = SocketAddr::from((Ipv4Addr::new(127, 0, 0, 1), port));

    tracing::info!("listening on http://{addr}/rspc");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(Into::into)
}
