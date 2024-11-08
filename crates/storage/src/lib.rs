use std::path::Path;

use hybrid_streaming_common::prelude::*;
use thiserror::Error;
use tracing::{debug, error, info};

mod iroh;
pub use iroh::IrohStorage;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Failed to initialize storage: {0}")]
    InitError(String),
    #[error("Upload failed: {0}")]
    UploadError(String),
    #[error("Download failed: {0}")]
    DownloadError(String),
    #[error("Content not found: {0}")]
    NotFound(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Represents the content identifier returned after uploading
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ContentId(pub String);

/// Core storage traits that define the interface for our storage system
#[async_trait::async_trait]
pub trait Storage: Send + Sync + 'static {
    /// Uploads a file from a path and returns a content identifier
    async fn upload_file(&self, path: &Path) -> Result<ContentId, StorageError>;
    
    /// Uploads bytes directly and returns a content identifier
    async fn upload_bytes(&self, bytes: Vec<u8>) -> Result<ContentId, StorageError>;
    
    /// Downloads content to a specific path
    async fn download_to_file(&self, id: &ContentId, path: &Path) -> Result<(), StorageError>;
    
    /// Downloads content as bytes
    async fn download_bytes(&self, id: &ContentId) -> Result<Vec<u8>, StorageError>;
    
    /// Checks if content exists
    async fn exists(&self, id: &ContentId) -> Result<bool, StorageError>;
    
    /// Deletes content if possible (may not be supported by all storage backends)
    async fn delete(&self, id: &ContentId) -> Result<(), StorageError>;
}

// Re-export storage implementation
pub use crate::iroh::IrohStorage as DefaultStorage;
