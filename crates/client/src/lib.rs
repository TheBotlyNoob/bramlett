use bytes::Bytes;
use common::GameInfo;
use obfstr::obfstr;
use reqwest::Client;
use rhai::{Engine, Scope, AST};
use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicU64, Ordering::Relaxed},
        Arc,
    },
};
use sysinfo::Pid;

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
    #[error("rhai error: {0}")]
    Rhai(#[from] rhai::EvalAltResult),
    #[error("invalid rhai script: {0}")]
    InvalidRhai(#[from] rhai::ParseError),
}

type Result<T, E = ClientError> = std::result::Result<T, E>;

#[derive(Debug, Clone)]
pub struct AtomicPercent(pub Arc<(AtomicU64, AtomicU64)>);
impl AtomicPercent {
    pub fn new() -> Self {
        Self(Arc::new((AtomicU64::new(0), AtomicU64::new(1))))
    }
    pub fn get(&self) -> f32 {
        self.0 .0.load(Relaxed) as f32 / self.0 .1.load(Relaxed) as f32
    }
    pub fn get_numerator(&self) -> u64 {
        self.0 .0.load(Relaxed)
    }
    pub fn get_denominator(&self) -> u64 {
        self.0 .1.load(Relaxed)
    }
    pub fn set(&self, numerator: u64, denominator: u64) {
        self.0 .0.store(numerator, Relaxed);
        self.0 .1.store(denominator, Relaxed);
    }
    pub fn add_numerator(&self, add: u64) {
        self.0 .0.fetch_add(add, Relaxed);
    }
}
impl Default for AtomicPercent {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum GameState {
    NotDownloaded,
    Downloading(AtomicPercent),
    Downloaded(Bytes),
    Installing(AtomicPercent),
    Installed,
    Running(Pid),
    // runs once; goes back to installed
    Stopped,
}

#[derive(Debug, Clone)]
pub struct Game {
    pub info: GameInfo,
    pub game_dir: PathBuf,
    pub save_dir: PathBuf,
    pub rhai_scope: Scope<'static>,
    pub hooks_ast: AST,
    pub state: GameState,
}

pub async fn get_game_list(client: Client, rhai_engine: &Engine) -> Result<Vec<Game>> {
    client
        .get(
            #[cfg(any(not(debug_assertions), feature = "prod_in_debug"))]
            obfstr!("https://bramletts-games.shuttleapp.rs"),
            #[cfg(all(debug_assertions, not(feature = "prod_in_debug")))]
            obfstr!("http://127.0.0.1:8000"),
        ) // obfstr because... it's something to try and stop defender from flagging the exec
        .send()
        .await?
        .json::<Vec<GameInfo>>()
        .await?
        .into_iter()
        .map(|g| setup_game(g, rhai_engine))
        .collect()
}

pub fn setup_game(info: GameInfo, rhai_engine: &Engine) -> Result<Game> {
    let game_dir = dirs::data_local_dir()
        .unwrap()
        .join("Bramletts Games")
        .join(info.name.clone());
    std::fs::create_dir_all(&game_dir)?;
    let mut save_dir = dirs::home_dir().unwrap();
    for path in std::fs::read_dir(&save_dir)?.filter_map(Result::ok) {
        if path.file_type()?.is_dir() {
            let name = path.file_name().to_string_lossy().to_lowercase();
            if name.starts_with("onedrive") {
                save_dir = save_dir.join(path.file_name());
                break;
            }
        };
    } // if no onedrive, just use home dir
    let save_dir = save_dir.join("Saves").join(&info.name);
    std::fs::create_dir_all(&save_dir)?;
    let mut scope = Scope::new();

    if !cfg!(windows) {
        let mut root = std::env::var("WINEPREFIX")
            .map(PathBuf::from)
            .unwrap_or_else(|_| dirs::home_dir().unwrap().join(".wine"));
        root.push("drive_c");

        let home_dir = root.join("users").join(whoami::username());

        scope.push_constant("DATA_LOCAL_DIR", home_dir.join("AppData").join("Local"));
        let config_dir = home_dir.join("AppData").join("Roaming");
        scope.push_constant("DATA_DIR", config_dir.clone());
        scope.push_constant("CONFIG_DIR", config_dir);
        scope.push_constant("HOME_DIR", home_dir);
    } else {
        scope.push_constant("DATA_LOCAL_DIR", dirs::data_local_dir().unwrap());
        scope.push_constant("DATA_DIR", dirs::data_dir().unwrap());
        scope.push_constant("CONFIG_DIR", dirs::config_dir().unwrap());
        scope.push_constant("HOME_DIR", dirs::home_dir().unwrap());
    }

    scope.push_constant("GAME_DIR", game_dir.clone());
    scope.push_constant("SAVE_DIR", save_dir.clone());

    let hooks_ast = rhai_engine.compile(info.hooks.clone())?;
    Ok(Game {
        save_dir,
        rhai_scope: scope,
        hooks_ast,
        info,
        state: if game_dir.join("installed").exists() {
            GameState::Installed
        } else {
            GameState::NotDownloaded
        },
        game_dir,
    })
}
