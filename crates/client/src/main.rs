#![warn(clippy::pedantic, clippy::nursery)]

use bramlett::{update_game_list, Config, Ctx};
use std::sync::Arc;
use tokio::sync::oneshot;
use tracing_subscriber::{filter::filter_fn, layer::SubscriberExt, util::SubscriberInitExt, Layer};
use warp::Filter;
use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    webview::WebViewBuilder,
};

mod gql;

const DEFAULT_PORT: u16 = 8635;

#[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .without_time()
                .with_filter(
                    tracing_subscriber::EnvFilter::builder()
                        .with_default_directive(tracing::level_filters::LevelFilter::INFO.into())
                        .from_env_lossy(),
                )
                .with_filter(filter_fn(|m| !m.target().contains("graphql"))),
        )
        .init();

    #[cfg(not(debug_assertions))]
    self_update::backends::github::Update::configure()
        .repo_owner("TheBotlyNoob")
        .repo_name("bramletts-games")
        .bin_name("bramlett")
        .show_download_progress(true)
        .current_version(self_update::cargo_crate_version!())
        .no_confirm(true)
        .build()?
        .update()?;

    let config_file = Config::file();

    tracing::info!("config file: {config_file:#?}");

    let config = if config_file.exists() {
        let config_file = std::fs::File::open(config_file)?;
        match serde_json::from_reader(config_file) {
            Ok(c) => c,
            Err(e) => {
                tracing::warn!("failed to parse config file: {e:#}");
                let config = Config::default();
                config.save()?;
                config
            }
        }
    } else {
        let config = Config::default();
        config.save()?;
        config
    };

    tracing::info!("save dir: {:#?}", config.saves_dir());
    tracing::info!("games dir: {:#?}", config.games_dir());

    let ctx = Ctx {
        config: config.clone(),
        client: reqwest::Client::new(),
    };

    let schema = Arc::new(gql::schema());

    let port = std::env::var("PORT")
        .map(|p| {
            tracing::warn!(
                "using port from env var: make sure to change the port on the frontend as well."
            );
            p.parse().expect("invalid port")
        })
        .unwrap_or(DEFAULT_PORT);

    let routes = warp::path("graphql").and(
        (warp::post().and(juniper_warp::make_graphql_filter(
            schema.clone(),
            warp::any().map(move || ctx.clone()).boxed(),
        )))
        .or(warp::get()
            .and(warp::path("playground"))
            .and(juniper_warp::playground_filter(
                "/graphql",
                Some("/subscriptions"),
            )))
        .or(warp::get()
            .and(warp::path("graphiql"))
            .and(juniper_warp::graphiql_filter(
                "/graphql",
                Some("/subscriptions"),
            )))
        .with(warp::log("graphql")),
    );

    #[cfg(not(debug_assertions))]
    let routes = routes.or(warp::path::tail()
        .and_then(frontend::serve)
        .or(warp::path::end().and_then(frontend::serve_index))
        .with(warp::log("frontend")));

    let routes = routes.with(
        warp::cors()
            .allow_origin("http://localhost:3000")
            .allow_origin(&*format!("http://localhost:{port}"))
            .allow_headers(["Content-Type", "User-Agent"])
            .allow_methods(["OPTIONS", "GET", "POST", "DELETE"]),
    );

    let (tx, rx) = oneshot::channel();

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?; // the below future doesn't run if this isn't put in it's own variable. maybe a lifetime issue?
    rt.spawn(async move {
        if let Err(e) = update_game_list(&config, true).await {
            tracing::warn!("failed to update game list: {e:#} -- is the server running?");
        } else {
            config.save().unwrap();
        };
        tracing::info!("{} games", config.games().len());

        let _ = tx.send(());

        warp::serve(routes).run(([127, 0, 0, 1], port)).await;
    });

    rx.blocking_recv()?;

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Bramlett's Games")
        .build(&event_loop)?;
    let port = if cfg!(debug_assertions) { 3000 } else { port };
    let _webview = WebViewBuilder::new(window)?
        .with_url(&format!("http://localhost:{port}"))?
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => tracing::info!("wry has started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}

#[cfg(not(debug_assertions))]
mod frontend {
    use rust_embed::RustEmbed;
    use warp::{http::header::HeaderValue, path::Tail, reply::Response, Rejection, Reply};

    #[derive(RustEmbed)]
    #[folder = "frontend/out"]
    struct Frontend;

    pub async fn serve_index() -> Result<impl Reply, Rejection> {
        serve_impl("index.html")
    }

    pub async fn serve(path: Tail) -> Result<impl Reply, Rejection> {
        serve_impl(path.as_str())
    }

    fn serve_impl(path: &str) -> Result<impl Reply, Rejection> {
        let asset = Frontend::get(path).ok_or_else(warp::reject::not_found)?;
        let mime = mime_guess::from_path(path).first_or_octet_stream();

        let mut res = Response::new(asset.data.into());
        res.headers_mut().insert(
            "content-type",
            HeaderValue::from_str(mime.as_ref()).unwrap(),
        );
        Ok(res)
    }
}
