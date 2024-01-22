use super::error::Result;
use crate::core::{
    db::{init_conn, CONNECTION},
    dirs, download,
    extract::extract_zip_with_password,
    game,
};
use flutter_rust_bridge::*;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

#[derive(sqlx::Type)]
#[repr(u8)]
#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Default, Debug)]
pub enum GameState {
    #[default]
    NotInstalled,
    Installed,
}

#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow, Clone, Debug)]
pub struct Game {
    pub name: String,
    pub exe: String,
    #[sqlx(json)]
    pub args: Vec<String>,
    pub icon: String,
    pub url: String,
    pub uuid: String,
    pub sha256: String,
    #[serde(default)]
    pub state: GameState,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Games {
    pub games: Vec<Game>,
}

pub async fn fetch_games() -> Result<Games> {
    game::fetch_games().await
}

pub async fn download_game(game: Game, progress: &Progress) -> Result<Vec<u8>> {
    download::download_game(game, progress).await
}

pub async fn run_game(game: Game) -> Result<()> {
    game::run_game(game).await
}

#[derive(Default, Clone)]
#[frb(opaque)]
pub struct Progress(Arc<(AtomicU64, AtomicU64)>);
impl Progress {
    #[frb(sync)]
    pub fn new() -> Progress {
        Self::default()
    }

    #[frb(sync)]
    pub fn increment_numerator(&self) {
        self.0 .1.fetch_add(1, Ordering::Relaxed);
    }
    #[frb(sync)]
    pub fn set_numerator(&self, numerator: u64) {
        self.0 .1.swap(numerator, Ordering::Relaxed);
    }
    #[frb(sync)]
    pub fn get_numerator(&self) -> u64 {
        self.0 .1.load(Ordering::Relaxed)
    }

    #[frb(sync)]
    pub fn increment_denominator(&self) {
        self.0 .0.fetch_add(1, Ordering::Relaxed);
    }
    #[frb(sync)]
    pub fn set_denominator(&self, denominator: u64) {
        self.0 .0.swap(denominator, Ordering::Relaxed);
    }
    #[frb(sync)]
    pub fn get_denominator(&self) -> u64 {
        self.0 .0.load(Ordering::Relaxed)
    }

    #[frb(sync)]
    pub fn is_full(&self) -> bool {
        self.get_numerator() == self.get_denominator()
    }
    #[frb(sync)]
    pub fn is_empty(&self) -> bool {
        (self.get_numerator(), self.get_denominator()) == (0, 0)
    }
}

// #[frb(sync)]
pub async fn extract_zip(bytes: Vec<u8>, game: Game, progress: &Progress) -> Result<()> {
    log::info!("unzip {}", game.name);

    let progress = progress.clone();
    let game_ = game.clone();
    tokio::task::spawn_blocking(move || {
        extract_zip_with_password(&bytes, &dirs::game_dir(&game_), "game", progress)
    })
    .await
    .unwrap()?;

    sqlx::query("UPDATE games SET state = ? WHERE uuid = ?")
        .bind(GameState::Installed)
        .bind(&game.uuid)
        .execute(CONNECTION.get().unwrap())
        .await?;

    Ok(())
}

#[frb(init)]
pub async fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();

    let _ = env_logger::builder()
        .target(env_logger::Target::Stdout)
        .try_init();

    init_conn().await;
}
