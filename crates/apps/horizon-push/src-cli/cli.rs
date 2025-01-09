use clap::{Parser, Subcommand};
use tokio::sync::mpsc;

use crate::error::AppError;
use crate::receive::HorizonPushReceive;
use crate::send::HorizonPushSend;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: HorizonPushCommand,
}

impl Cli {
    pub async fn eval(self) -> Result<bool, AppError> {
        let (tx_send, _rx) = mpsc::channel(1);
        let (tx_receive, _rx) = mpsc::channel(1);

        let cmd: Result<bool, AppError> = match self.command {
            HorizonPushCommand::Send(sm) => sm.eval(tx_send).await,
            HorizonPushCommand::Receive(sm) => sm.eval(tx_receive).await,
        };
        cmd?;
        Ok(true)
    }
}

#[derive(Debug, Subcommand)]
pub enum HorizonPushCommand {
    #[command()]
    Send(HorizonPushSend),
    #[command()]
    Receive(HorizonPushReceive),
}
