use horizon_core::event::HorizonChannel;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};
use thiserror::Error;
use tokio::sync::RwLock;
use wasmtime::{
    component::{Component, Linker, Resource, Val},
    Config, Engine, Store,
};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView}; // Import spawn_blocking

wasmtime::component::bindgen!({
   world: "extension",
   with: {
           // Specify that our host resource is going to point to the `MyLogger`
           // which is defined just below this macro.
           "horizon:extension/network/document": HorizonDocument,
    },
    trappable_imports: true,
    async: true
});

pub struct HorizonDocument {}

#[derive(Debug, Deserialize)]
pub struct Plugin {
    pub name: String,
    pub version: String,
    pub description: String,
    pub plugin_type: PluginType,
    pub entry_path: PathBuf,
}

struct WasmState {
    ctx: WasiCtx,
    http_ctx: WasiHttpCtx,
    table: ResourceTable,
}

impl WasiView for WasmState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

impl WasiHttpView for WasmState {
    fn ctx(&mut self) -> &mut wasmtime_wasi_http::WasiHttpCtx {
        &mut self.http_ctx
    }

    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl horizon::extension::network::Host for WasmState {}

impl horizon::extension::network::HostDocument for WasmState {
    async fn new(&mut self) -> wasmtime::Result<Resource<HorizonDocument>> {
        let id = self.table.push(HorizonDocument {})?;
        Ok(id)
    }
    async fn read_key(
        &mut self,
        document: Resource<HorizonDocument>,
        key: String,
    ) -> wasmtime::Result<Vec<u8>> {
        debug_assert!(!document.owned());
        println!("DODO");
        return Ok(vec![1, 2]);
    }
    async fn add_key_value(
        &mut self,
        document: Resource<HorizonDocument>,
        key: String,
        value: Vec<u8>,
    ) -> wasmtime::Result<()> {
        todo!()
    }
    async fn drop(
        &mut self,
        rep: wasmtime::component::Resource<HorizonDocument>,
    ) -> wasmtime::Result<()> {
        Ok(())
    }
}

impl horizon::extension::logger::Host for WasmState {
    async fn log(&mut self, msg: String) -> wasmtime::Result<()> {
        println!("extension log: {:?}", msg);
        Ok(())
    }
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
    Wasm(Component),
}

pub struct PluginLoader {
    plugins: Arc<RwLock<HashMap<String, (Plugin, LoadedPlugin)>>>,
    wasm_engine: Engine,
}

fn type_annotate_wasi<T, F>(val: F) -> F
where
    F: Fn(&mut T) -> wasmtime_wasi::WasiImpl<&mut T>,
{
    val
}

impl PluginLoader {
    /// Create a new PluginLoader instance
    pub fn new() -> Self {
        let mut config = Config::new();
        config.async_support(true);
        config.wasm_multi_memory(true);
        config.wasm_component_model(true);
        config
            .debug_info(true)
            .wasm_backtrace(true)
            .coredump_on_trap(true)
            .profiler(wasmtime::ProfilingStrategy::None)
            .wasm_tail_call(true)
            .wasm_function_references(true)
            .wasm_gc(true);

        Self {
            plugins: Arc::new(RwLock::new(HashMap::new())),
            wasm_engine: Engine::new(&config).unwrap(),
        }
    }

