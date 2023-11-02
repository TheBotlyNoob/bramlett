use crate::{ClientError, Result};
use crate::{Ctx, Game};
use bytes::{Bytes, BytesMut};
use futures::StreamExt;
use tl::ParserOptions;
use tokio::sync::watch;

pub async fn download_game(
    game: Game,
    ctx: Ctx,
    progress: watch::Sender<(u32, u32)>,
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

    tracing::info!("real google drive download URL: {}", real_url);

    let res = ctx.client.get(&real_url).send().await?;
    let content_length = res.content_length().unwrap().try_into().unwrap();
    if progress.send((0, content_length)).is_err() {
        tracing::warn!("progress receiver dropped");
    };

    let mut bytes = BytesMut::new();
    let mut stream = res.bytes_stream();
    let mut recvd = 0;
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        recvd += chunk.len() as u32;
        if progress.send((recvd, content_length)).is_err() {
            tracing::warn!("progress receiver dropped");
        };
        bytes.extend_from_slice(&chunk);
    }

    Ok(bytes.freeze())
}
