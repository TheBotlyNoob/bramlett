#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::must_use_candidate)]

use common::{GameId, GameInfo};
use dashmap::DashMap;
use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};
use tokio::sync::watch;

pub mod download;

#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("zip error: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("HTML parsing error: {0}")]
    Html(#[from] tl::ParseError),
    #[error("Google Drive HTML structure error")]
    BadDrive,
    #[error("incorrect zip password")]
    BadZipPassword,
}

pub type Result<T, E = ClientError> = std::result::Result<T, E>;

#[derive(Debug, Clone, serde::Deserialize)]
pub enum GameStatus {
    NotDownloaded,
    /// Downloading - (current, total)
    #[serde(skip)]
    Downloading(watch::Receiver<(u64, u64)>),
    /// Installing (unzipping) - (current, total)
    #[serde(skip)]
    Installing(watch::Receiver<(u64, u64)>),
    #[serde(skip)]
    Running,
    Stopped,
}

impl serde::Serialize for GameStatus {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            Self::Downloading(..) | Self::Installing(..) | Self::NotDownloaded => {
                ser.serialize_unit_variant("GameStatus", 0, "NotDownloaded")
            }
            Self::Running | Self::Stopped => ser.serialize_unit_variant("GameStatus", 4, "Stopped"),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Game {
    pub info: GameInfo,
    pub status: GameStatus,
}

/// Config
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    games_dir: Arc<RwLock<PathBuf>>,
    saves_dir: Arc<RwLock<PathBuf>>,
    games: Arc<DashMap<GameId, Game>>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            games_dir: Arc::new(RwLock::new(
                dirs::data_local_dir()
                    .unwrap_or_else(|| PathBuf::from("bramletts games local data"))
                    .join("Games"),
            )),
            saves_dir: Arc::new(RwLock::new(
                dirs::document_dir()
                    .unwrap_or_else(|| {
                        dirs::home_dir()
                            .unwrap_or_else(|| PathBuf::from("bramletts games documents"))
                    })
                    .join("Saves"),
            )),
            games: Arc::new(DashMap::new()),
        }
    }
}

impl Config {
    #[must_use = "pure function"]
    pub fn conf_dir() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("bramletts games config"))
            .join("Bramletts Games")
    }
    #[must_use = "pure function"]
    pub fn file() -> PathBuf {
        Self::conf_dir().join("config.json")
    }
    /// Saves the config to the config file.
    ///
    /// # Errors
    /// Returns an error if the config file can't be written to.
    pub fn save(&self) -> Result<()> {
        let config_dir = Self::conf_dir();
        let _ = std::fs::create_dir_all(config_dir);
        let config_file = Self::file();
        let config_file = std::fs::File::create(config_file)?;
        serde_json::to_writer_pretty(config_file, self)?;
        Ok(())
    }
    /// Gets the directory where games are stored.
    #[allow(clippy::missing_panics_doc)]
    pub fn games_dir(&self) -> PathBuf {
        self.games_dir.read().unwrap().clone()
    }
    #[allow(clippy::missing_panics_doc)]
    pub fn saves_dir(&self) -> PathBuf {
        self.saves_dir.read().unwrap().clone()
    }
    pub fn games(&self) -> Arc<DashMap<GameId, Game>> {
        self.games.clone()
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn set_games_dir(&self, games_dir: PathBuf) {
        *self.games_dir.write().unwrap() = games_dir;
    }
    #[allow(clippy::missing_panics_doc)]
    pub fn set_saves_dir(&self, saves_dir: PathBuf) {
        *self.saves_dir.write().unwrap() = saves_dir;
    }

    pub fn game_dir(&self, game_id: GameId) -> PathBuf {
        self.games_dir().join(game_id.0.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct Ctx {
    pub config: Config,
    pub client: reqwest::Client,
}

impl juniper::Context for Ctx {}

/// Updates the game list in the config file to match the server's game list.
/// Doesn't modify existing games.
///
/// # Errors
/// Returns an error if the server is unreachable, the game list is invalid, or the config file
/// can't be written to.
pub async fn update_game_list(config: &Config) -> Result<()> {
    tracing::info!("updating game list...");

    let games_list = reqwest::get(if cfg!(debug_assertions) {
        "http://localhost:8000"
    } else {
        "https://bramletts-games.shuttleapp.rs"
    })
    .await?
    .json::<Vec<GameInfo>>()
    .await?;

    for game_info in games_list {
        if config.games.contains_key(&game_info.id) {
            continue;
        }
        let game = Game {
            info: game_info,
            status: GameStatus::NotDownloaded,
        };
        config.games.insert(game.info.id, game);
    }

    Ok(())
}
