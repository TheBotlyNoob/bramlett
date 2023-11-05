use std::fs;
use std::io::Cursor;
use std::path::Path;

use crate::{ClientError, Result};
use crate::{Ctx, Game};
use bytes::{Bytes, BytesMut};
use futures::StreamExt;
use tl::ParserOptions;
use tokio::sync::watch;
use zip::ZipArchive;

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
) -> Result<Bytes> {
    let gdrive_url = format!(
        "https://drive.google.com/uc?export=download&id={}",
        game.info.gdrive_id
    );

    // TODO: multithreaded download
    let response = ctx.client.get(&gdrive_url).send().await?.text().await?;
    let real_url = {
        let dom = tl::parse(&response, ParserOptions::default())?;
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

    let res = ctx.client.get(&real_url).send().await?;
    let content_length = res.content_length().ok_or(ClientError::BadDrive)?;
    if progress.send((0, content_length)).is_err() {
        tracing::warn!("progress receiver dropped");
    };

    let mut bytes = BytesMut::new();
    let mut stream = res.bytes_stream();
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

    Ok(bytes.freeze())
}

/// Extracts a zip file to a directory.
///
/// # Errors
/// Returns an error if the zip file is invalid or the directory can't be written to.
///
/// # Panics
/// Panics if the zip file contains invalid paths (see [`ZipFile::enclosed_name`]).
#[allow(clippy::needless_pass_by_value, clippy::cognitive_complexity)]
pub fn extract_zip_with_password(
    bytes: &[u8],
    dir: &Path,
    password: &[u8],
    progress: watch::Sender<(u64, u64)>,
) -> Result<()> {
    std::fs::create_dir_all(dir)?;
    let mut archive = ZipArchive::new(Cursor::new(bytes))?;
    let archive_size = archive.len() as u64;
    if progress.send((0, archive_size)).is_err() {
        tracing::warn!("progress receiver dropped");
    };
    for i in 0..archive.len() {
        let mut file = archive
            .by_index_decrypt(i, password)?
            .map_err(|_| ClientError::BadZipPassword)?;
        let mut filepath_components = file.enclosed_name().unwrap().components();
        filepath_components.next();
        let outpath = dir.join(filepath_components.as_path());

        tracing::debug!(?outpath, "extracting");

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut outfile = fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }

        // make sure executable is executable on unix for wine users
        #[cfg(unix)]
        if let Some(ext) = outpath.extension() {
            if ext == "exe" {
                use std::os::unix::fs::PermissionsExt;

                let mut perms = fs::metadata(&outpath)?.permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&outpath, perms)?;
            }
        }

        if progress.send((i as u64 + 1, archive_size)).is_err() {
            tracing::warn!("progress receiver dropped");
        };
    }
    Ok(())
}
