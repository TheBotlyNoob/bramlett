use anyhow::Result;
use flutter_rust_bridge::*;

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
    pub games: Vec<Games>,
}

pub async fn fetch_games() -> Result<Games> {
    Ok(
        if let Ok(games) = tokio::fs::read_to_string("games.json").await {
            serde_json::from_str(&games)?
        } else {
            // TODO: make sure to change this once PR is merged
            reqwest::get("https://raw.githubusercontent.com/TheBotlyNoob/bramletts-games/chore/flutter/games.json").await?.json().await?
        },
    )
}

#[frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
