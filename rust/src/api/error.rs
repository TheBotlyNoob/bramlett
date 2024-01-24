use allo_isolate::ffi::DartCObject;
use flutter_rust_bridge::IntoDart;
use std::ffi::CString;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("7zip error: {0}")]
    Zip7(#[from] sevenz_rust::Error),
    #[error("zip error: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("invalid file checksum")]
    InvalidChecksum,
    #[error("invalid download link (this was mb)")]
    InvalidDownload,
}
impl IntoDart for Error {
    fn into_dart(self) -> DartCObject {
        let s = CString::new(self.to_string()).unwrap_or_default();
        s.into_dart()
    }
}
pub type Result<T, E = Error> = std::result::Result<T, E>;
