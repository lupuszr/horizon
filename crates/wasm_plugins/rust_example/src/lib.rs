// use horizon_core::event::HorizonChannel;
use serde_json;
// use wasm_bindgen::prelude::*;

use std::{path::PathBuf, time::Duration};

// use iroh_blobs::Hash;
use serde::{Deserialize, Serialize};
type Hash = String;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EventStats {
    pub duration: Duration,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReceiveEventStats {
    pub duration: Duration,
    pub throughput: u64,
    pub bytes_read: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ReceiveProgressType {
    Download,
    Export,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReceiveProgress {
    pub blob_number: u64,
    pub offset: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum HorizonChannel {
    IrohIndexingEvent {
        status: String,
        path: PathBuf,
        hash: Option<Hash>,
    },
    IrohSenderEvent {
        status: String,
        connection_id: u64,
        hash: Option<Hash>,
        stats: Option<EventStats>,
    },
    IrohReceiverEvent {
        status: String,
        info: Option<String>,
        hash: Option<Hash>,
        progress: Option<ReceiveProgress>,
        stats: Option<ReceiveEventStats>,
    },
    IrohTicket(String),
}

// #[wasm_bindgen] // Use wasm_bindgen macro
#[no_mangle]
pub extern "C" fn handle_event(event_type: i32, event_json_ptr: i32, event_json_len: i32) -> i32 {
    let mut event_json;
    unsafe {
        // Access the memory using the pointer and length
        let event_json_slice =
            std::slice::from_raw_parts(event_json_ptr as *const u8, event_json_len as usize);
        event_json = std::str::from_utf8(event_json_slice).unwrap(); // Convert to &str

        // ... rest of your handle_event logic (using event_json) ...
    }
    match event_type {
        0 => {
            // IrohIndexingEvent
            let event: HorizonChannel = serde_json::from_str(event_json)
                // .map_err(|e| JsValue::from_str(&e.to_string()))
                .unwrap();
            if let HorizonChannel::IrohIndexingEvent { status, .. } = event {
                println!("Rust WASM: Indexing event: {}", status);
                0 as i32 // Success
            } else {
                -1 // Type mismatch
            }
        }
        1 => {
            // IrohSenderEvent
            let event: HorizonChannel = serde_json::from_str(event_json)
                // .map_err(|e| JsValue::from_str(&e.to_string()))
                .unwrap();
            if let HorizonChannel::IrohSenderEvent {
                status,
                connection_id,
                ..
            } = event
            {
                println!("Rust WASM: Sender event: {}, {}", status, connection_id);
                0
            } else {
                -1
            }
        }
        2 => {
            // IrohReceiverEvent
            let event: HorizonChannel = serde_json::from_str(event_json)
                // .map_err(|e| JsValue::from_str(&e.to_string()))
                .unwrap();
            if let HorizonChannel::IrohReceiverEvent { status, info, .. } = event {
                println!("Rust WASM: Receiver event: {}, {:?}", status, info);
                0
            } else {
                -1
            }
        }
        3 => {
            // IrohTicket
            let event: HorizonChannel = serde_json::from_str(event_json)
                // .map_err(|e| JsValue::from_str(&e.to_string()))
                .unwrap();
            if let HorizonChannel::IrohTicket(ticket) = event {
                println!("Rust WASM: Ticket: {}", ticket);
                0
            } else {
                -1
            }
        }
        _ => -1, // Error: Unknown event type
    }
    // return 0;
}
