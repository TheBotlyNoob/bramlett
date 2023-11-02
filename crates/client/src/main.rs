#![warn(clippy::pedantic, clippy::nursery)]

use client::{update_game_list, Config, Ctx, Game, GameStatus};
use common::GameId;
use dashmap::DashMap;
use juniper::{graphql_object, EmptySubscription, FieldResult, GraphQLEnum, RootNode};
use std::sync::Arc;
use tokio::sync::watch;
use warp::{http::Response, Filter};

#[derive(Debug, Copy, Clone, thiserror::Error)]
pub enum GraphQLError {
    #[error("game not found")]
    GameNotFound,
}

struct GraphQLGame(pub GameId, Arc<DashMap<GameId, Game>>);

impl GraphQLGame {
    pub fn new(id: GameId, games: Arc<DashMap<GameId, Game>>) -> Option<Self> {
        // make sure the game exists
        if games.contains_key(&id) {
            Some(Self(id, games))
        } else {
            None
        }
    }
}

impl GraphQLGame {
    fn get(&self) -> FieldResult<Game> {
        self.1
            .get(&self.0)
            .map(|g| g.value().clone())
            .ok_or_else(|| GraphQLError::GameNotFound.into())
    }
}

// Field resolvers implementation
#[graphql_object(context = Ctx)]
impl GraphQLGame {
    fn id(&self) -> FieldResult<i32> {
        Ok(self.get()?.info.id.0)
    }
    fn name(&self) -> FieldResult<String> {
        Ok(self.get()?.info.name)
    }
    fn status(&self) -> FieldResult<GraphQLGameStatus> {
        Ok(GraphQLGameStatus::from(self.get()?.status))
    }
}

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize, GraphQLEnum)]
enum GraphQLGameStatusInner {
    NotDownloaded,
    Downloading,
    Installing,
    Running,
    Stopped,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct GraphQLGameStatus {
    pub status: GraphQLGameStatusInner,
    #[serde(skip)]
    pub progress: Option<watch::Receiver<(u32, u32)>>,
}

impl From<GameStatus> for GraphQLGameStatus {
    fn from(s: GameStatus) -> Self {
        match s {
            GameStatus::NotDownloaded => Self {
                status: GraphQLGameStatusInner::NotDownloaded,
                progress: None,
            },
            GameStatus::Downloading(prog) => Self {
                status: GraphQLGameStatusInner::Downloading,
                progress: Some(prog),
            },
            GameStatus::Installing(prog) => Self {
                status: GraphQLGameStatusInner::Installing,
                progress: Some(prog),
            },
            GameStatus::Running => Self {
                status: GraphQLGameStatusInner::Running,
                progress: None,
            },
            GameStatus::Stopped => Self {
                status: GraphQLGameStatusInner::Stopped,
                progress: None,
            },
        }
    }
}

#[graphql_object(context = Ctx)]
impl GraphQLGameStatus {
    const fn status(&self) -> GraphQLGameStatusInner {
        self.status
    }
    fn progress(&self) -> FieldResult<Option<[i32; 2]>> {
        Ok(
            if let Some((num, denom)) = self.progress.as_ref().map(|p| *p.borrow()) {
                Some([num.try_into()?, denom.try_into()?])
            } else {
                None
            },
        )
    }
}

struct Query;

#[graphql_object(context = Ctx)]
impl Query {
    fn game(context: &Ctx, id: i32) -> Option<GraphQLGame> {
        GraphQLGame::new(GameId(id), context.config.games())
    }
    fn games(context: &Ctx) -> Vec<GraphQLGame> {
        let mut games = context
            .config
            .games()
            .iter()
            .map(|k| GraphQLGame(*k.key(), context.config.games())) // we don't call GraphQLGame::new here because we know the game exists
            .collect::<Vec<_>>();
        games.sort_unstable_by_key(|g| g.0);
        games
    }
}

struct Mutation;

#[graphql_object(context = Ctx)]
impl Mutation {
    fn download(ctx: &Ctx, game: GameId) -> FieldResult<GraphQLGame> {
        let games = ctx.config.games();
        let mut game = games.get_mut(&game).ok_or(GraphQLError::GameNotFound)?;
        let (tx, rx) = watch::channel((0, 0));
        tokio::spawn({
            let game = game.clone();
            let ctx = ctx.clone();
            async move {
                let _ = client::download::download_game(game, ctx, tx).await;
            }
        });
        game.status = GameStatus::Downloading(rx);
        Ok(GraphQLGame(game.info.id, ctx.config.games()))
    }
}

type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Ctx>>;

fn schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(tracing::level_filters::LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    let log = warp::log("bramletts_games");

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

    let ctx = Ctx {
        config: config.clone(),
        client: reqwest::Client::new(),
    };

    let schema = Arc::new(schema());

    tracing::info!("listening on 127.0.0.1:8080");

    let routes = warp::path("graphql")
        .and(
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
            .or(warp::any().map(|| {
                Response::builder()
                    .header("content-type", "text/html")
                    .body(
                        "<html><h1>juniper_warp/subscription example</h1>
                                <div>visit <a href=\"/graphiql\">GraphiQL</a></div>
                                <div>visit <a href=\"/playground\">GraphQL Playground</a></div>
                        </html>",
                    )
            })),
        )
        .with(log)
        .with(
            warp::cors()
                .allow_origin("http://localhost:3000")
                .allow_origin(&*format!("http://localhost:8080"))
                .allow_headers(vec!["Content-Type", "User-Agent"])
                .allow_methods(vec!["OPTIONS", "GET", "POST", "DELETE"]),
        );

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;

    Ok(())
}
