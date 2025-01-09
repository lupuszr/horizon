// use anyhow::Result;
use console::style;
use indicatif::{
    HumanBytes, HumanDuration, MultiProgress, ProgressBar, ProgressDrawTarget, ProgressStyle,
};
use iroh::{Endpoint, RelayMode};
use iroh_base::ticket::BlobTicket;
use iroh_blobs::{
    format::collection::Collection,
    get::{db::DownloadProgress, request::get_hash_seq_and_sizes},
    HashAndFormat,
};

use clap::Args;
use tokio::sync::mpsc;

use crate::{
    common::{export, get_or_create_secret, CommonArgs},
    error::AppError,
};
use std::{collections::BTreeMap, path::PathBuf, str::FromStr};

#[derive(Debug, Args)]
pub struct HorizonPushReceive {
    #[clap(short, long, required(true))]
    pub path: PathBuf,

    #[clap(short, long, required(true))]
    pub url: String,

    #[clap(flatten)]
    pub common: CommonArgs,
}

impl HorizonPushReceive {
    pub async fn eval(self, sender: mpsc::Sender<bool>) -> Result<bool, AppError> {
        let Self { path, url, common } = self;

        let secret = get_or_create_secret()?;
        // TODO: use a custom relay if needed
        let relay_mode = RelayMode::Default;
        let mut builder = Endpoint::builder()
            .alpns(vec![])
            .secret_key(secret)
            .relay_mode(relay_mode);

        if let Some(addr) = common.magic_ipv4_addr {
            builder = builder.bind_addr_v4(addr);
        }
        if let Some(addr) = common.magic_ipv6_addr {
            builder = builder.bind_addr_v6(addr);
        }

        println!("getting started:");
        let endpoint = builder
            .bind()
            .await
            .map_err(|err| AppError::IrohEndpointError(err.to_string()))?;

        // bind ipv4
        // TODO: add ipv6 support
        // TODO: extract and use a custom discovey node
        // let endpoint = Endpoint::builder()
        //     .discovery_n0()
        //     .bind()
        //     .await
        //     .map_err(|e| AppError::IrohEndpointError(e.to_string()))?;

        // We initialize the Blobs protocol in-memory
        // let local_pool = LocalPool::default();
        // let blobs = Blobs::memory().build(&local_pool, &endpoint);

        // Now we build a router that accepts blobs connections & routes them
        // to the blobs protocol.
        // let router = Router::builder(endpoint.clone())
        //     .accept(iroh_blobs::ALPN, blobs.clone())
        //     .spawn()
        //     .await
        //     .map_err(|e| AppError::IrohRouterError(e.to_string()))?;

        println!("Indexing file.");

        // let blobs = blobs.client();

        let ticket = BlobTicket::from_str(url.as_str())
            .map_err(|e| AppError::IrohBlobTicketReadError(e.to_string()))?;
        let addr = ticket.node_addr().clone();

        println!("Starting download.");
        let dir_name = format!(".horizon-downloads-{}", ticket.hash().to_hex());
        let iroh_data_dir = std::env::current_dir()
            .map_err(|err| AppError::StdIOError(err.to_string()))?
            .join(dir_name);
        println!("Current dir:: {:?}", iroh_data_dir.clone());
        let db = iroh_blobs::store::fs::Store::load(&iroh_data_dir)
            .await
            .map_err(|err| AppError::IrohBlobStoreError(err.to_string()))?;

        let mp = MultiProgress::new();
        let connect_progress = mp.add(ProgressBar::hidden());
        connect_progress.set_draw_target(ProgressDrawTarget::stderr());
        connect_progress.set_style(ProgressStyle::default_spinner());
        connect_progress.set_message(format!(
            "connecting to {}",
            ticket.clone().node_addr().node_id
        ));

        let connection = endpoint
            .connect(addr, iroh_blobs::protocol::ALPN)
            .await
            .map_err(|err| AppError::IrohEndpointConnectionError(err.to_string()))?;
        let hash_and_format = HashAndFormat {
            hash: ticket.hash(),
            format: ticket.format(),
        };

        let (send, recv) = async_channel::bounded(32);
        let progress = iroh_blobs::util::progress::AsyncChannelProgressSender::new(send);

        // TODO: add error handling
        let (_hash_seq, sizes) =
            get_hash_seq_and_sizes(&connection, &hash_and_format.hash, 1024 * 1024 * 32)
                .await
                .map_err(|err| {
                    println!("{}", err.to_string());
                    return err;
                })
                .unwrap();
        // .map_err(show_get_error)?;
        let total_size = sizes.iter().sum::<u64>();
        let total_files = sizes.len().saturating_sub(1);
        let payload_size = sizes.iter().skip(1).sum::<u64>();
        println!(
            "\n getting collection {} {} files \n, {}",
            &ticket.hash().to_string(),
            total_files,
            HumanBytes(payload_size)
        );

        // TODO
        // let total_size = 100;
        let _task = tokio::spawn(show_download_progress(recv, total_size));
        let get_conn = || async move { Ok(connection) };
        // TODO: add error handling
        let _stats = iroh_blobs::get::db::get_to_db(&db, get_conn, &hash_and_format, progress)
            .await
            .unwrap();

        let collection = Collection::load_db(&db, &hash_and_format.hash)
            .await
            .map_err(|err| AppError::IrohBlobCollectionLoadError(err.to_string()))?;
        if common.verbose > 0 {
            for (name, hash) in collection.iter() {
                println!("    {} {name}", hash.to_string());
            }
        }
        if let Some((name, _)) = collection.iter().next() {
            if let Some(first) = name.split('/').next() {
                println!("downloading to: {};", first);
            }
        }

        // .map_err(|e| show_get_error(anyhow::anyhow!(e)))?;
        // let collection = Collection::load_db(&db, &hash_and_format.hash)
        //     .await
        //     .map_err(|err| AppError::IrohBlobCollectionLoadError(err.to_string()))?;

        // println!("collection:: {:?}", collection);

        // connect_progress.finish_and_clear();
        // let progress = blobs
        //     .download(ticket.hash(), ticket.node_addr().clone())
        //     .await
        //     .map_err(|e| AppError::IrohBlobDownloadError(e.to_string()))?;

        // progress
        //     .finish()
        //     .await
        //     .map_err(|e| AppError::IrohBlobFinishError(e.to_string()))?;

        // iroh_blobs::get::db::get_to_db(, , , )

        println!("Finished download.");

        println!("Copying to destination.");

        // let mut file = tokio::fs::File::create(path).await.unwrap();
        // let mut reader = blobs
        //     .read_at(ticket.hash(), 0, ReadAtLen::All)
        //     .await
        //     .map_err(|e| AppError::IrohBlobReadError(e.to_string()))?;
        // tokio::io::copy(&mut reader, &mut file).await.unwrap();

        println!("Finished copying.");

        export(path, db, collection).await?;
        // tokio::fs::remove_dir_all(iroh_data_dir)
        //     .await
        //     .map_err(|err| AppError::FsError(err.to_string()))?;
        // if common.verbose > 0 {
        //     println!(
        //         "downloaded {} files, {}. took {} ({}/s)",
        //         total_files,
        //         HumanBytes(payload_size),
        //         HumanDuration(stats.elapsed),
        //         HumanBytes((stats.bytes_read as f64 / stats.elapsed.as_secs_f64()) as u64),
        //     );
        // }

        sender.send(true).await.unwrap();
        println!("FINISH \n \n\n");

        // tokio::signal::ctrl_c()
        //     .await
        //     .map_err(|e| AppError::IOSignalError(e.to_string()))?;

        Ok(true)
    }
}