    /// Load all plugins from a specified directory
    pub async fn load_plugins(&self, directory: &str) -> Result<(), PluginLoaderError> {
        let dir = PathBuf::from(directory);
        println!("dir: {:?}", dir);
        let mut config = Config::new();
        config.async_support(true);
        config.wasm_multi_memory(true);
        config.wasm_component_model(true);
        config
            .debug_info(true)
            .wasm_backtrace(true)
            .coredump_on_trap(true)
            .profiler(wasmtime::ProfilingStrategy::None)
            .wasm_tail_call(true)
            .wasm_function_references(true)
            .wasm_gc(true);
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
                            let component = Component::from_file(&self.wasm_engine, &path)
                                .map_err(|err| {
                                    PluginLoaderError::WasmExecutionError(err.to_string())
                                })?;
                            LoadedPlugin::Wasm(component)
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
        event: &HorizonChannel,
    ) -> Result<(), PluginLoaderError> {
        println!("dispatch from plugin: {}", plugin_name);
        let plugins = self.plugins.read().await;

        let (_plugin, loaded_plugin) = plugins
            .get(plugin_name)
            .ok_or_else(|| PluginLoaderError::PluginNotFound(plugin_name.to_string()))?;

        match loaded_plugin {
            LoadedPlugin::Wasm(component) => {
                let event_json = serde_json::to_string(event).unwrap();

                let mut store = Store::new(
                    &self.wasm_engine,
                    WasmState {
                        ctx: WasiCtxBuilder::new()
                            .inherit_stdio()
                            .inherit_stderr()
                            .build(),
                        table: ResourceTable::new(),
                        http_ctx: WasiHttpCtx::new(),
                    },
                ); // Empty store data

                let mut linker = Linker::<WasmState>::new(&self.wasm_engine);
                let closure = type_annotate_wasi::<WasmState, _>(|t| wasmtime_wasi::WasiImpl(t));
                wasmtime_wasi::bindings::cli::terminal_stdin::add_to_linker_get_host(
                    &mut linker,
                    closure,
                )?;
                wasmtime_wasi::bindings::cli::terminal_stdout::add_to_linker_get_host(
                    &mut linker,
                    closure,
                )?;
                wasmtime_wasi::bindings::cli::terminal_stderr::add_to_linker_get_host(
                    &mut linker,
                    closure,
                )?;
                wasmtime_wasi::bindings::cli::terminal_output::add_to_linker_get_host(
                    &mut linker,
                    closure,
                )?;
                wasmtime_wasi::bindings::cli::terminal_input::add_to_linker_get_host(
                    &mut linker,
                    closure,
                )?;
                wasmtime_wasi::bindings::filesystem::types::add_to_linker_get_host(
                    &mut linker,
                    closure,
                )?;
                wasmtime_wasi::bindings::filesystem::preopens::add_to_linker_get_host(
                    &mut linker,
                    closure,
                )?;

                // crate::bindings::clocks::monotonic_clock::add_to_linker_get_host(l, closure)?;
                // crate::bindings::sync::filesystem::types::add_to_linker_get_host(l, closure)?;
                // crate::bindings::filesystem::preopens::add_to_linker_get_host(l, closure)?;
                // crate::bindings::io::error::add_to_linker_get_host(l, closure)?;
                // crate::bindings::sync::io::poll::add_to_linker_get_host(l, closure)?;
                // crate::bindings::sync::io::streams::add_to_linker_get_host(l, closure)?;
                // crate::bindings::random::random::add_to_linker_get_host(l, closure)?;
                // crate::bindings::random::insecure::add_to_linker_get_host(l, closure)?;
                // crate::bindings::random::insecure_seed::add_to_linker_get_host(l, closure)?;
                // crate::bindings::cli::exit::add_to_linker_get_host(l, &options.into(), closure)?;
                // crate::bindings::cli::environment::add_to_linker_get_host(l, closure)?;
                // crate::bindings::cli::stdin::add_to_linker_get_host(l, closure)?;
                // crate::bindings::cli::stdout::add_to_linker_get_host(l, closure)?;
                // crate::bindings::cli::stderr::add_to_linker_get_host(l, closure)?;
                // crate::bindings::cli::terminal_input::add_to_linker_get_host(l, closure)?;
                // crate::bindings::cli::terminal_output::add_to_linker_get_host(l, closure)?;
                // crate::bindings::cli::terminal_stdin::add_to_linker_get_host(l, closure)?;
                // crate::bindings::cli::terminal_stdout::add_to_linker_get_host(l, closure)?;
                // crate::bindings::cli::terminal_stderr::add_to_linker_get_host(l, closure)?;
                // crate::bindings::sync::sockets::tcp::add_to_linker_get_host(l, closure)?;
                // crate::bindings::sockets::tcp_create_socket::add_to_linker_get_host(l, closure)?;
                // crate::bindings::sync::sockets::udp::add_to_linker_get_host(l, closure)?;
                // crate::bindings::sockets::udp_create_socket::add_to_linker_get_host(l, closure)?;
                // crate::bindings::sockets::instance_network::add_to_linker_get_host(l, closure)?;
                // crate::bindings::sockets::network::add_to_linker_get_host(l, &options.into(), closure)?;
                // crate::bindings::sockets::ip_name_lookup::add_to_linker_get_host(l, closure)?;
                // wasmtime_wasi_http::add_to_linker_sync(l)
                Extension::add_to_linker(&mut linker, |state: &mut WasmState| state)?;
                // wasmtime_wasi::add_to_linker_async(&mut linker).unwrap();
                wasmtime_wasi_http::add_to_linker_async(&mut linker).unwrap();

                // linker.define_unknown_imports_as_traps(component).unwrap();
                let instance = linker
                    .instantiate_async(&mut store, component)
                    .await
                    .map_err(|err| PluginLoaderError::WasmExecutionError(err.to_string()))?;

                let params = vec![Val::String(event_json.into())];
                let mut result = vec![Val::String("".into())];
                let handle_event_func = instance.get_func(&mut store, "handle");

                if let Some(func) = handle_event_func {
                    // here we define the types of the wasm handle_event
                    // (i32, i32, i32) <=> (event_type, , len)i
                    // spawn_blocking(move || {
                    match func.call_async(store, &params, &mut result).await {
                        Ok(_) => {
                            println!("Res is:: {:?}", result);

                            ()
                        }
                        Err(err) => {
                            println!("invoke error {:?}", err);
                        }
                    }

                    // .await
                    // .unwrap(); // Handle errors
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

    use super::*;
    use std::env; // For creating test files

    // // #[tokio::test]
    // async fn test_load_plugins() -> Result<(), PluginLoaderError> {
    //     let loader = PluginLoader::new();
    //     let temp_dir = tempfile::tempdir().unwrap(); // Create a temporary directory

    //     // Create a dummy WASM file (replace with actual WASM if needed)
    //     let wasm_path = temp_dir.path().join("test_plugin.wasm");
    //     let mut wasm_file = fs::File::create(&wasm_path).unwrap();
    //     wasm_file.write_all(b"\0asm\x01\0\0\0").unwrap(); // Minimal valid WASM header

    //     // Create a dummy manifest file
    //     let manifest_path = temp_dir.path().join("test_plugin.json");
    //     let manifest_content = r#"
    //         {
    //             "name": "TestPlugin",
    //             "version": "1.0.0",
    //             "description": "A test plugin",
    //             "plugin_type": "Wasm",
    //             "entry_path": "test_plugin.wasm"
    //         }
    //     "#;
    //     fs::write(&manifest_path, manifest_content).unwrap();

    //     loader
    //         .load_plugins(temp_dir.path().to_str().unwrap())
    //         .await?;

    //     let plugins = loader.plugins.read().await;
    //     assert_eq!(plugins.len(), 1);
    //     assert!(plugins.contains_key("TestPlugin"));

    //     temp_dir.close().unwrap(); // Clean up the temporary directory
    //     Ok(())
    // }

    #[tokio::test]
    async fn test_dispatch_event() -> Result<(), PluginLoaderError> {
        let mut path = env::current_dir()?;

        println!("The current directory is {}", path.display());
        // 1. Setup: Create a PluginLoader instance
        let plugin_loader = PluginLoader::new();

        // 2. Load Plugins (replace with your actual directory)
        path.push("test_plugins"); // Create a directory with test plugins
        plugin_loader.load_plugins(&path.to_str().unwrap()).await?;

        // 3. Create a test event
        let test_event = HorizonChannel::IrohTicket("ticket".into());

        // 4. Dispatch the event
        let plugin_name = "TestPlugin"; // Name of the plugin you want to test
        plugin_loader
            .dispatch_event(plugin_name, &test_event)
            .await?;

        // 5. Assertions (verify the expected behavior)
        // This is the most important part of the test. You need to define
        // what you expect to happen when the event is dispatched.

        // For example, you might check if the plugin's state has been updated
        // or if a specific function has been called.

        // Since the test event is an IrohIndexingEvent, you might expect the WASM plugin to print "Indexing started"

        Ok(())
    }

    // #[tokio::test]
    // async fn test_dispatch_event() -> Result<(), PluginLoaderError> {
    //     let loader = PluginLoader::new();
    //     let temp_dir = tempfile::tempdir().unwrap();

    //     // 1. Create a REAL WASM module with handle_event (using wat2wasm)
    //     let wasm_wat = r#"
    //         (module
    //             (func $handle_event (param i32) (result i32)
    //                 local.get 0
    //                 i32.const 2
    //                 i32.mul
    //             )
    //             (export "handle_event" (func $handle_event))
    //         )
    //     "#;

    //     let wasm_bytes = wat2wasm(wasm_wat.as_bytes()).unwrap();

    //     let engine = Engine::new(&Config::default()).unwrap();
    //     let module = Module::new(&engine, &wasm_bytes).unwrap();

    //     let wasm_path = temp_dir.path().join("test_plugin.wasm");
    //     let mut wasm_file = fs::File::create(&wasm_path).unwrap();
    //     wasm_file.write_all(&wasm_bytes).unwrap();

    //     // 2. Create the manifest file
    //     let manifest_path = temp_dir.path().join("test_plugin.json");
    //     let manifest_content = r#"
    //         {
    //             "name": "TestPlugin",
    //             "version": "1.0.0",
    //             "description": "A test plugin",
    //             "plugin_type": "Wasm",
    //             "entry_path": "test_plugin.wasm"
    //         }
    //     "#;
    //     fs::write(&manifest_path, manifest_content).unwrap();

    //     loader
    //         .load_plugins(temp_dir.path().to_str().unwrap())
    //         .await?;

    //     // Optional: Compare the loaded module with the original module
    //     let plugins = loader.plugins.read().await;
    //     let (_, loaded_plugin) = plugins.get("TestPlugin").unwrap();

    //     match loaded_plugin {
    //         LoadedPlugin::Wasm(loaded_module) => {
    //             let loaded_bytes = loaded_module.serialize().unwrap();
    //             assert_eq!(wasm_bytes, loaded_bytes);
    //         }
    //     }

    //     let result = loader.dispatch_event("TestPlugin", "test_event").await;
    //     assert!(result.is_ok());
    //     // assert_eq!(result, Ok(())); // Check that dispatch was successful

    //     temp_dir.close().unwrap();
    //     Ok(())
    // }

    // #[tokio::test]
    // async fn test_plugin_not_found() {
    //     let loader = PluginLoader::new();
    //     let result = loader
    //         .dispatch_event("NonExistentPlugin", "test_event")
    //         .await;
    //     assert!(matches!(result, Err(PluginLoaderError::PluginNotFound(_))));
    // }
}
