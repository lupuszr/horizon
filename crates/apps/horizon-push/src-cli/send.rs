use data_encoding::HEXLOWER;
use indicatif::HumanBytes;
use rand::Rng;
// use anyhow::Result;
use iroh::{protocol::Router, AddrInfoOptions, Endpoint};
use iroh_base::ticket::BlobTicket;
use iroh_blobs::{net_protocol::Blobs, util::local_pool::LocalPool, BlobFormat};

use clap::Args;
use tokio::sync::mpsc;

use crate::{
    common::{import, CommonArgs, SendStatus},
    error::AppError,
};
use std::{fs, path::PathBuf};

#[derive(Debug, Args)]
pub struct HorizonPushSend {
    #[clap(short, long, default_value = "content")]
    pub path: PathBuf,

    #[clap(flatten)]
    pub common: CommonArgs,
}

impl HorizonPushSend {
    pub async fn eval(self, sender: mpsc::Sender<String>) -> Result<bool, AppError> {
        let Self { path, common } = self;
        // TODO: extract and use a custom discovey node
        let endpoint = Endpoint::builder()
            .discovery_n0()
            .bind()
            .await
            .map_err(|e| AppError::IrohEndpointError(e.to_string()))?;

        let suffix = rand::thread_rng().gen::<[u8; 16]>();
        let cwd = std::env::current_dir().unwrap();
        let blobs_data_dir = cwd.join(format!(".horizon-push-{}", HEXLOWER.encode(&suffix)));
        if blobs_data_dir.exists() {
            println!(
                "can not share twice from the same directory: {}",
                cwd.display(),
            );
            std::process::exit(1);
        }

        // We initialize the Blobs protocol in-memory
        let local_pool = LocalPool::default();
        let ps = SendStatus::new();
        let blobs = Blobs::persistent(&blobs_data_dir)
            .await
            .map_err(|err| AppError::IrohBlobStoreError(err.to_string()))?
            .events(ps.new_client().into())
            .build(local_pool.handle(), &endpoint);

        // Now we build a router that accepts blobs connections & routes them
        // to the blobs protocol.
        let router = Router::builder(endpoint)
            .accept(iroh_blobs::ALPN, blobs.clone())
            .spawn()
            .await
            .map_err(|e| AppError::IrohRouterError(e.to_string()))?;

        println!("Indexing file.");

        let (temp_tag, size, collection) = import(path.clone(), blobs.store().clone()).await?;
        let hash = *temp_tag.hash();

        // make a ticket
        let mut addr = router
            .endpoint()
            .node_addr()
            .await
            .map_err(|err| AppError::IrohEndpointError(err.to_string()))?;
        addr.apply_options(AddrInfoOptions::RelayAndAddresses);
        let ticket = BlobTicket::new(addr, hash, BlobFormat::HashSeq)
            .map_err(|err| AppError::IrohBlobTicketCreationError(err.to_string()))?;
        let entry_type = if path.is_file() { "file" } else { "directory" };
        println!(
            "imported {} {}, {}, hash {}",
            entry_type,
            path.display(),
            HumanBytes(size),
            &hash.to_string()
        );
        if common.verbose > 0 {
            for (name, hash) in collection.iter() {
                println!("    {} {name}", hash.to_string());
            }
        }

        drop(temp_tag);

        // Send the ticket through the channel
        sender.send(ticket.clone().to_string()).await.unwrap();

        println!("File analyzed. Fetch this file by running:");

        fs::write("ticket.horizon", ticket.clone().to_string()).unwrap();
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
