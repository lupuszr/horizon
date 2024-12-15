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
use std::{fs, path::PathBuf};

#[derive(Debug, Args)]
pub struct HorizonPushSend {
    #[clap(short, long, default_value = "content")]
    path: PathBuf,
}

impl HorizonPushSend {
    pub async fn eval(self) -> Result<bool, AppError> {
        // let abs_path = PathBuf::from_str(path)?.canonicalize()?;
        let Self { path } = self;
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

        let blob = blobs
            .add_from_path(path.clone(), true, SetTagOption::Auto, WrapOption::NoWrap)
            .await
            .map_err(|e| AppError::IrohBlobPathError(e.to_string()))?
            .finish()
            .await
            .map_err(|e| AppError::IrohBlobFinishError(e.to_string()))?;

        let node_id = router.endpoint().node_id();
        let ticket = BlobTicket::new(node_id.into(), blob.hash, blob.format)
            .map_err(|e| AppError::IrohBlobTicketCreationError(e.to_string()))?;

        println!("File analyzed. Fetch this file by running:");
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
