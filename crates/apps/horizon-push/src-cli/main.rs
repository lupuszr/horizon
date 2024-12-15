use clap::Parser;
use std::{path::PathBuf, str::FromStr};
use thiserror::Error;
// use anyhow::Result;
use iroh::{protocol::Router, Endpoint};
use iroh_base::ticket::BlobTicket;
use iroh_blobs::{
    net_protocol::Blobs,
    rpc::client::blobs::{ReadAtLen, WrapOption},
    util::{local_pool::LocalPool, SetTagOption},
};

pub mod cli;
pub mod error;
pub mod receive;
pub mod send;

use crate::cli::Cli;
use crate::error::AppError;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // let args = Cli::try_parse().unwrap();
    let args = Cli::parse();
    args.eval().await?;
    // args
    // args.eval()c

    Ok(())
}
