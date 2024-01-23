use crate::api::{
    error::{Error, Result},
    games::Progress,
};
use futures::StreamExt;
use reqwest::Client;

#[derive(serde::Deserialize, Clone, Debug)]
struct ReleaseAsset {
    name: String,
    direct_asset_url: String,
}
#[derive(serde::Deserialize, Clone, Debug)]
struct ReleaseAssetInner {
    links: Vec<ReleaseAsset>,
}
#[derive(serde::Deserialize, Clone, Debug)]
struct ReleaseAssets {
    assets: ReleaseAssetInner,
}

pub async fn download_librewolf(progress: &Progress) -> Result<Vec<u8>> {
    let client = Client::new();

    let assets = client
        .get("https://gitlab.com/api/v4/projects/44042130/releases/permalink/latest/")
        .send()
        .await?
        .json::<ReleaseAssets>()
        .await?;

    let dl_url = assets
        .assets
        .links
        .into_iter()
        .find(|asset| asset.name.ends_with("-windows-x86_64-portable.zip"))
        .ok_or(Error::InvalidDownload)?
        .direct_asset_url;

    let res = client.get(dl_url).send().await?;

    let content_len = res.content_length().ok_or(Error::InvalidDownload)?;

    progress.set_denominator(content_len);

    let mut bytes = Vec::with_capacity(content_len as usize);
    let mut byte_stream = res.bytes_stream();

    while let Some(new) = byte_stream.next().await {
        let chunk = new?;
        bytes.extend_from_slice(&chunk);
        progress.set_numerator(bytes.len() as u64);
    }

    Ok(bytes)
}

pub async fn extract_librewolf(bytes: &[u8], progress: &Progress) {}
