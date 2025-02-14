// use deno_core::{op2, Extension, JsRuntime, OpDecl, RuntimeOptions}; // For JavaScript
// use serde::{Deserialize, Serialize};
// use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};
// use thiserror::Error;
// use tokio::sync::{Mutex, RwLock};
// use wasmtime::{Caller, Engine, Linker, Module, Store}; // For WASM

use deno_core::{op2, Extension, JsRuntime, OpDecl, RuntimeOptions};
use horizon_core::event::HorizonChannel;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};
use thiserror::Error;
use tokio::sync::{Mutex, RwLock};
use wasmtime::{Caller, Engine, Func, Instance, Linker, Module, Store};

#[derive(Debug, Deserialize)]
pub struct Plugin {
    pub name: String,
    pub version: String,
    pub description: String,
    pub plugin_type: PluginType,
    pub entry_path: PathBuf,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum PluginType {
    Wasm,
}

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

enum LoadedPlugin {
    Wasm(Module),
}

pub struct PluginLoader {
    plugins: Arc<RwLock<HashMap<String, (Plugin, LoadedPlugin)>>>,
    wasm_engine: Engine,
}

impl PluginLoader {
    /// Create a new PluginLoader instance
    pub fn new() -> Self {
        Self {
            plugins: Arc::new(RwLock::new(HashMap::new())),
            wasm_engine: Engine::default(),
        }
    }

    /// Load all plugins from a specified directory
    pub async fn load_plugins(&self, directory: &str) -> Result<(), PluginLoaderError> {
        let dir = PathBuf::from(directory);
        let mut plugins = self.plugins.write().await;

        plugins.clear();

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension() == Some("wasm".as_ref()) {
                let manifest_path = path.with_extension("json");
                if manifest_path.exists() {
                    let manifest_content =
                        fs::read_to_string(&manifest_path).map_err(|source| {
                            PluginLoaderError::ReadManifestError {
                                path: manifest_path.clone(),
                                source,
                            }
                        })?;
                    let plugin: Plugin =
                        serde_json::from_str(&manifest_content).map_err(|source| {
                            PluginLoaderError::InvalidManifestFormat {
                                path: manifest_path.clone(),
                                source,
                            }
                        })?;

                    let loaded_plugin = match plugin.plugin_type {
                        PluginType::Wasm => {
                            let module =
                                Module::from_file(&self.wasm_engine, &path).map_err(|err| {
                                    PluginLoaderError::WasmExecutionError(err.to_string())
                                })?;
                            LoadedPlugin::Wasm(module)
                        }
                    };

                    plugins.insert(plugin.name.clone(), (plugin, loaded_plugin));
                }
            }
        }

        Ok(())
    }

    /// Reload all plugins from the directory (safe for concurrency)
    pub async fn reload_plugins(&self, directory: &str) -> Result<(), PluginLoaderError> {
        self.load_plugins(directory).await
    }

