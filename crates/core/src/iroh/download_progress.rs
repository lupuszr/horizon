use anyhow::{bail, Result};

use console::style;
use futures_lite::{Stream, StreamExt};
use indicatif::{
    HumanBytes, HumanDuration, MultiProgress, ProgressBar, ProgressDrawTarget, ProgressState,
    ProgressStyle,
};
use iroh_blobs::{
    get::{db::DownloadProgress, progress::BlobProgress, Stats},
    Hash,
};
use tokio::sync::mpsc;

use crate::{
    errors::AppError,
    event::{HorizonChannel, ReceiveEventStats, ReceiveProgress},
    iroh::client_status::send_horizon_event,
};

/// Creates a [`ProgressBar`] with some defaults for the overall progress.
fn make_overall_progress() -> ProgressBar {
    let pb = ProgressBar::hidden();
    pb.enable_steady_tick(std::time::Duration::from_millis(100));
    pb.set_style(
        ProgressStyle::with_template(
            "{msg}{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len}",
        )
        .unwrap()
        .progress_chars("#>-"),
    );
    pb
}

/// Creates a [`ProgressBar`] with some defaults for the individual progress.
fn make_individual_progress() -> ProgressBar {
    let pb = ProgressBar::hidden();
    pb.enable_steady_tick(std::time::Duration::from_millis(100));
    pb.set_style(
        ProgressStyle::with_template("{msg}{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .with_key(
                "eta",
                |state: &ProgressState, w: &mut dyn std::fmt::Write| {
                    write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
                },
            )
            .progress_chars("#>-"),
    );
    pb
}

pub async fn show_download_progress_cli(
    hash: Hash,
    mut stream: impl Stream<Item = Result<DownloadProgress>> + Unpin,
) -> Result<()> {
    eprintln!("Fetching: {}", hash);
    let mp = MultiProgress::new();
    mp.set_draw_target(ProgressDrawTarget::stderr());
    let op = mp.add(make_overall_progress());
    let ip = mp.add(make_individual_progress());
    op.set_message(format!("{} Connecting ...\n", style("[1/3]").bold().dim()));
    let mut seq = false;
    while let Some(x) = stream.next().await {
        match x? {
            DownloadProgress::InitialState(state) => {
                if state.connected {
                    op.set_message(format!("{} Requesting ...\n", style("[2/3]").bold().dim()));
                }
                if let Some(count) = state.root.child_count {
                    op.set_message(format!(
                        "{} Downloading {} blob(s)\n",
                        style("[3/3]").bold().dim(),
                        count + 1,
                    ));
                    op.set_length(count + 1);
                    op.reset();
                    op.set_position(state.current.map(u64::from).unwrap_or(0));
                    seq = true;
                }
                if let Some(blob) = state.get_current() {
                    if let Some(size) = blob.size {
                        ip.set_length(size.value());
                        ip.reset();
                        match blob.progress {
                            BlobProgress::Pending => {}
                            BlobProgress::Progressing(offset) => ip.set_position(offset),
                            BlobProgress::Done => ip.finish_and_clear(),
                        }
                        if !seq {
                            op.finish_and_clear();
                        }
                    }
                }
            }
            DownloadProgress::FoundLocal { .. } => {}
            DownloadProgress::Connected => {
                op.set_message(format!("{} Requesting ...\n", style("[2/3]").bold().dim()));
            }
            DownloadProgress::FoundHashSeq { children, .. } => {
                op.set_message(format!(
                    "{} Downloading {} blob(s)\n",
                    style("[3/3]").bold().dim(),
                    children + 1,
                ));
                op.set_length(children + 1);
                op.reset();
                seq = true;
            }
            DownloadProgress::Found { size, child, .. } => {
                if seq {
                    op.set_position(child.into());
                } else {
                    op.finish_and_clear();
                }
                ip.set_length(size);
                ip.reset();
            }
            DownloadProgress::Progress { offset, .. } => {
                ip.set_position(offset);
            }
            DownloadProgress::Done { .. } => {
                ip.finish_and_clear();
            }
            DownloadProgress::AllDone(Stats {
                bytes_read,
                elapsed,
                ..
            }) => {
                op.finish_and_clear();
                eprintln!(
                    "Transferred {} in {}, {}/s",
                    HumanBytes(bytes_read),
                    HumanDuration(elapsed),
                    HumanBytes((bytes_read as f64 / elapsed.as_secs_f64()) as u64)
                );
                break;
            }
            DownloadProgress::Abort(e) => {
                bail!("download aborted: {}", e);
            }
        }
    }
    Ok(())
}

