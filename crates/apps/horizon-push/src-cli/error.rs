use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Iroh Endpoint error: {0}")]
    IrohEndpointError(String),

    #[error("Iroh Router error: {0}")]
    IrohRouterError(String),

    #[error("Iroh Blob path error: {0}")]
    IrohBlobPathError(String),

    #[error("Iroh Blob finish error: {0}")]
    IrohBlobFinishError(String),

    #[error("Iroh Blob ticket creation error: {0}")]
    IrohBlobTicketCreationError(String),

    #[error("Iroh Blob ticket read error: {0}")]
    IrohBlobTicketReadError(String),

    #[error("Iroh Blob download error: {0}")]
    IrohBlobDownloadError(String),

    #[error("Iroh Blob read error: {0}")]
    IrohBlobReadError(String),

    #[error("IO signal error: {0}")]
    IOSignalError(String),
}
