use horizon_core::errors::AppError;
use horizon_core::iroh::receive::fetch_and_export;
use horizon_core::iroh::{client_status::HorizonChannel, common::IrohState};
use iroh_blobs::ticket::BlobTicket;

use clap::Args;
use tokio::sync::mpsc::{self};

use crate::common::CommonArgs;
use std::{path::PathBuf, str::FromStr};

#[derive(Debug, Args)]
pub struct HorizonPushReceive {
    #[clap(short, long, default_value = None)]
    pub base_path: Option<PathBuf>,

    #[clap(short, long, required(true))]
    pub path: PathBuf,

    #[clap(short, long, required(true))]
    pub url: String,

    #[clap(flatten)]
    pub common: CommonArgs,
}

impl HorizonPushReceive {
    pub async fn eval(self, sender: mpsc::Sender<HorizonChannel>) -> Result<bool, AppError> {
        let Self {
            path,
            url,
            base_path,
            ..
        } = self;

        let iroh_base_path = if base_path.is_none() {
            let mut ph = dirs_next::home_dir().ok_or(AppError::InternalStateError(
                "Could not determine home folder".to_string(),
            ))?;
            ph.push(".horizon-push-receiver");
            ph
        } else {
            base_path.unwrap()
        };

        let iroh_state = IrohState::new(iroh_base_path.clone(), sender.clone())
            .await
            .map_err(|err| AppError::IrohHorizonStateSetupError(err.to_string()))?;

        println!("Indexing file.");

        let ticket = BlobTicket::from_str(url.as_str())
            .map_err(|e| AppError::IrohBlobTicketReadError(e.to_string()))?;

        fetch_and_export(ticket, iroh_state, path, sender).await?;

        Ok(true)
    }
}
