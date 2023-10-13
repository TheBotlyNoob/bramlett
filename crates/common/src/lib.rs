#[derive(serde::Deserialize, serde::Serialize)]
pub struct GameInfo {
    pub name: String,
    pub gdrive_id: String,
}
