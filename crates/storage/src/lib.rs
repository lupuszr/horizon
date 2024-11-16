use std::path::Path;

use horizon_core::ContentId;
use thiserror::Error;
use tracing::error;

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

/// Core storage traits that define the interface for our storage system
#[async_trait::async_trait]
pub trait Storage: Send + Sync {
    /// Uploads a file from a path and returns a content identifier
    async fn upload_file(&self, path: &Path) -> Result<ContentId, StorageError>;

    async fn retrieve(&self, cid: &str) -> Result<Vec<u8>, StorageError>;
    /// Checks if content exists
    async fn exists(&self, id: &ContentId) -> Result<bool, StorageError>;

    /// Deletes content if possible (may not be supported by all storage backends)
    async fn delete(&self, id: &ContentId) -> Result<(), StorageError>;
}

// Re-export storage implementation
