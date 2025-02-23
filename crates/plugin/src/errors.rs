use crate::types::PluginType;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PluginLoaderError {
    #[error("Failed to read the plugin directory: {0}")]
    ReadDirError(#[from] std::io::Error),

    #[error("Failed to read manifest for plugin {path:?}: {source}")]
    ReadManifestError {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Invalid plugin manifest format for {path:?}: {source}")]
    InvalidManifestFormat {
        path: PathBuf,
        source: serde_json::Error,
    },

    #[error("Plugin '{0}' not found")]
    PluginNotFound(String),

    #[error("Plugin '{name}' is not of the expected type: {expected:?}")]
    PluginTypeMismatch { name: String, expected: PluginType },

    #[error("Failed to execute JavaScript plugin: {0}")]
    JavaScriptExecutionError(#[from] deno_core::error::AnyError),

    #[error("Failed to load or execute WASM plugin: {0}")]
    WasmExecutionError(String),
}
