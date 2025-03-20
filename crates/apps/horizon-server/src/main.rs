pub mod cli;
pub mod start_server;
use clap::Parser;
use cli::Cli;
use horizon_core::errors::AppError;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // let args = Cli::try_parse().unwrap();
    let args = Cli::parse();
    args.eval().await?;
    // args
    // args.eval()c

    Ok(())
}
