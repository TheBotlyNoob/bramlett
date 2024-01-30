use std::path::PathBuf;

use crate::core::game::Game;

pub fn config_dir() -> Option<PathBuf> {
    Some(dirs::config_dir()?.join("bramlett"))
}
pub fn data_local_dir() -> Option<PathBuf> {
    Some(dirs::data_local_dir()?.join("bramlett"))
}
pub fn document_dir() -> Option<PathBuf> {
    Some(dirs::document_dir()?.join("bramlett"))
}

pub fn game_dir(game: &Game) -> Option<PathBuf> {
    Some(data_local_dir()?.join("games").join(&game.uuid))
}
