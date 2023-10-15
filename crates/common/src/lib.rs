use std::path::PathBuf;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct GameInfo {
    pub name: String,
    pub gdrive_id: String,
    pub exe: PathBuf,
    /// RHAI script with `post_install`, `pre_run` and `post_run` functions.
    ///
    /// `post_run` should be used to sync the `game_dir`'s save data with the `save_dir`.
    /// You are allowed to assume you are running on Windows.
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
