use deno_core::{op2, Extension, JsRuntime, OpDecl, RuntimeOptions}; // For JavaScript
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};
use thiserror::Error;
use tokio::sync::{Mutex, RwLock};
use wasmtime::{Caller, Engine, Linker, Module, Store}; // For WASM

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
    JavaScript,
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
    JavaScript(Arc<Mutex<JsRuntime>>),
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

    // pub fn initialize_js_runtime(api: Arc<dyn HorizonPushAPI>) -> JsRuntime {
    //     const DECL: OpDecl = op_sum();
    //     let ext = Extension {
    //         name: "my_ext",
    //         ops: std::borrow::Cow::Borrowed(&[DECL]),
    //         ..Default::default()
    //     };

    //     //       ::builder()
    //     //     .ops(vec![
    //     //         (
    //     //             "op_log_event",
    //     //             op_sync(move |_, args, _| {
    //     //                 let message: String = args[0].as_str().unwrap().to_string();
    //     //                 api.log_event(&message);
    //     //                 Ok(())
    //     //             }),
    //     //         ),
    //     //         (
    //     //             "op_trigger_reload",
    //     //             op_sync(move |_, _, _| {
    //     //                 api.trigger_reload();
    //     //                 Ok(())
    //     //             }),
    //     //         ),
    //     //         (
    //     //             "op_get_plugin_info",
    //     //             op_sync(move |_, _, _| Ok(api.get_plugin_info())),
    //     //         ),
    //     //     ])
    //     //     .build();

    //     JsRuntime::new(RuntimeOptions {
    //         extensions: vec![ext],
    //         ..Default::default()
    //     })

    //     // todo!()
    // }
    /// Load all plugins from a specified directory
    pub async fn load_plugins(&self, directory: &str) -> Result<(), PluginLoaderError> {
        let dir = PathBuf::from(directory);
        let mut plugins = self.plugins.write().await;

        // Clear the existing plugins
        plugins.clear();

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file()
                && (path.extension() == Some("js".as_ref())
                    || path.extension() == Some("wasm".as_ref()))
            {
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
                        PluginType::JavaScript => {
                            let mut runtime = JsRuntime::new(RuntimeOptions::default());
                            let js_code = fs::read_to_string(&path).map_err(|err| {
                                PluginLoaderError::ReadManifestError {
                                    path: path.clone(),
                                    source: err,
                                }
                            })?;
                            runtime
                                .execute_script("plugin.js", js_code)
                                .map_err(|err| {
                                    PluginLoaderError::JavaScriptExecutionError(err.into())
                                })?;
                            LoadedPlugin::JavaScript(Arc::new(Mutex::new(runtime)))
                        }
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
        // Lock for writing to ensure no reads or writes happen while reloading
        self.load_plugins(directory).await
    }

    /// Dispatch an event to the specified plugin (safe for concurrent reads)
    pub async fn dispatch_event(
        &self,
        plugin_name: &str,
        event: &str,
    ) -> Result<(), PluginLoaderError> {
        println!("dispatch from plugin: {}", plugin_name);
        let plugins = self.plugins.read().await; // Concurrent reads

        let (plugin, loaded_plugin) = plugins
            .get(plugin_name)
            .ok_or_else(|| PluginLoaderError::PluginNotFound(plugin_name.to_string()))?;

        match loaded_plugin {
            LoadedPlugin::JavaScript(runtime) => {
                let event_script = format!("handleEvent('{}');", event);
                runtime
                    .lock()
                    .await
                    .execute_script("event.js", event_script)
                    .map_err(|err| PluginLoaderError::JavaScriptExecutionError(err.into()))?;
                Ok(())
            }
            LoadedPlugin::Wasm(module) => {
                let engine = self.wasm_engine.clone();
                let mut linker = Linker::new(&engine);
                linker.func_wrap(
                    "host",
                    "host_func",
                    |caller: Caller<'_, u32>, param: i32| {
                        println!("Got {} from WebAssembly", param);
                        println!("my host state is: {}", caller.data());
                    },
                )?;

                // let mut store = Store::new(&self.wasm_engine, event.to_string());
                // let instance = wasmtime::Instance::new(&mut store, module, &[])
                //     .map_err(|err| PluginLoaderError::WasmExecutionError(err.to_string()))?;
                // let func = instance
                //     .get_func(&mut store, "handle_event")
                //     .ok_or_else(|| {
                //         PluginLoaderError::WasmExecutionError(
                //             "Missing 'handle_event' function".to_string(),
                //         )
                //     })?;
                // let handle_event = func.typed::<(), (), _>(&store)?;
                // handle_event
                //     .call(&mut store, ())
                //     .map_err(|err| PluginLoaderError::WasmExecutionError(err.to_string()))?;
                // Ok(())
                todo!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;
    use tokio::runtime::Runtime;

    fn create_test_plugin(directory: &str, name: &str, plugin_type: PluginType) -> PathBuf {
        let manifest = json!({
            "name": name,
            "version": "1.0.0",
            "description": "Test Plugin",
            "plugin_type": plugin_type,
            "entry_path": format!("{}/{}.js", directory, name),
        });

        let plugin_dir = PathBuf::from(directory);
        let plugin_path = plugin_dir.join(format!("{}.js", name));
        let manifest_path = plugin_dir.join(format!("{}.json", name));

        // Create plugin directory
        std::fs::create_dir_all(&plugin_dir).unwrap();

        // Write the manifest file
        let mut manifest_file = File::create(&manifest_path).unwrap();
        manifest_file
            .write_all(manifest.to_string().as_bytes())
            .unwrap();

        // Write a basic plugin script (empty for now)
        let mut plugin_file = File::create(&plugin_path).unwrap();
        plugin_file
            .write_all(
                b"console.log('Plugin Loaded');\n\
            function handleEvent(event) {\n\
              console.log('Event is: ', event)\n\
            }",
            )
            .unwrap();

        plugin_path
    }

    #[tokio::test]
    async fn test_load_plugins() {
        let dir = PathBuf::from("/tmp/testload_folder");
        if dir.exists() {
            fs::remove_dir_all(dir.clone()).unwrap();
        }
        if !dir.exists() {
            fs::create_dir(&dir).unwrap();
        }
        // let dir = tempdir().unwrap();
        let plugin_loader = PluginLoader::new();

        // Create test plugins
        create_test_plugin(dir.to_str().unwrap(), "plugin1", PluginType::JavaScript);
        // create_test_plugin(dir.to_str().unwrap(), "plugin2", PluginType::Wasm);

        // Load plugins
        let result = plugin_loader.load_plugins(dir.to_str().unwrap()).await;
        assert!(result.is_ok());

        // Check that the plugins are loaded correctly
        let plugins = plugin_loader.plugins.read().await;
        assert_eq!(plugins.len(), 1);
        assert!(plugins.contains_key("plugin1"));
        // assert!(plugins.contains_key("plugin2"));
    }

    #[tokio::test]
    async fn test_dispatch_event_js_plugin() {
        let dir = tempdir().unwrap();
        let plugin_loader = PluginLoader::new();

        // Create a JavaScript plugin
        create_test_plugin(
            dir.path().to_str().unwrap(),
            "js_plugin",
            PluginType::JavaScript,
        );

        // Load plugins
        let result = plugin_loader
            .load_plugins(dir.path().to_str().unwrap())
            .await;
        assert!(result.is_ok());

        // Dispatch an event
        let result = plugin_loader
            .dispatch_event("js_plugin", "test_event")
            .await;
        println!("res:: {:?}", result);
        assert!(result.is_ok());
    }

    //     #[tokio::test]
    //     async fn test_dispatch_event_wasm_plugin() {
    //         let dir = tempdir().unwrap();
    //         let plugin_loader = PluginLoader::new();

    //         // Create a WASM plugin (using an empty .wasm file for testing)
    //         let plugin_path = create_test_plugin(
    //             dir.path().to_str().unwrap(),
    //             "wasm_plugin",
    //             PluginType::Wasm,
    //         );
    //         let wasm_file = File::create(plugin_path).unwrap();
    //         wasm_file.sync_all().unwrap(); // Empty WASM file

    //         // Load plugins
    //         let result = plugin_loader
    //             .load_plugins(dir.path().to_str().unwrap())
    //             .await;
    //         assert!(result.is_ok());

    //         // Dispatch an event
    //         let result = plugin_loader
    //             .dispatch_event("wasm_plugin", "test_event")
    //             .await;
    //         assert!(result.is_err()); // Should fail since the wasm plugin is empty
    //     }

    //     #[tokio::test]
    //     async fn test_plugin_not_found() {
    //         let plugin_loader = PluginLoader::new();

    //         // Try dispatching event for a non-existent plugin
    //         let result = plugin_loader
    //             .dispatch_event("non_existent_plugin", "test_event")
    //             .await;
    //         assert!(result.is_err());
    //         if let Err(PluginLoaderError::PluginNotFound(name)) = result {
    //             assert_eq!(name, "non_existent_plugin");
    //         }
    //     }

    //     #[tokio::test]
    //     async fn test_invalid_plugin_manifest() {
    //         let dir = tempdir().unwrap();
    //         let plugin_loader = PluginLoader::new();

    //         // Create an invalid plugin manifest (missing required fields)
    //         let invalid_manifest = json!({
    //             "name": "invalid_plugin",
    //             "version": "1.0.0",
    //         });
    //         let manifest_path = dir.path().join("invalid_plugin.json");
    //         let mut file = File::create(&manifest_path).unwrap();
    //         file.write_all(invalid_manifest.to_string().as_bytes())
    //             .unwrap();

    //         // Try loading plugins
    //         let result = plugin_loader
    //             .load_plugins(dir.path().to_str().unwrap())
    //             .await;
    //         assert!(result.is_err());
    //         if let Err(PluginLoaderError::InvalidManifestFormat { path, .. }) = result {
    //             assert_eq!(path, manifest_path);
    //         }
    //     }

    //     #[tokio::test]
    //     async fn test_plugin_type_mismatch() {
    //         let dir = tempdir().unwrap();
    //         let plugin_loader = PluginLoader::new();

    //         // Create a JavaScript plugin
    //         create_test_plugin(
    //             dir.path().to_str().unwrap(),
    //             "plugin_js",
    //             PluginType::JavaScript,
    //         );

    //         // Load plugins
    //         let result = plugin_loader
    //             .load_plugins(dir.path().to_str().unwrap())
    //             .await;
    //         assert!(result.is_ok());

    //         // Try dispatching a Wasm event on a JavaScript plugin
    //         let result = plugin_loader
    //             .dispatch_event("plugin_js", "test_event")
    //             .await;
    //         assert!(result.is_err());
    //         if let Err(PluginLoaderError::PluginTypeMismatch { name, expected }) = result {
    //             assert_eq!(name, "plugin_js");
    //             assert_eq!(expected, PluginType::Wasm);
    //         }
    //     }
}
