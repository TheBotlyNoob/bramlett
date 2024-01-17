use super::error::ClientError;

#[derive(Clone, Debug)]
pub struct Game {
    pub name: String,
    pub icon: String,
    pub url: String,
    pub uuid: uuid::Uuid,
}

#[derive(Clone, Debug)]
pub struct Games {
    pub games: Vec<Games>,
}
#[flutter_rust_bridge::frb]
pub async fn fetch_games() -> Result<(), ClientError> {
    Ok(())
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
