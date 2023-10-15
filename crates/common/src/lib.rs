use std::path::PathBuf;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct GameInfo {
    /// The name of the game.
    pub name: String,
    /// A public Google Drive ID, linking to a zip file with a password of "game".
    pub gdrive_id: String,
    /// The path to the game's executable, relative to the game directory.
    pub exe: PathBuf,
    /// Extra flags to pass to the game's executable.
    pub args: Vec<String>,
    /// RHAI script with `post_install`, `pre_run` and `post_run` functions.
    ///
    /// `pre_run` should be used to sync the `save_dir`'s save data with the `game_dir`.
    /// `post_run` should be used to sync the `game_dir`'s save data with the `save_dir`.
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
