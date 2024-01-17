#[derive(thiserror::Error, Debug)]
#[flutter_rust_bridge::frb(opaque)]
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
