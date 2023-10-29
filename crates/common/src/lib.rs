use std::path::PathBuf;

/// The ID of a game.
#[derive(
    Debug,
    Copy,
    Clone,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Deserialize,
    serde::Serialize,
    specta::Type,
)]
#[serde(transparent)]
pub struct GameId(pub u32);

#[derive(
    Debug,
    Clone,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Deserialize,
    serde::Serialize,
    specta::Type,
)]
pub struct GameInfo {
    /// The name of the game.
    pub name: String,
    /// The game's ID.
    pub id: GameId,
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
