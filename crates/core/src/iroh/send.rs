use std::path::PathBuf;

use iroh_blobs::ticket::BlobTicket;
use tokio::sync::mpsc;

use crate::{errors::AppError, event::HorizonChannel};

use super::common::IrohState;

/// Processes a blob by adding it to the blob store, sends indexing events, and returns a `BlobTicket`.
///
/// This function performs several operations on a blob, including adding it from a specified path to a blob store, sending status events through a channel, and generating a `BlobTicket` that contains metadata about the processed blob.
///
/// The steps performed by this function are:
/// 1. **Indexing Started**: Sends an event via the `tx` channel indicating the start of the blob indexing process. The event contains the status (`"IndexingStarted"`), the path of the blob, and an empty hash (since the blob has not been fully processed yet).
/// 2. **Blob Addition**: Adds the blob to the blob store using the provided `path`. The blob is added with default options such as `Auto` for setting the tag, and `NoWrap` for wrapping the blob.
/// 3. **Error Handling**: If the blob store addition fails, the error is mapped to an `AppError::IrohBlobStoreError` for more specific context.
/// 4. **Indexing Completed**: Once the blob is successfully added to the blob store, an event is sent via the `tx` channel indicating that the indexing has completed. This event includes the `hash` of the blob (if available) and the path.
/// 5. **Ticket Creation**: The function retrieves the router's address and creates a `BlobTicket` that includes metadata about the processed blob, such as its hash and format. This ticket is then sent to the `tx` channel, notifying listeners that the blob processing is complete.
/// 6. **Error Propagation**: Any errors during the sending of events or creating the `BlobTicket` are handled and mapped into `AppError::InternalChannelError` or `AppError::IrohEndpointError` for better traceability.
///
/// # Arguments
///
/// * `path` - A `String` representing the path to the blob to be indexed.
/// * `tx` - A channel sender (`mpsc::Sender<HorizonChannel>`) for sending events and results about the indexing process. The events sent include the start and completion of the indexing process, as well as the final `BlobTicket`.
/// * `iroh_state` - A reference to the `IrohState` that holds the blob store and router for the current process. The blob store is used to add the blob, and the router is used to retrieve the endpoint address needed for the `BlobTicket`.
///
/// # Returns
///
/// * `Result<BlobTicket, AppError>` - A result containing the created `BlobTicket` if the process is successful, or an `AppError` if any part of the process fails. The `AppError` can indicate failures such as blob store errors, channel communication errors, or endpoint retrieval errors.
///
/// # Errors
///
/// This function can return the following errors:
///
/// * `AppError::InternalChannelError`: If sending events through the channel (`tx.send`) fails.
/// * `AppError::IrohBlobStoreError`: If there is an error adding the blob to the blob store.
/// * `AppError::IrohEndpointError`: If retrieving the router's endpoint address fails.
/// * `AppError::IrohBlobTicketCreationError`: If creating the `BlobTicket` fails.
pub async fn index_and_expose(
    iroh_state: IrohState,
    path: PathBuf,
    tx: mpsc::Sender<HorizonChannel>,
) -> Result<BlobTicket, AppError> {
    let blobs = iroh_state.blobs;
    let router = iroh_state.router;
    // let blobs_store = iroh.clone().blobs_store;
    tx.send(HorizonChannel::IrohIndexingEvent {
        status: "IndexingStarted".to_string(),
        path: path.clone(),
        hash: None,
    })
    .await
    .map_err(|err| AppError::InternalChannelError(err.to_string()))?;

    let blob = blobs
        .add_from_path(
            path.clone(),
            false,
            iroh_blobs::util::SetTagOption::Auto,
            iroh_blobs::rpc::client::blobs::WrapOption::NoWrap,
        )
        .await
        .map_err(|err| AppError::IrohBlobStoreError(err.to_string()))?
        .finish()
        .await
        .map_err(|err| AppError::IrohBlobStoreError(err.to_string()))?;

    // let outcome: iroh_blobs::rpc::client::blobs::AddOutcome = import_progress
    //     .await
    //     .map_err(|err| AppError::IrohBlobStoreError(err.to_string()))?;

    let hash = blob.hash;

    tx.send(HorizonChannel::IrohIndexingEvent {
        status: "IndexingCompleted".to_string(),
        path: path.clone(),
        hash: Some(hash.clone()),
    })
    .await
    .map_err(|err| AppError::InternalChannelError(err.to_string()))?;

    let addr = router
        .endpoint()
        .node_addr()
        .await
        .map_err(|err| AppError::IrohEndpointError(err.to_string()))?;

    let ticket = BlobTicket::new(addr, hash, blob.format)
        .map_err(|err| AppError::IrohBlobTicketCreationError(err.to_string()))?;

    tx.send(HorizonChannel::IrohTicket(ticket.to_string()))
        .await
        .map_err(|err| AppError::InternalChannelError(err.to_string()))?;
    Ok(ticket)
}
