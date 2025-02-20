use std::{path::PathBuf, time::Duration};

use iroh_blobs::Hash;
use serde::{Deserialize, Serialize};

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
