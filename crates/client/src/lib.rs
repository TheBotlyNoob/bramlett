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
}

pub type Result<T, E = ClientError> = std::result::Result<T, E>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum GameStatus {
    NotDownloaded,
    /// Downloading - (current, total)
    #[serde(skip)]
    Downloading(watch::Receiver<(u64, u64)>),
    /// Installing (unzipping) - (current, total)
    #[serde(skip)]
    Installing(watch::Receiver<(u64, u64)>),
    Running,
    Stopped,
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
    pub fn conf_dir() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("bramletts games config"))
            .join("Bramletts Games")
    }
    pub fn file() -> PathBuf {
        Self::conf_dir().join("config.json")
    }
    pub fn save(&self) -> Result<()> {
        let config_dir = Self::conf_dir();
        let _ = std::fs::create_dir_all(config_dir);
        let config_file = Self::file();
        let config_file = std::fs::File::create(config_file)?;
        serde_json::to_writer_pretty(config_file, self)?;
        Ok(())
    }
    pub fn games_dir(&self) -> PathBuf {
        self.games_dir.read().unwrap().clone()
    }
    pub fn saves_dir(&self) -> PathBuf {
        self.saves_dir.read().unwrap().clone()
    }
    pub fn games(&self) -> Arc<DashMap<GameId, Game>> {
        self.games.clone()
    }

    pub fn set_games_dir(&self, games_dir: PathBuf) {
        *self.games_dir.write().unwrap() = games_dir;
    }
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

pub async fn update_game_list(config: &mut Config) -> Result<()> {
    let games_list = reqwest::get("http://localhost:8000")
        .await?
        .json::<Vec<GameInfo>>()
        .await?;

    for game_info in games_list.into_iter() {
        let game = Game {
            info: game_info,
            status: GameStatus::NotDownloaded,
        };
        config.games.insert(game.info.id, game);
    }

    Ok(())
}
