use std::path::PathBuf;

use crate::core::game::Game;

pub fn home_dir() -> PathBuf {
    dirs::home_dir().expect("home directory")
}
pub fn config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| home_dir().join(PathBuf::from(".config")))
        .join("bramlett")
}
pub fn data_local_dir() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| home_dir().join(PathBuf::from(".local/share")))
        .join("bramlett")
}
pub fn document_dir() -> PathBuf {
    dirs::document_dir()
        .unwrap_or_else(|| home_dir().join("Documents"))
        .join("bramlett")
}

pub fn game_dir(game: &Game) -> PathBuf {
    data_local_dir().join("games").join(&game.uuid)
}
