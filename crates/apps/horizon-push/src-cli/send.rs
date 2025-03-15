use horizon_core::{
    errors::AppError,
    event::HorizonChannel,
    iroh::{common::IrohState, send::index_and_expose},
};

use clap::Args;
use tokio::sync::mpsc;

use crate::common::CommonArgs;
use std::{fs, path::PathBuf};

#[derive(Debug, Args)]
pub struct HorizonPushSend {
    #[clap(short, long, required(true))]
    pub path: PathBuf,

    #[clap(short, long, default_value = None)]
    pub base_path: Option<PathBuf>,

    #[clap(flatten)]
    pub common: CommonArgs,
}

impl HorizonPushSend {
    pub async fn eval(self, sender: mpsc::Sender<HorizonChannel>) -> Result<bool, AppError> {
        let Self {
            path, base_path, ..
        } = self;

        let iroh_base_path = if base_path.is_none() {
            let mut ph = dirs_next::home_dir().ok_or(AppError::InternalStateError(
                "Could not determine home folder".to_string(),
            ))?;
            ph.push(".horizon-push-sender");
            ph
        } else {
            base_path.unwrap()
        };

        let iroh_state = IrohState::new(iroh_base_path.clone(), sender.clone())
            .await
            .map_err(|err| AppError::IrohHorizonStateSetupError(err.to_string()))?;

        let ticket = index_and_expose(iroh_state.clone(), path.clone(), sender.clone()).await?;
        let mut ticket_path = iroh_base_path.clone();
        ticket_path.push("ticket.horizon");
        fs::write(ticket_path, ticket.clone().to_string()).unwrap();
        println!(
            "horizon-cli receive --url {ticket} --path {}",
            path.to_str().unwrap()
        );

        tokio::signal::ctrl_c()
            .await
            .map_err(|e| AppError::IOSignalError(e.to_string()))?;

        Ok(true)
    }
}
