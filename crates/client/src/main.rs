//! This example demonstrates asynchronous subscriptions with warp and tokio 0.2

use anyhow::Result;
use client::{update_game_list, Context, Game};
use common::GameId;
use dashmap::DashMap;
use juniper::{graphql_object, EmptyMutation, EmptySubscription, GraphQLEnum, RootNode};
use std::sync::Arc;
use tokio::sync::watch;
use warp::{http::Response, Filter};

struct GraphQLGame(pub GameId, Arc<DashMap<GameId, Game>>);

impl GraphQLGame {
    fn get(&self) -> Game {
        self.1.get(&self.0).expect(
            "Game not found in the map. This should never happen, because we only send existing games."
        ).clone()
    }
}

// Field resolvers implementation
#[graphql_object(context = Context)]
impl GraphQLGame {
    fn id(&self) -> i32 {
        self.get().info.id.0
    }
    fn name(&self) -> String {
        self.get().info.name
    }
    // fn status(&self) -> GameStatus {
    //     self.get().status
    // }
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

#[graphql_object(context = Context)]
impl GraphQLGameStatus {
    fn status(&self) -> GraphQLGameStatusInner {
        self.status
    }
    fn progress(&self) -> Option<[i32; 2]> {
        self.progress.as_ref().map(|(num, denom)| {
            [
                (*num.borrow()).try_into().expect(""),
                (*denom).try_into().expect(""),
            ]
        })
    }
}

struct Query;

#[graphql_object(context = Context)]
impl Query {
    async fn game(context: &Context, id: i32) -> GraphQLGame {
        GraphQLGame(GameId(id), context.games())
    }
}

type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

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

    let log = warp::log("warp_subscriptions");

    let homepage = warp::path::end().map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(
                "<html><h1>juniper_warp/subscription example</h1>
                        <div>visit <a href=\"/graphiql\">GraphiQL</a></div>
                        <div>visit <a href=\"/playground\">GraphQL Playground</a></div>
                </html>",
            )
    });

    let config_file = Context::file();

    tracing::trace!("config file: {config_file:#?}");

    let mut config = if config_file.exists() {
        let config_file = std::fs::File::open(config_file)?;
        serde_json::from_reader(config_file)?
    } else {
        let config = Context::default();
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

    tracing::info!("Listening on 127.0.0.1:8080");

    let routes = (warp::post()
        .and(warp::path("graphql"))
        .and(juniper_warp::make_graphql_filter(
            schema.clone(),
            warp::any().map(move || config.clone()).boxed(),
        )))
    // .or(
    //     warp::path("subscriptions").and(juniper_warp::subscriptions::make_ws_filter(
    //         schema,
    //         ConnectionConfig::new(config.clone()),
    //     )),
    // )
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
    .or(homepage)
    .with(log);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;

    Ok(())
}
