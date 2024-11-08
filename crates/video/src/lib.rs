use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VideoError {
    #[error("Processing failed: {0}")]
    ProcessingFailed(String),
    #[error("Invalid format: {0}")]
    InvalidFormat(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoConfig {
    pub max_resolution: Resolution,
    pub default_format: VideoFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VideoFormat {
    MP4,
    HLS,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoMetadata {
    pub duration: f64,
    pub resolution: Resolution,
    pub format: VideoFormat,
    pub size: u64,
}

#[async_trait]
pub trait VideoProcessor: Send + Sync + 'static {
    async fn process_video(
        &self,
        input: Vec<u8>,
        config: VideoConfig,
    ) -> Result<(Vec<u8>, VideoMetadata), VideoError>;
    
    async fn extract_metadata(&self, input: &[u8]) -> Result<VideoMetadata, VideoError>;
}

// Simple video processor implementation
pub struct SimpleVideoProcessor {
    config: VideoConfig,
}

impl SimpleVideoProcessor {
    pub fn new() -> Self {
        Self {
            config: VideoConfig {
                max_resolution: Resolution {
                    width: 1920,
                    height: 1080,
                },
                default_format: VideoFormat::MP4,
            },
        }
    }
}

#[async_trait]
impl VideoProcessor for SimpleVideoProcessor {
    async fn process_video(
        &self,
        input: Vec<u8>,
        _config: VideoConfig,
    ) -> Result<(Vec<u8>, VideoMetadata), VideoError> {
        // Simplified implementation - just return input with mock metadata
        let metadata = VideoMetadata {
            duration: 60.0,
            resolution: Resolution {
                width: 1920,
                height: 1080,
            },
            format: VideoFormat::MP4,
            size: input.len() as u64,
        };
        
        Ok((input, metadata))
    }
    
    async fn extract_metadata(&self, input: &[u8]) -> Result<VideoMetadata, VideoError> {
        Ok(VideoMetadata {
            duration: 60.0,
            resolution: Resolution {
                width: 1920,
                height: 1080,
            },
            format: VideoFormat::MP4,
            size: input.len() as u64,
        })
    }
}
