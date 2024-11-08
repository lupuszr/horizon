use async_trait::async_trait;
use bytes::Bytes;
use cid::Cid;
use hybrid_streaming_common::{ContentId, StreamingError};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamConfig {
    pub chunk_size: usize,
    pub buffer_size: usize,
    pub target_chunk_duration: u64,
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self {
            chunk_size: 1024 * 64,
            buffer_size: 10,
            target_chunk_duration: 2000,
        }
    }
}

/// Wrapper for Cid to implement Serialize/Deserialize

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentManifest {
    pub content_id: ContentId,
    pub total_size: u64,
    pub content_type: String,
    pub chunks: Vec<ChunkInfo>,
    pub metadata: ContentMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkInfo {
    pub index: u64,
    pub cid: ContentId,
    pub size: u32,
    pub offset: u64,
    pub timestamp: Option<u64>,
    pub codec: Option<String>,      // Optional codec information
    pub encryption: Option<String>, // Optional encryption information
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentMetadata {
    pub title: Option<String>,
    pub duration: Option<f64>,
    pub format: Option<String>,
    pub codecs: Vec<String>,
    pub creation_date: Option<chrono::DateTime<chrono::Utc>>,
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
    pub cid: ContentId,
}

#[async_trait]
pub trait Streaming: Send + Sync + 'static {
    /// Get content manifest by CID
    async fn get_manifest(&self, cid: &ContentId) -> Result<ContentManifest, StreamingError>;

    /// Stream content based on manifest
    async fn stream(&self, manifest: &ContentManifest) -> Result<StreamHandle, StreamingError>;

    /// Stream specific chunk by CID
    async fn stream_chunk(&self, cid: &ContentId) -> Result<Bytes, StreamingError>;
}

#[derive(Clone)]
pub struct CidStreaming {
    config: StreamConfig,
    storage: Arc<dyn hybrid_streaming_storage::Storage>,
}

impl CidStreaming {
    pub fn new(config: StreamConfig, storage: Arc<dyn hybrid_streaming_storage::Storage>) -> Self {
        Self { config, storage }
    }

    pub async fn load_chunk(&self, chunk: &ChunkInfo) -> Result<Bytes, StreamingError> {
        self.storage
            .retrieve(&chunk.cid.to_string())
            .await
            .map(Bytes::from)
            .map_err(|e| StreamingError::StorageError(e.to_string()))
    }

    /// Create a new CID for content
    pub fn create_content_cid(data: &[u8]) -> Result<ContentId, StreamingError> {
        use multihash::{Code, MultihashDigest};

        // Create a SHA2-256 hash of the content
        let hash = Code::Sha2_256.digest(data);

        // Create a CIDv1 with raw codec
        let cid = Cid::new_v1(0x55, hash); // 0x55 is the codec for raw data

        Ok(ContentId(cid))
    }
}

#[async_trait]
impl Streaming for CidStreaming {
    async fn get_manifest(&self, cid: &ContentId) -> Result<ContentManifest, StreamingError> {
        let manifest_data = self
            .storage
            .retrieve(&cid.to_string())
            .await
            .map_err(|e| StreamingError::StorageError(e.to_string()))?;

        serde_json::from_slice(&manifest_data)
            .map_err(|e| StreamingError::InvalidManifest(e.to_string()))
    }

    async fn stream(&self, manifest: &ContentManifest) -> Result<StreamHandle, StreamingError> {
        let (tx, rx) = mpsc::channel(self.config.buffer_size);
        let manifest_clone = manifest.clone();
        let storage = self.storage.clone();

        tokio::spawn(async move {
            for chunk_info in &manifest_clone.chunks {
                let chunk_data = match storage.retrieve(&chunk_info.cid.to_string()).await {
                    Ok(data) => data,
                    Err(e) => {
                        error!("Failed to load chunk {}: {}", chunk_info.cid.0, e);
                        break;
                    }
                };

                let chunk = StreamChunk {
                    data: Bytes::from(chunk_data),
                    index: chunk_info.index,
                    timestamp: chunk_info.timestamp,
                    cid: chunk_info.cid.clone(),
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

    async fn stream_chunk(&self, cid: &ContentId) -> Result<Bytes, StreamingError> {
        self.storage
            .retrieve(&cid.to_string())
            .await
            .map(Bytes::from)
            .map_err(|e| StreamingError::StorageError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use hybrid_streaming_common::ContentId;
    use hybrid_streaming_storage::StorageError;
    use multihash::MultihashDigest;
    use tokio::time::timeout;

    use super::*;
    use std::{collections::HashMap, path::Path, time::Duration};

    // Mock storage implementation for testing
    #[derive(Clone)]
    struct MockStorage {
        data: Arc<tokio::sync::RwLock<HashMap<String, Vec<u8>>>>,
    }

    impl MockStorage {
        fn new() -> Self {
            Self {
                data: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            }
        }

        async fn insert(&self, cid: &ContentId, data: Vec<u8>) {
            self.data.write().await.insert(cid.to_string(), data);
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
            self.data.read().await.get(cid).cloned().ok_or(
                hybrid_streaming_storage::StorageError::NotFound(cid.to_string()),
            )
        }

        // Implement other required methods...
        // This is just for testing, so we'll implement only what we need
    }

    #[test]
    fn test_content_id_display() {
        let data = b"test data";
        let cid = CidStreaming::create_content_cid(data).unwrap();

        // Test Display implementation
        let display_str = format!("{}", cid);
        let parsed_back = ContentId::from_str(&display_str).unwrap();
        assert_eq!(cid, parsed_back);
    }

    #[test]
    fn test_content_id_encoding() {
        let data = b"test data";
        let cid = CidStreaming::create_content_cid(data).unwrap();

        // Test different encodings
        let base32 = cid.to_base32();
        let base58 = cid.to_base58();
        let bytes = cid.to_bytes();

        // Verify we can parse back from both encodings
        assert_eq!(ContentId::from_str(&base58).unwrap(), cid);

        // Verify bytes roundtrip
        assert_eq!(Cid::try_from(bytes.as_slice()).unwrap(), cid.into_inner());
    }

    #[test]
    fn test_content_id_json() {
        let data = b"test data";
        let cid = CidStreaming::create_content_cid(data).unwrap();

        // Test JSON serialization
        let json = serde_json::to_string(&cid).unwrap();
        let parsed: ContentId = serde_json::from_str(&json).unwrap();
        assert_eq!(cid, parsed);

        // Test in a struct
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct TestStruct {
            id: ContentId,
            name: String,
        }

        let test_struct = TestStruct {
            id: cid.clone(),
            name: "test".to_string(),
        };

        let json = serde_json::to_string(&test_struct).unwrap();
        let parsed: TestStruct = serde_json::from_str(&json).unwrap();
        assert_eq!(test_struct, parsed);
    }

    // #[test]
    // fn test_content_id_conversions() {
    //     let data = b"test data";
    //     let cid = CidStreaming::create_content_cid(data).unwrap();

    //     // Test From<Cid>
    //     let inner_cid = cid.clone().into_inner();
    //     let converted: ContentId = inner_cid.into();
    //     assert_eq!(cid, converted);

    //     // Test AsRef<Cid>
    //     let cid_ref: &Cid = cid.as_ref();
    //     assert_eq!(cid_ref, &cid.into_inner());
    // }

    #[test]
    fn test_content_id_version_checks() {
        let data = b"test data";
        let cid = CidStreaming::create_content_cid(data).unwrap();

        // Should be V1 by default
        assert!(cid.is_v1());
        assert!(!cid.is_v0());

        // Create a V0 CID (only possible with SHA2-256)
        let hash = multihash::Code::Sha2_256.digest(data);
        let v0_cid = ContentId::new(Cid::new_v0(hash).unwrap());

        assert!(v0_cid.is_v0());
        assert!(!v0_cid.is_v1());
    }

    fn create_patterned_data(pattern: u8, size: usize) -> Vec<u8> {
        let mut data = Vec::with_capacity(size);
        for i in 0..size {
            data.push((i as u8).wrapping_add(pattern));
        }
        data
    }

    async fn setup_test_streaming() -> (CidStreaming, Arc<MockStorage>) {
        let mock_storage = Arc::new(MockStorage::new());
        let config = StreamConfig {
            chunk_size: 1024,            // 1KB chunks for testing
            buffer_size: 5,              // Small buffer to test backpressure
            target_chunk_duration: 1000, // 1s chunks
        };
        let streaming = CidStreaming::new(config, mock_storage.clone());
        (streaming, mock_storage)
    }

    async fn create_test_chunks(count: usize, size: usize) -> Vec<(ChunkInfo, Vec<u8>)> {
        let mut chunks = Vec::with_capacity(count);
        let mut offset = 0;

        for i in 0..count {
            let data = create_patterned_data(i as u8, size);
            let cid = CidStreaming::create_content_cid(&data).unwrap();

            let chunk_info = ChunkInfo {
                index: i as u64,
                cid,
                size: size as u32,
                offset,
                timestamp: Some(i as u64 * 1000), // 1s intervals
                codec: Some("h264".to_string()),
                encryption: None,
            };

            offset += size as u64;
            chunks.push((chunk_info, data));
        }

        chunks
    }

    #[tokio::test]
    async fn test_stream_chunks() {
        // Setup
        let (streaming, storage) = setup_test_streaming().await;

        // Create test data
        let chunk_size = 1024;
        let chunk_count = 5;
        let chunks_with_data = create_test_chunks(chunk_count, chunk_size).await;
        let chunks_info: Vec<ChunkInfo> = chunks_with_data
            .iter()
            .map(|(info, _)| info.clone())
            .collect();

        // Store chunks in mock storage
        for (chunk_info, data) in &chunks_with_data {
            storage.insert(&chunk_info.cid, data.clone()).await;
        }

        // Create and store manifest
        let manifest = ContentManifest {
            content_id: CidStreaming::create_content_cid(b"test_manifest").unwrap(),
            total_size: (chunk_size * chunk_count) as u64,
            content_type: "video/mp4".to_string(),
            chunks: chunks_info.clone(),
            metadata: ContentMetadata {
                title: Some("Test Video".to_string()),
                duration: Some(5.0),
                format: Some("mp4".to_string()),
                codecs: vec!["h264".to_string(), "aac".to_string()],
                creation_date: Some(chrono::Utc::now()),
                custom: HashMap::new(),
            },
        };

        storage
            .insert(&manifest.content_id, serde_json::to_vec(&manifest).unwrap())
            .await;

        // Test cases

        // 1. Test manifest retrieval
        let loaded_manifest = streaming.get_manifest(&manifest.content_id).await.unwrap();
        assert_eq!(loaded_manifest.chunks.len(), chunk_count);
        assert_eq!(
            loaded_manifest.total_size,
            (chunk_size * chunk_count) as u64
        );

        // 2. Test streaming with timeout
        let mut handle = streaming.stream(&manifest).await.unwrap();

        // 3. Verify each chunk with timeout
        for i in 0..chunk_count {
            let chunk_timeout = timeout(Duration::from_secs(1), handle.rx.recv())
                .await
                .unwrap()
                .unwrap();

            // Verify chunk properties
            assert_eq!(chunk_timeout.index, i as u64);
            assert_eq!(chunk_timeout.data.len(), chunk_size);
            assert_eq!(chunk_timeout.timestamp, Some(i as u64 * 1000));

            // Verify chunk data pattern
            let expected_data = create_patterned_data(i as u8, chunk_size);
            assert_eq!(&chunk_timeout.data[..], &expected_data[..]);

            // Verify CID matches
            assert_eq!(chunk_timeout.cid, chunks_info[i].cid);
        }

        // 4. Verify stream ends properly
        let no_more_chunks = timeout(Duration::from_millis(100), handle.rx.recv())
            .await
            .unwrap();
        assert!(no_more_chunks.is_none());

        // 5. Test individual chunk streaming
        for (chunk_info, expected_data) in chunks_with_data {
            println!("chunk info cid:: {}", &chunk_info.cid);
            let chunk_data = streaming.stream_chunk(&chunk_info.cid).await.unwrap();
            println!("chunk data:: {:?}", &chunk_data[..]);
            assert_eq!(&chunk_data[..], &expected_data[..]);
        }

        // 6. Test non-existent chunk
        let non_existent_cid = CidStreaming::create_content_cid(b"non_existent").unwrap();
        let error = streaming.stream_chunk(&non_existent_cid).await.unwrap_err();
        assert!(matches!(error, StreamingError::StorageError(_)));

        // 7. Test manifest not found
        let non_existent_manifest_cid = CidStreaming::create_content_cid(b"no_manifest").unwrap();
        let error = streaming
            .get_manifest(&non_existent_manifest_cid)
            .await
            .unwrap_err();
        assert!(matches!(error, StreamingError::StorageError(_)));
    }
}
