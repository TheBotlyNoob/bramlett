use bramlett::{Ctx, Game, GameStatus};
use common::GameId;
use dashmap::DashMap;
use juniper::{graphql_object, EmptySubscription, FieldResult, GraphQLEnum, RootNode};
use std::sync::Arc;
use tokio::{process::Command, sync::watch};

#[derive(Debug, Copy, Clone, thiserror::Error)]
pub enum GraphQLError {
    #[error("game not found")]
    NotFound,
    #[error("game already downloaded, downloading or installing")]
    AlreadyDownloaded,
    #[error("game running, downloading, installing, or not downloaded")]
    NotDeleted,
}

pub struct GraphQLGame(pub GameId, Arc<DashMap<GameId, Game>>);

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
    pub fn get(&self) -> FieldResult<Game> {
        self.1
            .get(&self.0)
            .map(|g| g.value().clone())
            .ok_or_else(|| GraphQLError::NotFound.into())
    }
}

// Field resolvers implementation
#[graphql_object(context = Ctx)]
impl GraphQLGame {
    pub fn id(&self) -> FieldResult<i32> {
        Ok(self.get()?.info.id.0)
    }
    pub fn name(&self) -> FieldResult<String> {
        Ok(self.get()?.info.name)
    }
    pub fn icon(&self) -> FieldResult<String> {
        Ok(self.get()?.info.icon)
    }
    pub fn status(&self) -> FieldResult<GraphQLGameStatus> {
        Ok(GraphQLGameStatus::from(self.get()?.status))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, GraphQLEnum)]
pub enum GraphQLGameStatusInner {
    NotDownloaded,
    Downloading,
    Installing,
    Running,
    Ready,
}

pub struct Void;

#[graphql_object]
impl Void {
    #[allow(clippy::self_named_constructors)]
    const fn void() -> Self {
        Self
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GraphQLGameStatus {
    pub status: GraphQLGameStatusInner,
    #[serde(skip)]
    pub progress: Option<watch::Receiver<(u64, u64)>>,
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
            GameStatus::Ready => Self {
                status: GraphQLGameStatusInner::Ready,
                progress: None,
            },
        }
    }
}

#[graphql_object(context = Ctx)]
impl GraphQLGameStatus {
    pub const fn status(&self) -> GraphQLGameStatusInner {
        self.status
    }
    /// Progress in megabytes
    pub fn progress(&self) -> std::option::Option<[i32; 2]> {
        #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
        self.progress
            .as_ref()
            .map(|p| *p.borrow())
            .map(|(num, denom)| {
                if self.status == GraphQLGameStatusInner::Downloading {
                    [
                        (num as f32 / 1e+6) as i32, // make it megabytes b/c bytes are too big to fit in i32
                        (denom as f32 / 1e+6) as i32,
                    ]
                } else {
                    [num as i32, denom as i32]
                }
            })
    }
}

pub struct Query;

#[graphql_object(context = Ctx)]
impl Query {
    pub fn game(context: &Ctx, id: i32) -> Option<GraphQLGame> {
        GraphQLGame::new(GameId(id), context.config.games())
    }
    pub fn games(context: &Ctx) -> Vec<GraphQLGame> {
        let mut games = context
            .config
            .games()
            .iter()
            .map(|k| GraphQLGame(*k.key(), context.config.games())) // we don't call GraphQLGame::new here because we know the game exists
            .collect::<Vec<_>>();
        games.sort_unstable_by_key(|g| g.0); // needed b/c DashMap doesn't guarantee order
        games
    }
}

pub struct Mutation;

#[graphql_object(context = Ctx)]
impl Mutation {
    pub fn download(ctx: &Ctx, game: GameId) -> FieldResult<Void> {
        let games = ctx.config.games();
        let (tx, rx) = watch::channel((0, 0));
        let game = {
            let mut game = games.get_mut(&game).ok_or(GraphQLError::NotFound)?;
            if matches!(
                game.status,
                GameStatus::Downloading(_) | GameStatus::Installing(_)
            ) {
                return Err(GraphQLError::AlreadyDownloaded.into());
            }
            game.status = GameStatus::Downloading(rx);
            game.clone()
        };
        tracing::info!("downloading game: {game:?}");
        tokio::spawn({
            let ctx = ctx.clone();
            async move {
                let bytes = bramlett::download::download_game(game.clone(), ctx.clone(), tx)
                    .await
                    .unwrap();
                tracing::info!("downloaded game: {game:?}; extracting...");
                tokio::task::spawn_blocking(move || {
                    let (tx, rx) = watch::channel((0, 0));
                    games.get_mut(&game.info.id).unwrap().status = GameStatus::Installing(rx);
                    bramlett::download::extract_zip_with_password(
                        &bytes,
                        &ctx.config.game_dir(game.info.id),
                        "game",
                        tx,
                    )
                    .unwrap();

                    games.get_mut(&game.info.id).unwrap().status = GameStatus::Ready;
                    ctx.config.save().unwrap();
                });
            }
        });
        Ok(Void)
    }

    pub fn run(ctx: &Ctx, game: GameId) -> FieldResult<Void> {
        let games = ctx.config.games();
        let game = {
            let mut game = games.get_mut(&game).ok_or(GraphQLError::NotFound)?;
            game.status = GameStatus::Running;
            game.clone()
        };
        tracing::info!("running game: {game:?}");
        let ctx = ctx.clone();
        tokio::spawn(async move {
            let game_dir = ctx.config.game_dir(game.info.id);
            let exe = game_dir.join(&game.info.exe);

            if let Ok(mut child) = Command::new(&exe)
                .current_dir(exe.parent().unwrap_or(&game_dir))
                .spawn()
            {
                let _ = child.wait().await;
            }

            tracing::info!("game stopped: {game:?}");

            let mut game = games.get_mut(&game.info.id).unwrap();
            game.status = GameStatus::Ready;
        });
        Ok(Void)
    }

    pub async fn delete(ctx: &Ctx, game: GameId) -> FieldResult<Void> {
        let games = ctx.config.games();

        {
            let mut game = games.get_mut(&game).ok_or(GraphQLError::NotFound)?;
            if !matches!(game.status, GameStatus::Ready) {
                return Err(GraphQLError::NotDeleted.into());
            }

            tracing::info!("deleting game: {game:?}");

            tokio::fs::remove_dir_all(ctx.config.game_dir(game.info.id)).await?;
            game.status = GameStatus::NotDownloaded;
        }

        Ok(Void)
    }

    pub async fn update_game_list(ctx: &Ctx) -> FieldResult<Void> {
        let ctx = ctx.clone();
        bramlett::update_game_list(&ctx.config, true).await?;
        Ok(Void)
    }
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Ctx>>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}
