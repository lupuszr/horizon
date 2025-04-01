use clap::{Parser, Subcommand};
use horizon_core::errors::AppError;
use tokio::sync::mpsc;

use crate::start_server::HorizonServerStart;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: HorizonServerCommand,
}

impl Cli {
    pub async fn eval(self) -> Result<bool, AppError> {
        let (tx_send, _rx_sender) = mpsc::channel(100);
        // let (tx_receive, mut rx_receiver) = mpsc::channel(100);

        // tokio::spawn(async move {
        //     while let Some(ev) = rx_receiver.recv().await {
        //         println!("received event: {:?}", ev);
        //     }
        // });
        let cmd: Result<bool, AppError> = match self.command {
            HorizonServerCommand::StartServer(sm) => sm.eval(tx_send).await,
        };

        cmd?;
        Ok(true)
    }
}

#[derive(Debug, Subcommand)]
pub enum HorizonServerCommand {
    #[command()]
    StartServer(HorizonServerStart),
}
