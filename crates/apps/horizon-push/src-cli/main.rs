use clap::Parser;
use horizon_core::errors::AppError;

pub mod cli;
pub mod common;
pub mod error;
pub mod receive;
pub mod send;

use crate::cli::Cli;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // let args = Cli::try_parse().unwrap();
    let args = Cli::parse();
    args.eval().await?;
    // args
    // args.eval()c

    Ok(())
}
