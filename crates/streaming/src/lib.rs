use async_trait::async_trait;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::mpsc;

#[derive(Error, Debug)]
pub enum StreamingError {
    #[error("Content not found")]
    NotFound,
    #[error("Streaming failed: {0}")]
    StreamingFailed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamConfig {
    pub chunk_size: usize,
    pub buffer_size: usize,
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self {
            chunk_size: 1024 * 64, // 64KB chunks
            buffer_size: 10,       // Buffer 10 chunks
        }
    }
}

#[async_trait]
pub trait Streaming: Send + Sync + 'static {
    async fn stream(&self, content_id: &str) -> Result<StreamHandle, StreamingError>;
}

pub struct StreamHandle {
    pub rx: mpsc::Receiver<Bytes>,
    pub content_type: String,
    pub size: u64,
}

// Simple streaming implementation
pub struct SimpleStreaming {
    config: StreamConfig,
    storage: Arc<dyn storage::Storage>,
}

impl SimpleStreaming {
    pub fn new(storage: Arc<dyn storage::Storage>) -> Self {
        Self {
            config: StreamConfig::default(),
            storage,
        }
    }
}

#[async_trait]
impl Streaming for SimpleStreaming {
    async fn stream(&self, content_id: &str) -> Result<StreamHandle, StreamingError> {
        let content = self.storage
            .retrieve(content_id)
            .await
            .map_err(|e| StreamingError::StreamingFailed(e.to_string()))?;

        let (tx, rx) = mpsc::channel(self.config.buffer_size);
        let chunk_size = self.config.chunk_size;
        let size = content.len() as u64;

        // Start streaming chunks
        tokio::spawn(async move {
            for chunk in content.chunks(chunk_size) {
                if tx.send(Bytes::copy_from_slice(chunk)).await.is_err() {
                    break;
                }
            }
        });

        Ok(StreamHandle {
            rx,
            content_type: "video/mp4".into(), // Simplified for example
            size,
        })
    }
}
