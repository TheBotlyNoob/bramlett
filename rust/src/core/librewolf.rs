use std::io::Cursor;

use crate::api::{
    error::{Error, Result},
    games::Progress,
};
use futures::StreamExt;
use reqwest::Client;

use super::{dirs, download::download_with_progress};

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

    download_with_progress(&dl_url, progress, |_| {}).await
}

pub async fn extract_librewolf(bytes: Vec<u8>, progress: &Progress) -> Result<()> {
    let mut zip = zip::ZipArchive::new(Cursor::new(bytes))?;

    let path = dirs::data_local_dir().join("librewolf");

    let progress = progress.clone();
    tokio::task::spawn_blocking(move || {
        use std::fs;

        progress.set_denominator(zip.len() as u64);

        for i in 0..zip.len() {
            let mut file = zip.by_index(i)?;
            let filepath = file
                .enclosed_name()
                .ok_or(zip::result::ZipError::InvalidArchive("Invalid file path"))?;

            let outpath = path.join(filepath);

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
            // Get and Set permissions
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Some(mode) = file.unix_mode() {
                    fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
                }
            }

            progress.set_numerator(i as u64);
        }

        Ok::<_, Error>(())
    })
    .await
    .unwrap()?;

    Ok(())
}
