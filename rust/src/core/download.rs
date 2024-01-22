use super::game::Game;
use crate::api::{
    error::{Error, Result},
    games::Progress,
};
use reqwest::Client;

pub async fn download_game(game: Game, progress: &Progress) -> Result<Vec<u8>> {
    const NUM_CHUNKS: usize = 4;

    let url = format!(
        "https://qiwi.lol/{}.7z",
        game.url.rsplit_once('/').ok_or(Error::InvalidDownload)?.1
    );
    let client = Client::new();
    let content_len: usize = client
        .head(&url)
        .send()
        .await?
        .headers()
        .get("content-length")
        .ok_or(Error::InvalidDownload)?
        .to_str()
        .unwrap()
        .parse()
        .unwrap();

    let (num_chunks, chunk_size) = if content_len < 1_000_000 * 10 * NUM_CHUNKS {
        (1, content_len)
    } else {
        (NUM_CHUNKS, content_len.div_ceil(NUM_CHUNKS))
    };

    progress.set_denominator(num_chunks as u64);

    let res = futures::future::join_all((0..num_chunks).map(|i| {
        let url = url.clone();
        let client = client.clone();
        let progress = progress.clone();
        async move {
            let res = client
                .get(url)
                .header(
                    "Range",
                    format!(
                        "bytes={}-{}",
                        dbg!(i * chunk_size),
                        dbg!((i + 1) * chunk_size)
                    ),
                )
                .send()
                .await?
                .bytes()
                .await?;
            progress.increment_numerator();
            Ok(res)
        }
    }))
    .await
    .into_iter()
    .collect::<Result<Vec<_>>>()?;

    progress.set_numerator(0);
    progress.set_denominator(0); // we can't really give progress for the below stuff; just have a loading thing.

    let mut collected = Vec::with_capacity(num_chunks * chunk_size);

    for bytes in res {
        collected.extend_from_slice(&bytes);
    }

    // TODO: checksums
    // if dbg!(sha256::digest(&*collected)) != dbg!(game.sha256) {
    //     return Err(Error::InvalidChecksum);
    // };

    Ok(collected.to_vec())
}
