use serde::{Deserialize, Serialize};
use thiserror::Error;

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
