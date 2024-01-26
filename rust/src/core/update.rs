use crate::api::error::Result;
use reqwest::Client;
use rfd::MessageDialogResult;

#[derive(serde::Deserialize, Debug)]
pub struct TagName {
    tag_name: String,
}

pub async fn check_latest_version() -> Result<()> {
    let client = Client::builder()
        .user_agent(concat!("bramletts-games/", env!("CARGO_PKG_VERSION")))
        .build()?;

    let TagName { tag_name } = client
        .get("https://api.github.com/repos/TheBotlyNoob/bramletts-games/releases/latest")
        .send()
        .await?
        .json()
        .await?;

    let current_tag = concat!("v", env!("CARGO_PKG_VERSION"));

    if current_tag != tag_name {
        let res = rfd::AsyncMessageDialog::new().set_title("new version available").set_description("A new update is available to download. I would recommend you download it at:\n\thttps://jjay.dev/bramletts-games\n\nClick YES to exit the app. You have to download the update yourself.\nClick NO to keep running the app (some things might be broken or freeze).").set_buttons(rfd::MessageButtons::YesNo).show().await;

        if res == MessageDialogResult::Yes {
            std::process::exit(0);
        }
    }

    Ok(())
}