pub async fn show_download_progress(
    hash: Hash,
    mut stream: impl Stream<Item = Result<DownloadProgress>> + Unpin,
    tx: mpsc::Sender<HorizonChannel>,
) -> Result<(), AppError> {
    if let Err(e) = tx
        .send(HorizonChannel::IrohReceiverEvent {
            status: "DownloadStarted".into(),
            progress: None,
            stats: None,
            hash: Some(hash),
            info: None,
        })
        .await
    {
        eprintln!("Failed to send receive event {e}");
    };
    eprintln!("Fetching: {}", hash);
    while let Some(x) = stream.next().await {
        let x = x.map_err(|err| AppError::IrohDownloadProgressError(err.to_string()));
        match x? {
            DownloadProgress::InitialState(state) => {
                if state.connected {
                    send_horizon_event(
                        tx.clone(),
                        HorizonChannel::IrohReceiverEvent {
                            status: "ConnectedToProvider".into(),
                            progress: None,
                            stats: None,
                            hash: Some(hash),
                            info: Some("Requesting".into()),
                        },
                    )
                    .await;
                }
                if let Some(count) = state.root.child_count {
                    let info = format!(
                        "{} Downloading {} blob(s)\n",
                        style("[3/3]").bold().dim(),
                        count + 1,
                    );
                    send_horizon_event(
                        tx.clone(),
                        HorizonChannel::IrohReceiverEvent {
                            status: "DownloadingBlobsFoundHashSeq".into(),
                            progress: Some(ReceiveProgress {
                                blob_number: count + 1,
                                offset: 0,
                            }),
                            stats: None,
                            hash: Some(hash),
                            info: Some(info),
                        },
                    )
                    .await;
                }
                if let Some(_blob) = state.get_current() {
                    // TODO: expand if needed
                }
            }
            DownloadProgress::FoundLocal { .. } => {}
            DownloadProgress::Connected => {
                send_horizon_event(
                    tx.clone(),
                    HorizonChannel::IrohReceiverEvent {
                        status: "ConnectedToProvider".into(),
                        progress: None,
                        stats: None,
                        hash: Some(hash),
                        info: Some("Requesting".into()),
                    },
                )
                .await;
            }
            DownloadProgress::FoundHashSeq { children, .. } => {
                let count = children;
                let info = format!(
                    "{} Downloading {} blob(s)\n",
                    style("[3/3]").bold().dim(),
                    count + 1,
                );
                send_horizon_event(
                    tx.clone(),
                    HorizonChannel::IrohReceiverEvent {
                        status: "DownloadingBlobsFoundHashSeq".into(),
                        progress: Some(ReceiveProgress {
                            blob_number: count + 1,
                            offset: 0,
                        }),
                        stats: None,
                        hash: Some(hash),
                        info: Some(info),
                    },
                )
                .await;
            }
            DownloadProgress::Found { .. } => {}
            DownloadProgress::Progress { .. } => {}
            DownloadProgress::Done { .. } => {}
            DownloadProgress::AllDone(Stats {
                bytes_read,
                elapsed,
                ..
            }) => {
                let throughput = (bytes_read as f64 / elapsed.as_secs_f64()) as u64;
                let info = format!(
                    "Transferred {} in {}, {}/s",
                    HumanBytes(bytes_read),
                    HumanDuration(elapsed),
                    HumanBytes(throughput)
                );
                send_horizon_event(
                    tx.clone(),
                    HorizonChannel::IrohReceiverEvent {
                        status: "DownloadingBlobsAllDone".into(),
                        progress: None,
                        stats: Some(ReceiveEventStats {
                            duration: elapsed,
                            bytes_read,
                            throughput,
                        }),
                        hash: Some(hash),
                        info: Some(info),
                    },
                )
                .await;
                break;
            }
            DownloadProgress::Abort(_e) => {
                send_horizon_event(
                    tx.clone(),
                    HorizonChannel::IrohReceiverEvent {
                        status: "DownloadingBlobsAbort".into(),
                        progress: None,
                        stats: None,
                        hash: None,
                        info: None,
                    },
                )
                .await;
                return Err(AppError::IrohDownloadProgressError(
                    "Download aborted".into(),
                ));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::stream;
    use iroh_blobs::get::progress::{BlobState, TransferState};
    use std::{collections::HashMap, time::Duration};
    use tokio::sync::mpsc;

    fn create_test_hash() -> Hash {
        Hash::new([0u8; 32])
    }

    #[tokio::test]
    async fn test_successful_download() {
        let (tx, mut rx) = mpsc::channel(32);
        let hash = create_test_hash();

        let children = HashMap::new();
        let progress_stream = stream::iter(vec![
            Ok(DownloadProgress::InitialState(TransferState {
                children,
                current: None,
                progress_id_to_blob: HashMap::new(),
                connected: true,
                root: BlobState {
                    hash,
                    size: None,
                    progress: BlobProgress::Pending,
                    local_ranges: None,
                    child_count: Some(2),
                },
            })),
            Ok(DownloadProgress::AllDone(Stats {
                bytes_read: 1000,
                elapsed: Duration::from_secs(2),
                bytes_written: 1000,
            })),
        ]);

        let handle = tokio::spawn(async move {
            show_download_progress(hash, progress_stream, tx)
                .await
                .unwrap();
        });

        // Verify initial "DownloadStarted" event
        if let Some(HorizonChannel::IrohReceiverEvent { status, .. }) = rx.recv().await {
            assert_eq!(status, "DownloadStarted");
        }

        // Verify "ConnectedToProvider" event
        if let Some(HorizonChannel::IrohReceiverEvent { status, info, .. }) = rx.recv().await {
            assert_eq!(status, "ConnectedToProvider");
            assert_eq!(info, Some("Requesting".into()));
        }

        // Verify "DownloadingBlobsFoundHashSeq" event
        if let Some(HorizonChannel::IrohReceiverEvent {
            status, progress, ..
        }) = rx.recv().await
        {
            assert_eq!(status, "DownloadingBlobsFoundHashSeq");
            assert_eq!(progress.unwrap().blob_number, 3);
        }

        // Verify final "DownloadingBlobsAllDone" event
        if let Some(HorizonChannel::IrohReceiverEvent { status, stats, .. }) = rx.recv().await {
            assert_eq!(status, "DownloadingBlobsAllDone");
            let stats = stats.unwrap();
            assert_eq!(stats.bytes_read, 1000);
            assert_eq!(stats.throughput, 500); // 1000 bytes / 2 seconds
        }

        handle.await.unwrap();
    }

    #[tokio::test]
    async fn test_download_abort() {
        let (tx, mut rx) = mpsc::channel(32);
        let hash = create_test_hash();

        let children = HashMap::new();

        let progress_stream = stream::iter(vec![
            Ok(DownloadProgress::InitialState(TransferState {
                children,
                current: None,
                progress_id_to_blob: HashMap::new(),
                connected: true,
                root: BlobState {
                    hash,
                    size: None,
                    progress: BlobProgress::Pending,
                    local_ranges: None,
                    child_count: None,
                },
            })),
            Ok(DownloadProgress::Abort(serde_error::Error::new(
                &AppError::IrohDownloadProgressError("Download Aborted".to_string()),
            ))),
        ]);

        let result = show_download_progress(hash, progress_stream, tx.clone()).await;
        assert!(result.is_err());

        drop(tx);
        // Verify events sequence
        let mut events = Vec::new();
        while let Some(event) = rx.recv().await {
            events.push(event);
        }

        assert_eq!(events.len(), 3); // DownloadStarted, ConnectedToProvider,  DownloadingBlobsAbort

        if let HorizonChannel::IrohReceiverEvent { status, .. } = &events[2] {
            assert_eq!(status, "DownloadingBlobsAbort");
        }
    }

    #[tokio::test]
    async fn test_empty_stream() {
        let (tx, _rx) = mpsc::channel(32);
        let hash = create_test_hash();
        let empty_stream = stream::iter::<Vec<Result<DownloadProgress, _>>>(vec![]);

        let result = show_download_progress(hash, empty_stream, tx).await;
        assert!(result.is_ok());
    }

    // Helper function to create a stream that simulates network disconnection
    fn create_disconnecting_stream() -> impl Stream<Item = Result<DownloadProgress>> {
        stream::iter(vec![
            Ok(DownloadProgress::InitialState(TransferState {
                children: HashMap::new(),
                current: None,
                progress_id_to_blob: HashMap::new(),
                connected: true,
                root: BlobState {
                    hash: create_test_hash(),
                    size: None,
                    progress: BlobProgress::Pending,
                    local_ranges: None,
                    child_count: None,
                },
            })),
            Ok(DownloadProgress::Abort(serde_error::Error::new(
                &AppError::IrohDownloadProgressError("Download Aborted".to_string()),
            ))),
        ])
    }

    #[tokio::test]
    async fn test_connection_handling() {
        let (tx, mut rx) = mpsc::channel(32);
        let hash = create_test_hash();
        let stream = create_disconnecting_stream();

        let result = show_download_progress(hash, stream, tx).await;
        assert!(result.is_err());

        // Verify we get the connection events in order
        let mut events = Vec::new();
        while let Some(event) = rx.recv().await {
            events.push(event);
        }

        println!("events:: {:?}", events);

        // Check for initial DownloadStarted
        if let HorizonChannel::IrohReceiverEvent { status, .. } = &events[0] {
            assert_eq!(status, "DownloadStarted");
        }

        // Check for Connected event
        if let HorizonChannel::IrohReceiverEvent { status, .. } = &events[1] {
            assert_eq!(status, "ConnectedToProvider");
        }

        // Check for final Abort event
        if let HorizonChannel::IrohReceiverEvent { status, .. } = &events[2] {
            assert_eq!(status, "DownloadingBlobsAbort");
        }
    }
}
