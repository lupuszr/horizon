use deno_core::{op2, Extension, JsRuntime, OpDecl, OpState, RuntimeOptions}; // For JavaScript
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};
use thiserror::Error;
use tokio::sync::{Mutex, RwLock};
use wasmtime::{Engine, Module, Store}; // For WASM

pub trait HorizonPushAPI: Send + Sync {
    fn log_event(&self, event: &str);
    // fn trigger_reload(&self);
    // fn get_plugin_info(&self) -> String;
}

struct HorizonPAPI {}

impl HorizonPushAPI for HorizonPAPI {
    fn log_event(&self, event: &str) {
        todo!()
    }
}

#[op2]
fn op_sum(#[serde] nums: Vec<f64>) -> Result<f64, deno_error::JsErrorBox> {
    // Sum inputs
    let sum = nums.iter().fold(0.0, |a, v| a + v);
    // return as a Result<f64, OpError>
    Ok(sum)
}
struct DenoInvoker<T: HorizonPushAPI> {
    api: T,
}

// impl<T: HorizonPushAPI> DenoInvoker<T> {
//     // #[op2]
//     fn log_event(#[serde] file_path: String) -> Result<f64, deno_error::JsErrorBox> {
//         // Sum inputs
//         // let sum = nums.iter().fold(0.0, |a, v| a + v);
//         // return as a Result<f64, OpError>
//         Ok(file_path)
//     }
// }

pub fn initialize_js_runtime(api: HorizonPAPI) -> Extension {
    // let op_log_event = op_sync(move |_, args, _| {
    //     let message: String = args[0].as_str().unwrap().to_string();
    //     api_clone.log_event(&message);
    //     Ok(())
    // });

    let c = move |state: &mut OpState| state.put(api);

    // deno_core::extension!(horizon_js, ops = [op_test], state = c);

    #[op2(fast)]
    pub fn op_test() {
        // api.log_event("go");
        println!("hail");
    }
    // Define the ops
    // let op_log_event = op_sync(move |_, args, _| {
    //     let message: String = args[0].as_str().unwrap().to_string();
    //     api.log_event(&message);
    //     Ok(())
    // });

    // let op_trigger_reload = op_sync(move |_, _, _| {
    //     api.trigger_reload();
    //     Ok(())
    // });

    // let op_get_plugin_info = op_sync(move |_, _, _| Ok(api.get_plugin_info()));

    // // Create the extension
    // let extension = Extension {
    //     name: "horizon_push_extension",
    //     ops: vec![
    //         ("op_log_event", op_log_event),
    //         ("op_trigger_reload", op_trigger_reload),
    //         ("op_get_plugin_info", op_get_plugin_info),
    //     ],
    //     ..Default::default()
    // };
    todo!()
}

#[cfg(test)]
mod tests {
    use super::HorizonPushAPI;

    struct HorizonPushState {
        // Internal state
    }

    impl HorizonPushState {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl HorizonPushAPI for HorizonPushState {
        fn log_event(&self, event: &str) {
            println!("Plugin logged event: {}", event);
        }

        // fn trigger_reload(&self) {
        //     println!("Reloading plugins...");
        //     // Add reload logic here
        // }

        // fn get_plugin_info(&self) -> String {
        //     "HorizonPush v1.0".to_string()
        // }
    }
}
