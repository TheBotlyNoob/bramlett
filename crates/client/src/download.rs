use std::fs::File;
use std::io::{Cursor, Write};
use std::path::Path;

use crate::{ClientError, Result};
use crate::{Ctx, Game};
use futures::StreamExt;
use tl::ParserOptions;
use tokio::sync::watch;

#[allow(clippy::module_name_repetitions)]
/// Downloads a game from Google Drive.
/// Returns the zip file as a byte array (the password is "game").
///
/// # Errors
/// Returns an error if the game is not found, the download fails,
/// or Google Drive's virus scanning feature breaks things.
pub async fn download_game(
    game: Game,
    ctx: Ctx,
    progress: watch::Sender<(u64, u64)>,
) -> Result<Vec<u8>> {
    let gdrive_url = format!(
        "https://drive.google.com/uc?export=download&id={}",
        game.info.gdrive_id
    );

    // TODO: multithreaded download
    let mut response = ctx.client.get(&gdrive_url).send().await?;

    if response
        .headers()
        .get("Content-Type")
        .is_some_and(|h| h.as_ref().starts_with(b"text/html"))
    {
        let real_url = {
            let text = response.text().await?;
            let dom = tl::parse(&text, ParserOptions::default())?;
            let parser = dom.parser();
            dom.get_element_by_id("download-form")
                .ok_or(ClientError::BadDrive)?
                .get(parser)
                .ok_or(ClientError::BadDrive)?
                .as_tag()
                .ok_or(ClientError::BadDrive)?
                .attributes()
                .get("action")
                .ok_or(ClientError::BadDrive)?
                .ok_or(ClientError::BadDrive)?
                .as_utf8_str()
                .replace("&amp;", "&")
        };

        tracing::info!("real google drive download URL: {real_url}");

        response = ctx.client.get(&real_url).send().await?;
    }

    let content_length = response.content_length().ok_or(ClientError::BadDrive)?;
    if progress.send((0, content_length)).is_err() {
        tracing::warn!("progress receiver dropped");
    };

    let mut bytes = Vec::new();
    let mut stream = response.bytes_stream();
    let mut recvd = 0;
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        recvd += chunk.len() as u64;

        tracing::trace!("received {} bytes", chunk.len());

        if progress.send((recvd, content_length)).is_err() {
            tracing::warn!("progress receiver dropped");
        };
        bytes.extend_from_slice(&chunk);
    }

    Ok(bytes)
}

/// Extracts a 7zip file to a directory.
///
/// # Errors
/// Returns an error if the 7zip file is invalid or the directory can't be written to.
///
/// # Panics
/// Panics if a the 7zip file doesn't have a single root directory.
#[allow(clippy::needless_pass_by_value, clippy::cognitive_complexity)]
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
    sz.for_each_entries(|entry, reader| {
        if entry.is_directory() {
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
            if progress.send((files, total_files as u64)).is_err() {
                tracing::warn!("progress receiver dropped");
            };
        };

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = file.metadata()?.permissions();
            perms.set_mode(0o777);
            file.set_permissions(perms)?;
        }

        files += 1;
        res
    })?;
    Ok(())
}
