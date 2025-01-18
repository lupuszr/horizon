use std::path::PathBuf;

use futures::Stream;
use futures_lite::StreamExt;
use iroh_blobs::{
    export::ExportProgress, net_protocol::DownloadMode, rpc::client::blobs::DownloadOptions,
    store::ExportFormat, ticket::BlobTicket, util::SetTagOption, BlobFormat,
};
use tokio::sync::mpsc;

use crate::{errors::AppError, iroh::download_progress::show_download_progress};

use super::{client_status::HorizonChannel, common::IrohState};

pub async fn show_export_progress(
    mut stream: impl Stream<Item = anyhow::Result<ExportProgress>> + Unpin,
    _tx: mpsc::Sender<HorizonChannel>,
) -> Result<(), AppError> {
    while let Some(x) = stream.next().await {
        let _x = x.map_err(|err| AppError::IrohBlobExportProgressError(err.to_string()))?;
        // match x {
        //     ExportProgress::Found {
        //         id,
        //         hash,
        //         size,
        //         outpath,
        //         meta,
        //     } => todo!(),
        //     ExportProgress::Progress { id, offset } => todo!(),
        //     ExportProgress::Done { id } => todo!(),
        //     ExportProgress::AllDone => todo!(),
        //     ExportProgress::Abort(error) => todo!(),
        // }
    }
    Ok(())
}

pub async fn fetch_and_export(
    ticket: BlobTicket,
    iroh_state: IrohState,
    path: PathBuf,
    tx: mpsc::Sender<HorizonChannel>,
) -> Result<(), AppError> {
    let blobs = iroh_state.blobs;
    let mut stream = blobs
        .download_with_opts(
            ticket.hash(),
            DownloadOptions {
                format: ticket.format(),
                nodes: vec![ticket.node_addr().clone()],
                tag: SetTagOption::Auto,
                mode: DownloadMode::Direct,
            },
        )
        .await
        .map_err(|err| AppError::IrohStreamError(err.to_string()))?;

    println!("blob format:: {}", ticket.format());

    show_download_progress(ticket.hash(), &mut stream, tx.clone()).await?;

    let recursive = ticket.format() == BlobFormat::HashSeq;
    // let mode = match stable {
    //     true => ExportMode::TryReference,
    //     false => ExportMode::Copy,
    // };
    let format = match recursive {
        true => ExportFormat::Collection,
        false => ExportFormat::Blob,
    };

    if let Err(e) = tx
        .send(HorizonChannel::IrohReceiverEvent {
            status: "ExportStarted".into(),
            progress: None,
            stats: None,
            hash: Some(ticket.hash()),
            info: None,
        })
        .await
    {
        eprintln!("Failed to send receive event {e}");
    };

    let stream = blobs
        .export(
            ticket.hash(),
            path,
            format,
            iroh_blobs::store::ExportMode::Copy,
        )
        .await
        .map_err(|err| AppError::IrohBlobExportError(err.to_string()))?;

    // TODO: use show_export_progress
    stream
        .await
        .map_err(|err| AppError::IrohBlobExportProgressError(err.to_string()))?;

    Ok(())
}