fn make_download_progress() -> ProgressBar {
    let pb = ProgressBar::hidden();
    pb.enable_steady_tick(std::time::Duration::from_millis(100));
    pb.set_style(
        ProgressStyle::with_template(
            "{msg}{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} {binary_bytes_per_sec}",
        )
        .unwrap()
        .progress_chars("#>-"),
    );
    pb
}

async fn show_download_progress(
    receiver: async_channel::Receiver<DownloadProgress>,
    total_size: u64,
) -> Result<(), AppError> {
    let mp = MultiProgress::new();
    mp.set_draw_target(ProgressDrawTarget::stderr());
    let op = mp.add(make_download_progress());
    op.set_message(format!("{} Connecting ...\n", style("[1/3]").bold().dim()));
    let mut total_done = 0;
    let mut sizes = BTreeMap::new();
    loop {
        let x = receiver.recv().await;
        match x {
            Ok(DownloadProgress::Connected) => {
                op.set_message(format!("{} Requesting ...\n", style("[2/3]").bold().dim()));
            }
            Ok(DownloadProgress::FoundHashSeq { children, .. }) => {
                op.set_message(format!(
                    "{} Downloading {} blob(s)\n",
                    style("[3/3]").bold().dim(),
                    children + 1,
                ));
                op.set_length(total_size);
                op.reset();
            }
            Ok(DownloadProgress::Found { id, size, .. }) => {
                sizes.insert(id, size);
            }
            Ok(DownloadProgress::Progress { offset, .. }) => {
                op.set_position(total_done + offset);
            }
            Ok(DownloadProgress::Done { id }) => {
                total_done += sizes.remove(&id).unwrap_or_default();
            }
            Ok(DownloadProgress::AllDone(stats)) => {
                op.finish_and_clear();
                eprintln!(
                    "Transferred {} in {}, {}/s",
                    HumanBytes(stats.bytes_read),
                    HumanDuration(stats.elapsed),
                    HumanBytes((stats.bytes_read as f64 / stats.elapsed.as_secs_f64()) as u64)
                );
                break;
            }
            Ok(DownloadProgress::Abort(e)) => {
                return Err(AppError::UserAbortedError(e.to_string()))
            }
            Err(_e) => return Err(AppError::FailedToReadProgressError),
            _ => {}
        }
    }
    Ok(())
}
