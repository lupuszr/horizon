use clap::Parser;
use console::style;
use futures_buffered::BufferedStreamExt;
use futures_lite::{future::Boxed, StreamExt};
use indicatif::{
    HumanBytes, HumanDuration, MultiProgress, ProgressBar, ProgressDrawTarget, ProgressStyle,
};

use iroh::{key::SecretKey, RelayMap, RelayMode, RelayUrl};
use iroh_blobs::{
    format::collection::Collection,
    provider::{self, CustomEventSender},
    store::{ExportMode, ImportMode, ImportProgress},
    util::fs::canonicalized_path_to_string,
    BlobFormat, TempTag,
};
use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
    net::{SocketAddrV4, SocketAddrV6},
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
    time::Duration,
};
use walkdir::WalkDir;

use crate::error::AppError;

/// Available command line options for configuring relays.
#[derive(Clone, Debug)]
pub enum RelayModeOption {
    /// Disables relays altogether.
    Disabled,
    /// Uses the default relay servers.
    Default,
    /// Uses a single, custom relay server by URL.
    Custom(RelayUrl),
}

impl FromStr for RelayModeOption {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "disabled" => Ok(Self::Disabled),
            "default" => Ok(Self::Default),
            _ => {
                Ok(Self::Custom(RelayUrl::from_str(s).map_err(|err| {
                    AppError::IrohRelayUrlError(err.to_string())
                })?))
            }
        }
    }
}

impl Display for RelayModeOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Disabled => f.write_str("disabled"),
            Self::Default => f.write_str("default"),
            Self::Custom(url) => url.fmt(f),
        }
    }
}

impl From<RelayModeOption> for RelayMode {
    fn from(value: RelayModeOption) -> Self {
        match value {
            RelayModeOption::Disabled => RelayMode::Disabled,
            RelayModeOption::Default => RelayMode::Default,
            RelayModeOption::Custom(url) => RelayMode::Custom(RelayMap::from_url(url)),
        }
    }
}

#[derive(Parser, Debug)]
pub struct CommonArgs {
    /// The IPv4 address that magicsocket will listen on.
    ///
    /// If None, defaults to a random free port, but it can be useful to specify a fixed
    /// port, e.g. to configure a firewall rule.
    #[clap(long, default_value = None)]
    pub magic_ipv4_addr: Option<SocketAddrV4>,

    /// The IPv6 address that magicsocket will listen on.
    ///
    /// If None, defaults to a random free port, but it can be useful to specify a fixed
    /// port, e.g. to configure a firewall rule.
    #[clap(long, default_value = None)]
    pub magic_ipv6_addr: Option<SocketAddrV6>,

    // #[clap(long, default_value_t = Format::Hex)]
    // pub format: Format,
    #[clap(short = 'v', long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    ///
    /// Can be set to "disable" to disable relay servers and "default"
    /// to configure default servers.
    #[clap(long, default_value_t = RelayModeOption::Default)]
    pub relay: RelayModeOption,
}

#[derive(Debug, Clone)]
pub struct SendStatus {
    /// the multiprogress bar
    mp: MultiProgress,
}

impl SendStatus {
    pub fn new() -> Self {
        let mp = MultiProgress::new();
        mp.set_draw_target(ProgressDrawTarget::stderr());
        Self { mp }
    }

