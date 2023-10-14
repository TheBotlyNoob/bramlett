use std::path::PathBuf;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct GameInfo {
    pub name: String,
    pub gdrive_id: String,
    pub exe: PathBuf,
    /// RHAI script with `post_install`, `pre_run` and `post_run` functions.
    ///
    /// `post_install` should be used in most cases to make a symlink from the game
    /// directory's save folder to the `save_dir`.
    ///
    /// File system is available.
    ///
    /// # Constants
    ///
    /// `game_dir`: `PathBuf` to the game directory.
    ///
    /// `save_dir`: `PathBuf` to the save directory.
    pub hooks: String,
}
