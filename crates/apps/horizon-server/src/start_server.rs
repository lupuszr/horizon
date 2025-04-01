use std::path::PathBuf;

use clap::Args;
use horizon_api::{run_api_server, ApiState};
use horizon_core::{
    errors::AppError, event::HorizonChannel, iroh::common::IrohState,
    s3::iroh_impl::HorizonS3System,
};
use tokio::sync::mpsc;

#[derive(Debug, Args)]
pub struct HorizonServerStart {
    #[clap(short, long, default_value = None)]
    pub base_path: Option<PathBuf>,

    #[clap(short, long, default_value = None)]
    pub access_key: Option<String>,

    #[clap(short, long, default_value = None)]
    pub secret_key: Option<String>,

    #[clap(short, long, default_value = "http://0.0.0.0:3000")]
    pub domain_name: url::Url,
}

impl HorizonServerStart {
    pub async fn eval(self, sender: mpsc::Sender<HorizonChannel>) -> Result<bool, AppError> {
        let Self {
            base_path,
            access_key,
            secret_key,
            domain_name,
            ..
        } = self;

        let iroh_base_path = if base_path.is_none() {
            let mut ph = dirs_next::home_dir().ok_or(AppError::InternalStateError(
                "Could not determine home folder".to_string(),
            ))?;
            ph.push(".horizon-server");
            ph
        } else {
            base_path.unwrap()
        };

        let iroh_state = IrohState::new(iroh_base_path.clone(), sender.clone())
            .await
            .map_err(|err| AppError::IrohHorizonStateSetupError(err.to_string()))?;

        let address = format!(
            "{}:{}",
            domain_name.host_str().unwrap_or("0.0.0.0"),
            domain_name.port_or_known_default().unwrap_or(3000)
        );
        let s3 = HorizonS3System::new_with_iroh(iroh_state);
        let api_state = ApiState {
            s3,
            domain_name: address.clone(),
            access_key,
            secret_key,
        };

        run_api_server(api_state, &address).await?;

        tokio::signal::ctrl_c()
            .await
            .map_err(|e| AppError::IOSignalError(e.to_string()))?;

        Ok(true)
    }
}
