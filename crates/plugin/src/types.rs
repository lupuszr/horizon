// horizon-plugin/src/types/mod.rs
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::any::Any;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub id: uuid::Uuid,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub plugin_type: PluginType,
    pub capabilities: Vec<PluginCapability>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PluginType {
    Integration,
    Media,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PluginCapability {
    Storage,
    Streaming,
    Processing,
    Authentication,
    Monitoring,
    Custom(String),
}

#[async_trait]
pub trait Plugin: Send + Sync {
    fn metadata(&self) -> &PluginMetadata;

    async fn initialize(&self, config: PluginConfig) -> anyhow::Result<()>;
    async fn shutdown(&self) -> anyhow::Result<()>;

    async fn health_check(&self) -> anyhow::Result<HealthStatus>;

    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub settings: std::collections::HashMap<String, serde_json::Value>,
    pub work_dir: std::path::PathBuf,
    pub max_memory: Option<u64>,
    pub allowed_capabilities: Vec<PluginCapability>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub healthy: bool,
    pub message: Option<String>,
    pub last_check: chrono::DateTime<chrono::Utc>,
    pub metrics: std::collections::HashMap<String, f64>,
}

// Specific plugin type traits
pub mod custom;
pub mod integration;
pub mod media;

// Re-exports
pub use custom::CustomPlugin;
pub use integration::IntegrationPlugin;
pub use media::MediaPlugin;
// Plugin registry interfaces
#[async_trait]
pub trait PluginRegistry: Send + Sync {
    async fn register_plugin(&self, metadata: PluginMetadata) -> anyhow::Result<()>;
    async fn unregister_plugin(&self, id: uuid::Uuid) -> anyhow::Result<()>;
    async fn get_plugin(&self, id: uuid::Uuid) -> anyhow::Result<Option<PluginMetadata>>;
    async fn list_plugins(&self) -> anyhow::Result<Vec<PluginMetadata>>;
    async fn list_plugins_by_type(
        &self,
        plugin_type: PluginType,
    ) -> anyhow::Result<Vec<PluginMetadata>>;
    async fn list_plugins_by_capability(
        &self,
        capability: PluginCapability,
    ) -> anyhow::Result<Vec<PluginMetadata>>;
}

// Plugin manager interfaces
#[async_trait]
pub trait PluginManager: Send + Sync {
    async fn load_plugin(&self, path: &std::path::Path) -> anyhow::Result<uuid::Uuid>;
    async fn unload_plugin(&self, id: uuid::Uuid) -> anyhow::Result<()>;
    async fn get_plugin_instance(&self, id: uuid::Uuid) -> anyhow::Result<Box<dyn Plugin>>;
    async fn reload_plugin(&self, id: uuid::Uuid) -> anyhow::Result<()>;
    async fn get_plugin_status(&self, id: uuid::Uuid) -> anyhow::Result<PluginStatus>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginStatus {
    pub id: uuid::Uuid,
    pub state: PluginState,
    pub loaded_at: chrono::DateTime<chrono::Utc>,
    pub last_health_check: Option<HealthStatus>,
    pub error_count: u32,
    pub memory_usage: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PluginState {
    Initialized,
    Running,
    Stopped,
    Failed(String),
    Updating,
}

// Plugin SDK helper macros
#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty, $constructor:expr) => {
        #[no_mangle]
        pub extern "C" fn _plugin_create() -> *mut dyn $crate::Plugin {
            let plugin = $constructor();
            Box::into_raw(Box::new(plugin))
        }

        #[no_mangle]
        pub extern "C" fn _plugin_metadata() -> *mut $crate::PluginMetadata {
            let plugin = $constructor();
            Box::into_raw(Box::new(plugin.metadata().clone()))
        }
    };
}
