use clap::Parser;

use iroh::{RelayMap, RelayMode, RelayUrl};
use iroh_base::SecretKey;
use iroh_blobs::{
    format::collection::Collection,
    net_protocol::Blobs,
    store::{fs::Store, ExportMode},
};
use std::{
    fmt::{Display, Formatter},
    net::{SocketAddrV4, SocketAddrV6},
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
};
use tokio::sync::mpsc;

use anyhow::Result;
use iroh::protocol::Router;
use iroh_blobs::util::local_pool::LocalPool;
use quic_rpc::transport::flume::FlumeConnector;

pub type BlobsClient = iroh_blobs::rpc::client::blobs::Client<
    FlumeConnector<iroh_blobs::rpc::proto::Response, iroh_blobs::rpc::proto::Request>,
>;
pub type DocsClient = iroh_docs::rpc::client::docs::Client<
    FlumeConnector<iroh_docs::rpc::proto::Response, iroh_docs::rpc::proto::Request>,
>;
use crate::{errors::AppError, event::HorizonChannel};

use super::client_status::IrohClientStatus;

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
            let rand = rand::rngs::OsRng;
            let key = SecretKey::generate(rand);
            eprintln!("using secret key {}", key);
            Ok(key)
        }
    }
}

#[derive(Clone, Debug)]
pub struct IrohState {
    _local_pool: Arc<LocalPool>,
    pub router: Router,
    pub blobs: BlobsClient,
    pub docs: DocsClient,
    pub blobs_store: Blobs<Store>,
}

impl IrohState {
    pub async fn new(
        path: PathBuf,
        sender: mpsc::Sender<HorizonChannel>,
    ) -> Result<Self, AppError> {
        // create dir if it doesn't already exist
        tokio::fs::create_dir_all(&path)
            .await
            .map_err(|e| AppError::FsError(e.to_string()))?;

        let key = iroh_blobs::util::fs::load_secret_key(path.clone().join("keypair"))
            .await
            .map_err(|err| AppError::IrohSecretKeyError(err.to_string()))?;

        // local thread pool manager for blobs
        let local_pool = LocalPool::default();

        // create endpoint
        let endpoint = iroh::Endpoint::builder()
            .discovery_n0()
            .secret_key(key)
            .bind()
            .await
            .map_err(|e| AppError::IrohEndpointError(e.to_string()))?;

        // build the protocol router
        let mut builder = iroh::protocol::Router::builder(endpoint);

        // add iroh gossip
        let gossip = iroh_gossip::net::Gossip::builder()
            .spawn(builder.endpoint().clone())
            .await
            .map_err(|err| AppError::IrohGossipError(err.to_string()))?;
        builder = builder.accept(iroh_gossip::ALPN, Arc::new(gossip.clone()));

        let client_status = IrohClientStatus { sender };
        // add iroh blobs
        let blobs = iroh_blobs::net_protocol::Blobs::persistent(&path)
            .await
            .map_err(|err| AppError::IrohBlobStoreError(err.to_string()))?
            .events(client_status.into())
            .build(&local_pool.handle(), builder.endpoint());

        builder = builder.accept(iroh_blobs::ALPN, blobs.clone());

        // add docs
        let docs = iroh_docs::protocol::Docs::persistent(path)
            .spawn(&blobs, &gossip)
            .await
            .map_err(|err| AppError::IrohDocsError(err.to_string()))?;
        builder = builder.accept(iroh_docs::ALPN, Arc::new(docs.clone()));

        let router = builder
            .spawn()
            .await
            .map_err(|e| AppError::IrohRouterError(e.to_string()))?;

        println!("router;: {:?}", router.endpoint().direct_addresses().get());
        // Err(router)

        let blobs_client = blobs.client().clone();
        let docs_client = docs.client().clone();

        Ok(Self {
            _local_pool: Arc::new(local_pool),
            router,
            blobs: blobs_client,
            blobs_store: blobs,
            docs: docs_client,
        })
    }

    pub(crate) async fn shutdown(self) -> Result<()> {
        self.router.shutdown().await
    }
}