    /// Dispatch an event to the specified plugin (safe for concurrent reads)
    pub async fn dispatch_event(
        &self,
        plugin_name: &str,
        event: &str,
    ) -> Result<(), PluginLoaderError> {
        println!("dispatch from plugin: {}", plugin_name);
        let plugins = self.plugins.read().await;

        let (plugin, loaded_plugin) = plugins
            .get(plugin_name)
            .ok_or_else(|| PluginLoaderError::PluginNotFound(plugin_name.to_string()))?;

        match loaded_plugin {
            LoadedPlugin::Wasm(module) => {
                let engine = self.wasm_engine.clone();
                let mut store = Store::new(&engine, ()); // Empty store data
                let instance = Instance::new(&mut store, module, &[])
                    .map_err(|err| PluginLoaderError::WasmExecutionError(err.to_string()))?;

                let handle_event_func = instance.get_func(&mut store, "handle_event");

                if let Some(func) = handle_event_func {
                    // here we define the types of the wasm handle_event
                    // TODO: right now the types are i32 -> i32 fix that
                    let typed_func = func.typed::<i32, i32>(&store).map_err(|e| {
                        PluginLoaderError::WasmExecutionError(format!(
                            "Error getting typed function: {}",
                            e
                        ))
                    })?;

                    // invoke the wasm method
                    let result = typed_func
                        .call(&mut store, event.len() as i32) // Pass event length
                        .map_err(|e| {
                            PluginLoaderError::WasmExecutionError(format!(
                                "Error calling function: {}",
                                e
                            ))
                        })?;

                    println!("WASM function returned: {}", result);
                } else {
                    return Err(PluginLoaderError::WasmExecutionError(
                        "Missing 'handle_event' function".to_string(),
                    ));
                }

                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod plugin_tests {
    use tokio::runtime::Runtime;
    use wabt::wat2wasm;
    use wasmtime::Config;

    use super::*;
    use std::io::Write; // For creating test files

    #[tokio::test]
    async fn test_load_plugins() -> Result<(), PluginLoaderError> {
        let loader = PluginLoader::new();
        let temp_dir = tempfile::tempdir().unwrap(); // Create a temporary directory

        // Create a dummy WASM file (replace with actual WASM if needed)
        let wasm_path = temp_dir.path().join("test_plugin.wasm");
        let mut wasm_file = fs::File::create(&wasm_path).unwrap();
        wasm_file.write_all(b"\0asm\x01\0\0\0").unwrap(); // Minimal valid WASM header

        // Create a dummy manifest file
        let manifest_path = temp_dir.path().join("test_plugin.json");
        let manifest_content = r#"
            {
                "name": "TestPlugin",
                "version": "1.0.0",
                "description": "A test plugin",
                "plugin_type": "Wasm",
                "entry_path": "test_plugin.wasm"
            }
        "#;
        fs::write(&manifest_path, manifest_content).unwrap();

        loader
            .load_plugins(temp_dir.path().to_str().unwrap())
            .await?;

        let plugins = loader.plugins.read().await;
        assert_eq!(plugins.len(), 1);
        assert!(plugins.contains_key("TestPlugin"));

        temp_dir.close().unwrap(); // Clean up the temporary directory
        Ok(())
    }

    #[tokio::test]
    async fn test_dispatch_event() -> Result<(), PluginLoaderError> {
        let loader = PluginLoader::new();
        let temp_dir = tempfile::tempdir().unwrap();

        // 1. Create a REAL WASM module with handle_event (using wat2wasm)
        let wasm_wat = r#"
            (module
                (func $handle_event (param i32) (result i32)
                    local.get 0
                    i32.const 2
                    i32.mul
                )
                (export "handle_event" (func $handle_event))
            )
        "#;

        let wasm_bytes = wat2wasm(wasm_wat.as_bytes()).unwrap();

        let engine = Engine::new(&Config::default()).unwrap();
        let module = Module::new(&engine, &wasm_bytes).unwrap();

        let wasm_path = temp_dir.path().join("test_plugin.wasm");
        let mut wasm_file = fs::File::create(&wasm_path).unwrap();
        wasm_file.write_all(&wasm_bytes).unwrap();

        // 2. Create the manifest file
        let manifest_path = temp_dir.path().join("test_plugin.json");
        let manifest_content = r#"
            {
                "name": "TestPlugin",
                "version": "1.0.0",
                "description": "A test plugin",
                "plugin_type": "Wasm",
                "entry_path": "test_plugin.wasm"
            }
        "#;
        fs::write(&manifest_path, manifest_content).unwrap();

        loader
            .load_plugins(temp_dir.path().to_str().unwrap())
            .await?;

        // Optional: Compare the loaded module with the original module
        let plugins = loader.plugins.read().await;
        let (_, loaded_plugin) = plugins.get("TestPlugin").unwrap();

        match loaded_plugin {
            LoadedPlugin::Wasm(loaded_module) => {
                let loaded_bytes = loaded_module.serialize().unwrap();
                assert_eq!(wasm_bytes, loaded_bytes);
            }
        }

        let result = loader.dispatch_event("TestPlugin", "test_event").await;
        assert!(result.is_ok());
        // assert_eq!(result, Ok(())); // Check that dispatch was successful

        temp_dir.close().unwrap();
        Ok(())
    }

    #[tokio::test]
    async fn test_plugin_not_found() {
        let loader = PluginLoader::new();
        let result = loader
            .dispatch_event("NonExistentPlugin", "test_event")
            .await;
        assert!(matches!(result, Err(PluginLoaderError::PluginNotFound(_))));
    }
}
