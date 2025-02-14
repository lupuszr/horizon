use cid::{Cid, Version};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};
use thiserror::Error;

pub mod errors;
pub mod event;
pub mod iroh;

// TODO: deprecate this

#[derive(Error, Debug)]
pub enum StreamingError {
    #[error("Content not found: {0}")]
    NotFound(String),
    #[error("Streaming failed: {0}")]
    StreamingFailed(String),
    #[error("Invalid manifest: {0}")]
    InvalidManifest(String),
    #[error("Storage error: {0}")]
    StorageError(String),
    #[error("Invalid CID: {0}")]
    InvalidCid(String),
    #[error("Encoding error: {0}")]
    EncodingError(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ContentId(pub Cid);

impl ContentId {
    pub fn new(cid: Cid) -> Self {
        Self(cid)
    }

    pub fn into_inner(self) -> Cid {
        self.0
    }

    pub fn as_cid(&self) -> &Cid {
        &self.0
    }

    /// Get the base32 encoded string
    pub fn to_base32(&self) -> String {
        use multibase::Base;
        multibase::encode(Base::Base32Lower, self.0.to_bytes())
    }

    /// Get the base58 encoded string (default IPFS format)
    pub fn to_base58(&self) -> String {
        self.0.to_string()
    }

    /// Get the raw bytes of the CID
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }

    /// Check if this is a CIDv0
    pub fn is_v0(&self) -> bool {
        self.0.version() == Version::V0
    }

    /// Check if this is a CIDv1
    pub fn is_v1(&self) -> bool {
        self.0.version() == Version::V1
    }
}

impl Display for ContentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for ContentId {
    type Err = StreamingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Cid::from_str(s)
            .map(ContentId)
            .map_err(|e| StreamingError::InvalidCid(e.to_string()))
    }
}

impl Serialize for ContentId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ContentId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Cid::from_str(&s)
            .map(ContentId)
            .map_err(serde::de::Error::custom)
    }
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Authentication failed")]
    AuthError,
    #[error("Storage error: {0}")]
    StorageError(String),
    #[error("Streaming error: {0}")]
    StreamingError(String),
    #[error("Video processing error: {0}")]
    VideoError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoMetadata {
    pub title: String,
    pub duration: f64,
    pub resolution: String,
    pub bitrate: u32,
}

pub mod config {
    #[derive(Debug, Default, Clone)]
    pub struct AppConfig {
        pub iroh_endpoint: String,
        pub jwt_secret: String,
        pub max_upload_size: usize,
    }

    impl AppConfig {
        pub fn new() -> Self {
            // In a real app, load from environment or config file
            Self {
                iroh_endpoint: "http://localhost:8080".to_string(),
                jwt_secret: "your-secret-key".to_string(),
                max_upload_size: 100 * 1024 * 1024, // 100MB
            }
        }
    }
}

pub mod prelude {
    pub use super::config::AppConfig;
    pub use super::ApiError;
    pub use super::VideoMetadata;
}

#[cfg(test)]
mod tests {
    // TODO: add tests
}
