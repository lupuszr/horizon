use futures_lite::future::Boxed;
use iroh_blobs::provider::{self, CustomEventSender};
use tokio::sync::mpsc;

use crate::event::{EventStats, HorizonChannel};

pub async fn send_horizon_event(tx: mpsc::Sender<HorizonChannel>, event: HorizonChannel) {
    if let Err(e) = tx.send(event.clone()).await {
        eprintln!("Failed to send horizon event {:?} \n with error {e}", event);
    };
}

#[derive(Debug, Clone)]
pub struct IrohClientStatus {
    pub sender: mpsc::Sender<HorizonChannel>,
}

impl CustomEventSender for IrohClientStatus {
    fn send(&self, event: iroh_blobs::provider::Event) -> Boxed<()> {
        self.try_send(event);
        Box::pin(std::future::ready(()))
    }

    fn try_send(&self, event: provider::Event) {
        tracing::info!("{:?}", event);

        let channel_message = match event {
            provider::Event::ClientConnected { connection_id } => {
                Some(HorizonChannel::IrohSenderEvent {
                    status: format!("connection_established"),
                    connection_id,
                    hash: None,
                    stats: None,
                })
            }
            provider::Event::GetRequestReceived {
                connection_id,
                hash,
                ..
            } => Some(HorizonChannel::IrohSenderEvent {
                status: format!("get-request-received"),
                connection_id,
                hash: Some(hash),
                stats: None,
            }),
            // provider::Event::TransferBlobCompleted {
            //     connection_id,
            //     hash,
            //     index,
            //     size,
            //     ..
            // } => Some(HorizonChannel::IrohSenderEvent {
            //     status: format!("transfer blob completed",),
            //     connection_id,
            //     hash: Some(hash),
            //     stats: None,
            // }),
            // provider::Event::TransferProgress { connection_id, request_id, hash, end_offset }
            provider::Event::TransferCompleted {
                connection_id,
                stats,
                ..
            } => Some(HorizonChannel::IrohSenderEvent {
                status: format!("transfer completed",),
                connection_id,
                hash: None,
                stats: Some(EventStats {
                    duration: stats.duration,
                }),
            }),
            provider::Event::TransferAborted { connection_id, .. } => {
                Some(HorizonChannel::IrohSenderEvent {
                    status: format!("transfer aborted"),
                    connection_id,
                    hash: None,
                    stats: None,
                })
            }
            _ => None,
        };

        if let Some(msg) = channel_message {
            // let x = self.sender.try_send(msg);
            if let Err(e) = self.sender.try_send(msg) {
                tracing::warn!("Failed to send message: {:?}", e);
            }
        }
    }
}
