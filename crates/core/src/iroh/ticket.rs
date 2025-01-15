use std::path::PathBuf;

use iroh::protocol::Router;
use iroh_blobs::{ticket::BlobTicket, BlobFormat};
use iroh_docs::rpc::AddrInfoOptions;

use crate::errors::AppError;

use super::common::import;

pub async fn generate_blob_ticket(
    path: PathBuf,
    db: impl iroh_blobs::store::Store,
    router: Router,
) -> Result<BlobTicket, AppError> {
    // Import and create ticket
    let (temp_tag, size, collection) = import(path.clone(), db.clone()).await?;
    let hash = *temp_tag.hash();
    let mut addr = router
        .endpoint()
        .node_addr()
        .await
        .map_err(|err| AppError::IrohEndpointError(err.to_string()))?;
    // addr.apply_options(AddrInfoOptions::RelayAndAddresses);
    let ticket = BlobTicket::new(addr, hash, BlobFormat::HashSeq)
        .map_err(|err| AppError::IrohBlobTicketCreationError(err.to_string()))?;

    println!("ticket is:: {}", ticket.clone());

    Ok(ticket)
}