    pub fn new_client(&self) -> ClientStatus {
        let current = self.mp.add(ProgressBar::hidden());
        current.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} [{elapsed_precise}] {msg}")
                .unwrap(),
        );
        current.enable_steady_tick(Duration::from_millis(100));
        current.set_message("waiting for requests");
        ClientStatus {
            current: current.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClientStatus {
    current: Arc<ProgressBar>,
}

impl Drop for ClientStatus {
    fn drop(&mut self) {
        if Arc::strong_count(&self.current) == 1 {
            self.current.finish_and_clear();
        }
    }
}

impl CustomEventSender for ClientStatus {
    fn send(&self, event: iroh_blobs::provider::Event) -> Boxed<()> {
        self.try_send(event);
        Box::pin(std::future::ready(()))
    }

    fn try_send(&self, event: provider::Event) {
        tracing::info!("{:?}", event);
        let msg = match event {
            provider::Event::ClientConnected { connection_id } => {
                Some(format!("{} got connection", connection_id))
            }
            provider::Event::TransferBlobCompleted {
                connection_id,
                hash,
                index,
                size,
                ..
            } => Some(format!(
                "{} transfer blob completed {} {} {}",
                connection_id,
                hash,
                index,
                HumanBytes(size)
            )),
            provider::Event::TransferCompleted {
                connection_id,
                stats,
                ..
            } => Some(format!(
                "{} transfer completed {} {}",
                connection_id,
                stats.send.write_bytes.size,
                HumanDuration(stats.send.write_bytes.stats.duration)
            )),
            provider::Event::TransferAborted { connection_id, .. } => {
                Some(format!("{} transfer completed", connection_id))
            }
            _ => None,
        };
        if let Some(msg) = msg {
            self.current.set_message(msg);
        }
    }
}

pub async fn show_ingest_progress(
    recv: async_channel::Receiver<ImportProgress>,
) -> Result<(), AppError> {
    let mp = MultiProgress::new();
    mp.set_draw_target(ProgressDrawTarget::stderr());
    let op = mp.add(ProgressBar::hidden());
    op.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} [{elapsed_precise}] {msg}")
            .map_err(|err| AppError::UIError(err.to_string()))?,
    );
    // op.set_message(format!("{} Ingesting ...\n", style("[1/2]").bold().dim()));
    // op.set_length(total_files);
    let mut names = BTreeMap::new();
    let mut sizes = BTreeMap::new();
    let mut pbs = BTreeMap::new();
    loop {
        let event = recv.recv().await;
        match event {
            Ok(ImportProgress::Found { id, name }) => {
                names.insert(id, name);
            }
            Ok(ImportProgress::Size { id, size }) => {
                sizes.insert(id, size);
                let total_size = sizes.values().sum::<u64>();
                op.set_message(format!(
                    "{} Ingesting {} files, {}\n",
                    style("[1/2]").bold().dim(),
                    sizes.len(),
                    HumanBytes(total_size)
                ));
                let name = names.get(&id).cloned().unwrap_or_default();
                let pb = mp.add(ProgressBar::hidden());
                pb.set_style(ProgressStyle::with_template(
                    "{msg}{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes}",
                ).map_err(|err| AppError::UIError(err.to_string()))?.progress_chars("#>-"));
                pb.set_message(format!("{} {}", style("[2/2]").bold().dim(), name));
                pb.set_length(size);
                pbs.insert(id, pb);
            }
            Ok(ImportProgress::OutboardProgress { id, offset }) => {
                if let Some(pb) = pbs.get(&id) {
                    pb.set_position(offset);
                }
            }
            Ok(ImportProgress::OutboardDone { id, .. }) => {
                // you are not guaranteed to get any OutboardProgress
                if let Some(pb) = pbs.remove(&id) {
                    pb.finish_and_clear();
                }
            }
            Ok(ImportProgress::CopyProgress { .. }) => {
                // we are not copying anything
            }
            Err(e) => {
                op.set_message(format!("Error receiving progress: {e}"));
                break;
            }
        }
    }
    op.finish_and_clear();
    Ok(())
}

