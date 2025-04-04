use std::sync::PoisonError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Iroh Endpoint error: {0}")]
    IrohEndpointError(String),

    #[error("Iroh Relay url error: {0}")]
    IrohRelayUrlError(String),

    #[error("Iroh Router error: {0}")]
    IrohRouterError(String),

    #[error("Iroh Stream error: {0}")]
    IrohStreamError(String),

    #[error("Iroh Download Progress error: {0}")]
    IrohDownloadProgressError(String),

    #[error("Iroh Blob path error: {0}")]
    IrohBlobPathError(String),

    #[error("Iroh Blob finish error: {0}")]
    IrohBlobFinishError(String),

    #[error("Iroh Blob collection load error: {0}")]
    IrohBlobCollectionLoadError(String),

    #[error("Iroh Blob store error: {0}")]
    IrohBlobStoreError(String),

    #[error("Iroh Blob ticket creation error: {0}")]
    IrohBlobTicketCreationError(String),

    #[error("Iroh Blob ticket read error: {0}")]
    IrohBlobTicketReadError(String),

    #[error("Iroh Blob download error: {0}")]
    IrohBlobDownloadError(String),

    #[error("Iroh Blob export error: {0}")]
    IrohBlobExportError(String),

    #[error("Iroh Blob export progress error: {0}")]
    IrohBlobExportProgressError(String),

    #[error("Iroh Blob read error: {0}")]
    IrohBlobReadError(String),

    #[error("Iroh Docs error: {0}")]
    IrohDocsError(String),

    #[error("Iroh gossip error: {0}")]
    IrohGossipError(String),

    #[error("Iroh secret key error: {0}")]
    IrohSecretKeyError(String),

    #[error("Iroh connection error: {0}")]
    IrohEndpointConnectionError(String),

    #[error("Iroh Horizon State setup error: {0}")]
    IrohHorizonStateSetupError(String),

    #[error("IO signal error: {0}")]
    IOSignalError(String),

    #[error("Path error: {0}")]
    PathError(String),

    #[error("fs error: {0}")]
    FsError(String),

    #[error("Std IO error: {0}")]
    StdIOError(String),

    #[error("Aborted by user: {0}")]
    UserAbortedError(String),

    #[error("Failed to read progress")]
    FailedToReadProgressError,

    #[error("UI Error: {0}")]
    UIError(String),

    #[error("JoinHandle error: {0}")]
    JoinHandleError(String),

    #[error("Internal state error: {0}")]
    InternalStateError(String),

    #[error("Internal channel error: {0}")]
    InternalChannelError(String),

    #[error("Lock poisoned: {0}")]
    LockError(String),

    #[error("No such bucket")]
    S3NoBucket,

    #[error("Bucket already imported")]
    S3BucketExists,

    #[error("S3 metadata miss")]
    S3MetadataMiss,

    #[error("App service error: {0}")]
    AppServiceError(String),
}

impl<T> From<PoisonError<T>> for AppError {
    fn from(err: PoisonError<T>) -> Self {
        AppError::LockError(err.to_string())
    }
}
