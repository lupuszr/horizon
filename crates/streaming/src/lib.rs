use async_trait::async_trait;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::mpsc;
use tracing::error;

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamConfig {
    pub chunk_size: usize,
    pub buffer_size: usize,
    pub target_chunk_duration: u64, // in milliseconds, for video/audio
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self {
            chunk_size: 1024 * 64,       // 64KB chunks
            buffer_size: 10,             // Buffer 10 chunks
            target_chunk_duration: 2000, // 2 seconds for video chunks
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentManifest {
    pub content_id: String,
    pub total_size: u64,
    pub content_type: String,
    pub chunks: Vec<ChunkInfo>,
    pub metadata: ContentMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkInfo {
    pub index: u64,
    pub cid: String,
    pub size: u32,
    pub offset: u64,
    pub timestamp: Option<u64>, // For video/audio chunks
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentMetadata {
    pub title: Option<String>,
    pub duration: Option<f64>,
    pub format: Option<String>,
    pub custom: std::collections::HashMap<String, String>,
}

pub struct StreamHandle {
    pub rx: mpsc::Receiver<StreamChunk>,
    pub manifest: ContentManifest,
}

#[derive(Debug)]
pub struct StreamChunk {
    pub data: Bytes,
    pub index: u64,
    pub timestamp: Option<u64>,
}

#[async_trait]
pub trait Streaming: Send + Sync + 'static {
    /// Get content manifest
    async fn get_manifest(&self, content_id: &str) -> Result<ContentManifest, StreamingError>;

    /// Stream content by manifest
    async fn stream(&self, manifest: &ContentManifest) -> Result<StreamHandle, StreamingError>;

    /// Stream specific chunk by CID
    async fn stream_chunk(&self, cid: &str) -> Result<Bytes, StreamingError>;
}

pub struct CidStreaming {
    config: StreamConfig,
    storage: Arc<dyn hybrid_streaming_storage::Storage>,
}

impl CidStreaming {
    pub fn new(config: StreamConfig, storage: Arc<dyn hybrid_streaming_storage::Storage>) -> Self {
        Self { config, storage }
    }

    async fn load_chunk(&self, chunk: &ChunkInfo) -> Result<Bytes, StreamingError> {
        self.storage
            .retrieve(&chunk.cid)
            .await
            .map(Bytes::from)
            .map_err(|e| StreamingError::StorageError(e.to_string()))
    }
}

#[async_trait]
impl Streaming for CidStreaming {
    async fn get_manifest(&self, content_id: &str) -> Result<ContentManifest, StreamingError> {
        // Load manifest from storage
        let manifest_data = self
            .storage
            .retrieve(content_id)
            .await
            .map_err(|e| StreamingError::StorageError(e.to_string()))?;

        serde_json::from_slice(&manifest_data)
            .map_err(|e| StreamingError::InvalidManifest(e.to_string()))
    }

    async fn stream(&self, manifest: &ContentManifest) -> Result<StreamHandle, StreamingError> {
        let (tx, rx) = mpsc::channel(self.config.buffer_size);
        let manifest_clone = manifest.clone();
        let storage = self.storage.clone();

        // Spawn task to stream chunks
        tokio::spawn(async move {
            for chunk_info in &manifest_clone.chunks {
                let chunk_data = match storage.retrieve(&chunk_info.cid).await {
                    Ok(data) => data,
                    Err(e) => {
                        error!("Failed to load chunk {}: {}", chunk_info.cid, e);
                        break;
                    }
                };

                let chunk = StreamChunk {
                    data: Bytes::from(chunk_data),
                    index: chunk_info.index,
                    timestamp: chunk_info.timestamp,
                };

                if tx.send(chunk).await.is_err() {
                    break;
                }
            }
        });

        Ok(StreamHandle {
            rx,
            manifest: manifest.clone(),
        })
    }

    async fn stream_chunk(&self, cid: &str) -> Result<Bytes, StreamingError> {
        self.storage
            .retrieve(cid)
            .await
            .map(Bytes::from)
            .map_err(|e| StreamingError::StorageError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use hybrid_streaming_storage::{ContentId, StorageError};

    use super::*;
    use std::{collections::HashMap, path::Path};

    // Mock storage implementation for testing
    #[derive(Clone)]
    struct MockStorage {
        data: Arc<std::sync::RwLock<HashMap<String, Vec<u8>>>>,
    }

    impl MockStorage {
        fn new() -> Self {
            Self {
                data: Arc::new(std::sync::RwLock::new(HashMap::new())),
            }
        }

        fn insert(&self, cid: &str, data: Vec<u8>) {
            self.data.write().unwrap().insert(cid.to_string(), data);
        }
    }

    #[async_trait]
    impl hybrid_streaming_storage::Storage for MockStorage {
        async fn upload_file(&self, _path: &Path) -> Result<ContentId, StorageError> {
            todo!()
        }

        /// Checks if content exists
        async fn exists(&self, _id: &ContentId) -> Result<bool, StorageError> {
            Ok(true)
        }

        /// Deletes content if possible (may not be supported by all storage backends)
        async fn delete(&self, _id: &ContentId) -> Result<(), StorageError> {
            todo!()
        }
        async fn retrieve(&self, cid: &str) -> Result<Vec<u8>, StorageError> {
            self.data.read().unwrap().get(cid).cloned().ok_or(
                hybrid_streaming_storage::StorageError::NotFound(cid.to_string()),
            )
        }

        // Implement other required methods...
        // This is just for testing, so we'll implement only what we need
    }

    fn create_test_manifest(chunks: Vec<ChunkInfo>) -> ContentManifest {
        ContentManifest {
            content_id: "test_content".to_string(),
            total_size: chunks.iter().map(|c| c.size as u64).sum(),
            content_type: "video/mp4".to_string(),
            chunks,
            metadata: ContentMetadata {
                title: Some("Test Video".to_string()),
                duration: Some(60.0),
                format: Some("mp4".to_string()),
                custom: HashMap::new(),
            },
        }
    }

    #[tokio::test]
    async fn test_stream_chunks() {
        let mock_storage = Arc::new(MockStorage::new());
        let config = StreamConfig::default();
        let streaming = CidStreaming::new(config, mock_storage.clone());

        // Create test chunks
        let chunks = vec![
            ChunkInfo {
                index: 0,
                cid: "chunk0".to_string(),
                size: 100,
                offset: 0,
                timestamp: Some(0),
            },
            ChunkInfo {
                index: 1,
                cid: "chunk1".to_string(),
                size: 100,
                offset: 100,
                timestamp: Some(2000),
            },
        ];

        // Insert test data
        mock_storage.insert("chunk0", vec![1; 100]);
        mock_storage.insert("chunk1", vec![2; 100]);

        let manifest = create_test_manifest(chunks);
        mock_storage.insert("test_content", serde_json::to_vec(&manifest).unwrap());

        // Test manifest retrieval
        let loaded_manifest = streaming.get_manifest("test_content").await.unwrap();
        assert_eq!(loaded_manifest.chunks.len(), 2);

        // Test streaming
        let mut handle = streaming.stream(&manifest).await.unwrap();

        // Verify first chunk
        let chunk1 = handle.rx.recv().await.unwrap();
        assert_eq!(chunk1.index, 0);
        assert_eq!(chunk1.data.len(), 100);
        assert!(chunk1.data.iter().all(|&b| b == 1));

        // Verify second chunk
        let chunk2 = handle.rx.recv().await.unwrap();
        assert_eq!(chunk2.index, 1);
        assert_eq!(chunk2.data.len(), 100);
        assert!(chunk2.data.iter().all(|&b| b == 2));

        // Verify stream end
        assert!(handle.rx.recv().await.is_none());
    }

    #[tokio::test]
    async fn test_stream_chunk_not_found() {
        let mock_storage = Arc::new(MockStorage::new());
        let config = StreamConfig::default();
        let streaming = CidStreaming::new(config, mock_storage);

        let result = streaming.stream_chunk("nonexistent").await;
        assert!(matches!(result, Err(StreamingError::StorageError(_))));
    }

    #[tokio::test]
    async fn test_invalid_manifest() {
        let mock_storage = Arc::new(MockStorage::new());
        let config = StreamConfig::default();
        let streaming = CidStreaming::new(config, mock_storage.clone());

        // Insert invalid manifest data
        mock_storage.insert("invalid_manifest", vec![1, 2, 3]); // Invalid JSON

        let result = streaming.get_manifest("invalid_manifest").await;
        assert!(matches!(result, Err(StreamingError::InvalidManifest(_))));
    }
}
