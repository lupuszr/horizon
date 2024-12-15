use thiserror::Error;
// use anyhow::Result;
use iroh::{protocol::Router, Endpoint};
use iroh_base::ticket::BlobTicket;
use iroh_blobs::{
    net_protocol::Blobs,
    rpc::client::blobs::{self, ReadAtLen, WrapOption},
    util::{local_pool::LocalPool, SetTagOption},
};

use clap::{Args, Parser, Subcommand};

use crate::error::AppError;
use std::{fs, path::PathBuf, str::FromStr};

#[derive(Debug, Args)]
pub struct HorizonPushReceive {
    #[clap(short, long, required(true))]
    path: PathBuf,

    #[clap(short, long, required(true))]
    url: String,
}

impl HorizonPushReceive {
    pub async fn eval(self) -> Result<bool, AppError> {
        // let abs_path = PathBuf::from_str(path)?.canonicalize()?;
        let Self { path, url } = self;
        // TODO: extract and use a custom discovey node
        let endpoint = Endpoint::builder()
            .discovery_n0()
            .bind()
            .await
            .map_err(|e| AppError::IrohEndpointError(e.to_string()))?;

        // We initialize the Blobs protocol in-memory
        let local_pool = LocalPool::default();
        let blobs = Blobs::memory().build(&local_pool, &endpoint);

        // Now we build a router that accepts blobs connections & routes them
        // to the blobs protocol.
        let router = Router::builder(endpoint)
            .accept(iroh_blobs::ALPN, blobs.clone())
            .spawn()
            .await
            .map_err(|e| AppError::IrohRouterError(e.to_string()))?;

        println!("Indexing file.");

        let blobs = blobs.client();

        let ticket = BlobTicket::from_str(url.as_str())
            .map_err(|e| AppError::IrohBlobTicketReadError(e.to_string()))?;

        println!("Starting download.");

        blobs
            .download(ticket.hash(), ticket.node_addr().clone())
            .await
            .map_err(|e| AppError::IrohBlobDownloadError(e.to_string()))?
            .finish()
            .await
            .map_err(|e| AppError::IrohBlobFinishError(e.to_string()))?;

        println!("Finished download.");

        println!("Copying to destination.");

        let mut file = tokio::fs::File::create(path).await.unwrap();
        let mut reader = blobs
            .read_at(ticket.hash(), 0, ReadAtLen::All)
            .await
            .map_err(|e| AppError::IrohBlobReadError(e.to_string()))?;
        tokio::io::copy(&mut reader, &mut file).await.unwrap();

        println!("Finished copying.");

        tokio::signal::ctrl_c()
            .await
            .map_err(|e| AppError::IOSignalError(e.to_string()))?;

        Ok(true)
    }
}
