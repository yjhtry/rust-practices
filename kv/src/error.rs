use thiserror::Error;
use tokio_rustls::rustls;

#[derive(Error, Debug, PartialEq)]
pub enum KvError {
    #[error("Not found for table: {0}, key: {1}")]
    NotFound(String, String),

    #[error("Not found: {0}")]
    NotSubscription(String),

    #[error("Cannot parse command: `{0}`")]
    InvalidCommand(String),

    #[error("Cannot convert value {0:?} to {1}")]
    ConvertError(String, &'static str),

    #[error("Cannot process command {0} with table: {1}, key: {2}. Error: {3}")]
    StorageError(&'static str, String, String, String),

    // StorageError(&'静态 str, 字符串, 字符串, 字符串),
    #[error("Failed to encode protobuf message")]
    EncodeError(#[from] prost::EncodeError),

    #[error("Failed to decode protobuf message")]
    DecodeError(#[from] prost::DecodeError),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Sled error: {0}")]
    SledError(#[from] sled::Error),

    #[error("Frame error")]
    FrameError,
    #[error("Io error")]
    IoError,

    #[error("TLS error: {0}")]
    TLSError(#[from] rustls::TLSError),

    #[error("TLS error: {0} {1}")]
    CertifcateParseError(&'static str, &'static str),

    #[error("Parse config error")]
    ConfigError(#[from] toml::de::Error),
}

impl From<std::io::Error> for KvError {
    fn from(_: std::io::Error) -> Self {
        KvError::IoError
    }
}
