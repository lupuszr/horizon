use iroh::Endpoint;

use crate::errors::AppError;

pub async fn initialize_iroh_endpoint() -> Result<Endpoint, AppError> {
    let endpoint = Endpoint::builder()
        .discovery_n0()
        .discovery_dht()
        .discovery_local_network()
        .bind()
        .await
        .map_err(|e| AppError::IrohEndpointError(e.to_string()))?;

    Ok(endpoint)
}
