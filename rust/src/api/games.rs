use std::{
    fs::File,
    io::{Cursor, Write},
    path::Path,
};

use anyhow::Result;
use flutter_rust_bridge::*;
use for_generated::BaseThreadPool;
use tokio::sync::watch;

use crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER;

#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("zip error: {0}")]
    Zip(#[from] sevenz_rust::Error),
    #[error("incorrect zip password")]
    BadZipPassword,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Game {
    pub name: String,
    pub exe: String,
    pub icon: String,
    pub url: String,
    pub uuid: uuid::Uuid,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Games {
    pub games: Vec<Game>,
}

pub async fn fetch_games() -> Result<Games> {
    Ok(
        if let Ok(games) = tokio::fs::read_to_string("games.json")
            .await
            .map_err(anyhow::Error::from)
            .and_then(|g| {
                log::info!("{g}");
                serde_json::from_str(&g).map_err(|e| {
                    log::warn!("invalid syntax in games.json: {e:#?}");
                    e.into()
                })
            })
        {
            games
        } else {
            // TODO: make sure to change this once PR is merged
            reqwest::get("https://raw.githubusercontent.com/TheBotlyNoob/bramletts-games/chore/flutter/games.json").await?.json().await?
        },
    )
}

pub struct FlutterWatch(watch::Receiver<(u64, u64)>);
#[frb(sync)]
pub fn get_watcher(obj: &FlutterWatch) -> (u64, u64) {
    *obj.0.borrow()
}

pub fn extract_zip(bytes: Vec<u8>, game: Game) -> Result<FlutterWatch> {
    let (tx, rx) = watch::channel((0, 0));

    log::info!("unzip {}", game.name);

    FLUTTER_RUST_BRIDGE_HANDLER
        .thread_pool()
        .execute(transfer!(|| {
            extract_zip_with_password(&bytes, &std::env::home_dir().unwrap().join("a"), "game", tx)
                .unwrap();
        }));

    Ok(FlutterWatch(rx))
}

/// Extracts a 7zip file to a directory.
///
/// # Errors
/// Returns an error if the 7zip file is invalid or the directory can't be written to.
///
/// # Panics
/// Panics if a the 7zip file doesn't have a single root directory.
#[allow(
    clippy::needless_pass_by_value,
    clippy::cognitive_complexity,
    clippy::non_octal_unix_permissions
)]
pub fn extract_zip_with_password(
    bytes: &[u8],
    dest: &Path,
    password: &str,
    progress: watch::Sender<(u64, u64)>,
) -> Result<()> {
    let mut sz =
        sevenz_rust::SevenZReader::new(Cursor::new(bytes), bytes.len() as u64, password.into())?;
    let total_files = sz.archive().files.len();
    let mut files = 0;
    macro_rules! push_progress {
        () => {
            if progress.send((files, total_files as u64)).is_err() {
                log::warn!("progress receiver dropped");
            };
        };
    }
    sz.for_each_entries(|entry, reader| {
        if entry.is_directory() {
            files += 1;
            return Ok(true); // we create the directory before creating files; removing this will cause an error with `File::create`
        }

        let path = Path::new(entry.name()); // TODO: handle invalid paths; we don't really need to worry about this but it's a good habit
        let mut components = path.components();
        components.next();
        let path = components.as_path();
        let path = dest.join(path);

        let mut buf = [0u8; 1024];
        std::fs::create_dir_all(path.parent().unwrap())?;
        let mut file = File::create(path)?;
        let res = loop {
            let read_size = reader.read(&mut buf)?;
            if read_size == 0 {
                break Ok(true);
            }
            file.write_all(&buf[..read_size])?;
        };

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = file.metadata()?.permissions();
            perms.set_mode(0o755);
            file.set_permissions(perms)?;
        }

        files += 1;

        push_progress!();

        res
    })?;

    push_progress!();

    log::warn!("DONE!! {files} / {total_files}");
    Ok(())
}

#[frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();

    let _ = env_logger::builder()
        .target(env_logger::Target::Stdout)
        .try_init();
}
