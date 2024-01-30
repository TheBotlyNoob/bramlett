use super::game::Game;
use crate::api::{
    error::{Error, Result},
    games::Progress,
};
use futures::StreamExt;
use sha2::{Digest, Sha256};

pub async fn download_game(game: Game, progress: &Progress) -> Result<Vec<u8>> {
    let url = format!(
        "https://qiwi.lol/{}.7z",
        game.url.rsplit_once('/').ok_or(Error::InvalidDownload)?.1
    );

    let mut hasher = Sha256::new();

    let downloaded = download_with_progress(&url, progress, |chunk| hasher.update(chunk)).await?;

    if let Ok(expected_sha) = hex::decode(&game.sha256) {
        if *hasher.finalize() != expected_sha {
            log::error!("invalid checksum for game: {:#?}", game.name);
            return Err(Error::InvalidChecksum);
        }
    } else {
        log::error!("games db contains invalid SHA256 checksum.");
    };

    Ok(downloaded.to_vec())
}

pub async fn download_with_progress(
    url: &str,
    progress: &Progress,
    mut cb: impl FnMut(&[u8]),
) -> Result<Vec<u8>> {
    let res = reqwest::get(url).await?;

    let content_len = res.content_length().ok_or(Error::InvalidDownload)?;

    progress.set_denominator(content_len);

    let mut bytes = Vec::with_capacity(content_len as usize);
    let mut byte_stream = res.bytes_stream();

    while let Some(new) = byte_stream.next().await {
        let chunk = new?;
        cb(&chunk);
        bytes.extend_from_slice(&chunk);
        progress.set_numerator(bytes.len() as u64);
    }

    log::info!("UPDATE: {} / {content_len}", bytes.len());

    Ok(bytes)
}