pub async fn import(
    path: PathBuf,
    db: impl iroh_blobs::store::Store,
) -> Result<(TempTag, u64, Collection), AppError> {
    let path = path
        .canonicalize()
        .map_err(|err| AppError::PathError(err.to_string()))?;
    // anyhow::ensure!(path.exists(), "path {} does not exist", path.display());
    let root = path
        .parent()
        .ok_or(AppError::PathError("get parrent".to_string()))?;
    // walkdir also works for files, so we don't need to special case them
    let files = WalkDir::new(path.clone()).into_iter();
    // flatten the directory structure into a list of (name, path) pairs.
    // ignore symlinks.
    let data_sources: Vec<(String, PathBuf)> = files
        .map(|entry| {
            let entry = entry.unwrap();
            if !entry.file_type().is_file() {
                // Skip symlinks. Directories are handled by WalkDir.
                return Ok(None);
            }
            let path = entry.into_path();
            let relative = path
                .strip_prefix(root)
                .map_err(|err| AppError::PathError(err.to_string()))?;
            let name = canonicalized_path_to_string(relative, true)
                .map_err(|err| AppError::IrohBlobStoreError(err.to_string()))?;
            Ok(Some((name, path)))
        })
        .filter_map(Result::transpose)
        .collect::<Result<Vec<_>, AppError>>()?;
    let (send, recv) = async_channel::bounded(32);
    let progress = iroh_blobs::util::progress::AsyncChannelProgressSender::new(send);
    let show_progress = tokio::spawn(show_ingest_progress(recv));
    // import all the files, using num_cpus workers, return names and temp tags
    let mut names_and_tags = futures_lite::stream::iter(data_sources)
        .map(|(name, path)| {
            let db = db.clone();
            let progress = progress.clone();
            async move {
                let (temp_tag, file_size) = db
                    .import_file(path, ImportMode::TryReference, BlobFormat::Raw, progress)
                    .await
                    .map_err(|err| AppError::IrohBlobStoreError(err.to_string()))?;
                Ok((name, temp_tag, file_size))
            }
        })
        .buffered_unordered(num_cpus::get())
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<Result<Vec<_>, AppError>>()?;
    drop(progress);
    names_and_tags.sort_by(|(a, _, _), (b, _, _)| a.cmp(b));
    // total size of all files
    let size = names_and_tags.iter().map(|(_, _, size)| *size).sum::<u64>();
    // collect the (name, hash) tuples into a collection
    // we must also keep the tags around so the data does not get gced.
    let (collection, tags) = names_and_tags
        .into_iter()
        .map(|(name, tag, _)| ((name, *tag.hash()), tag))
        .unzip::<_, _, Collection, Vec<_>>();
    let temp_tag = collection
        .clone()
        .store(&db)
        .await
        .map_err(|err| AppError::IrohBlobStoreError(err.to_string()))?;
    // now that the collection is stored, we can drop the tags
    // data is protected by the collection
    drop(tags);
    show_progress
        .await
        .map_err(|err| AppError::JoinHandleError(err.to_string()))??;
    Ok((temp_tag, size, collection))
}

fn validate_path_component(_component: &str) -> Result<(), AppError> {
    // if !component.contains('/') {
    //     return Err(AppError::PathError(
    //         "path components must not contain the only correct path separator, /".to_string(),
    //     ));
    // }
    Ok(())
}

fn get_export_path(root: &Path, name: &str) -> Result<PathBuf, AppError> {
    let parts = name.split('/');
    // println!("parts:: {:?}", pars);
    let mut path = root.to_path_buf();
    for part in parts {
        println!("\n \n part:: {:?} \n\n", part);
        validate_path_component(part)?;
        path.push(part);
    }
    Ok(path)
}

pub async fn export(
    root: PathBuf,
    db: impl iroh_blobs::store::Store,
    collection: Collection,
) -> Result<(), AppError> {
    println!("root:: {:?}", root);
    println!("collection:: {:?}", collection);
    // let root = std::env::current_dir().map_err(|err| AppError::PathError(err.to_string()))?;
    for (name, hash) in collection.iter() {
        let target = get_export_path(&root, name)?;
        if target.exists() {
            eprintln!(
                "target {} already exists. Export stopped.",
                target.display()
            );
            eprintln!("You can remove the file or directory and try again. The download will not be repeated.");
            return Err(AppError::FsError(format!(
                "target {} already exists",
                target.display()
            )));
        }
        println!("Exporting to: {:?}", target.clone());
        db.export(
            *hash,
            target,
            ExportMode::TryReference,
            Box::new(move |_position| Ok(())),
        )
        .await
        .map_err(|err| AppError::IrohBlobStoreError(err.to_string()))?;
    }
    Ok(())
}

/// Get the secret key or generate a new one.
/// The secret key public key is the address of the node
/// Print the secret key to stderr if it was generated, so the user can save it.
pub fn get_or_create_secret() -> Result<SecretKey, AppError> {
    match std::env::var("IROH_SECRET") {
        Ok(secret) => SecretKey::from_str(&secret)
            .map_err(|_err| AppError::IrohSecretKeyError("Not a valid secret".to_string())),
        Err(_) => {
            let key = SecretKey::generate();
            eprintln!("using secret key {}", key);
            Ok(key)
        }
    }
}
