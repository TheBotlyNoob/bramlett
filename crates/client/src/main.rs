//! This example demonstrates asynchronous subscriptions with warp and tokio 0.2

use anyhow::{Context, Result};
use client::{update_game_list, Ctx, Game, GameStatus};
use common::GameId;
use dashmap::DashMap;
use juniper::{
    graphql_object, EmptyMutation, EmptySubscription, FieldResult, GraphQLEnum, RootNode,
};
use std::sync::Arc;
use tokio::sync::watch;
use warp::{http::Response, Filter};

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
            .context("game not found")
            .map_err(Into::into)
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
    pub progress: Option<(watch::Receiver<u32>, u32)>,
}

impl From<GameStatus> for GraphQLGameStatus {
    fn from(s: GameStatus) -> Self {
        match s {
            GameStatus::NotDownloaded => Self {
                status: GraphQLGameStatusInner::NotDownloaded,
                progress: None,
            },
            GameStatus::Downloading(num, denom) => Self {
                status: GraphQLGameStatusInner::Downloading,
                progress: Some((num, denom)),
            },
            GameStatus::Installing(num, denom) => Self {
                status: GraphQLGameStatusInner::Installing,
                progress: Some((num, denom)),
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
    fn status(&self) -> GraphQLGameStatusInner {
        self.status
    }
    fn progress(&self) -> FieldResult<Option<[i32; 2]>> {
        Ok(if let Some((num, denom)) = &self.progress {
            Some([(*num.borrow()).try_into()?, (*denom).try_into()?])
        } else {
            None
        })
    }
}

struct Query;

#[graphql_object(context = Ctx)]
impl Query {
    async fn game(context: &Ctx, id: i32) -> Option<GraphQLGame> {
        GraphQLGame::new(GameId(id), context.games())
    }
    async fn games(context: &Ctx) -> Vec<GraphQLGame> {
        context
            .games()
            .iter()
            .map(|k| GraphQLGame(*k.key(), context.games())) // we don't call GraphQLGame::new here because we know the game exists
            .collect()
    }
}

type Schema = RootNode<'static, Query, EmptyMutation<Ctx>, EmptySubscription<Ctx>>;

fn schema() -> Schema {
    Schema::new(Query, EmptyMutation::new(), EmptySubscription::new())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(tracing::level_filters::LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    let log = warp::log("bramletts_games");

    let config_file = Ctx::file();

    tracing::trace!("config file: {config_file:#?}");

    let mut config = if config_file.exists() {
        let config_file = std::fs::File::open(config_file)?;
        serde_json::from_reader(config_file)?
    } else {
        let config = Ctx::default();
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

    let schema = Arc::new(schema());

    tracing::info!("listening on 127.0.0.1:8080");

    let routes = warp::path("graphql")
        .and(
            (warp::post().and(juniper_warp::make_graphql_filter(
                schema.clone(),
                warp::any().map(move || config.clone()).boxed(),
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
