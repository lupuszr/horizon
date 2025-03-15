use horizon_core::{
    errors::AppError,
    event::HorizonChannel,
    iroh::{
        common::{CommonArgs, IrohState},
        send::index_and_expose,
    },
};
use std::{collections::HashMap, fmt};
use std::{env, path::PathBuf};
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::{mpsc, Mutex};

#[derive(Debug, Clone)]
pub struct OperationInfo {
    pub path: PathBuf,
    pub ticket: Option<String>, // Stores ticket after generation
    pub verbose: u8,
    pub request_id: Option<u64>,
    pub connection_id: Option<u64>,
}

impl std::fmt::Display for OperationInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "OperationInfo {{ path: {}, ticket: {}, verbose: {} }}",
            self.path.display(),
            self.ticket.as_deref().unwrap_or("None"),
            self.verbose
        )
    }
}

#[derive(Debug)]
pub struct HorizonState {
    pub operations: Mutex<HashMap<String, OperationInfo>>, // Map operation IDs to details
    pub sender: mpsc::Sender<HorizonChannel>,
    pub iroh: IrohState, // Channel for ticket notifications
}

async fn setup(app: AppHandle, tx: mpsc::Sender<HorizonChannel>) -> Result<(), AppError> {
    let mut path = dirs_next::home_dir().ok_or(AppError::InternalStateError(
        "Could not determine home folder".to_string(),
    ))?;

    path.push(".horizon-push");
    let iroh = IrohState::new(path, tx.clone()).await.unwrap();
    let horizon_state = HorizonState::new(iroh, tx);
    let horizon_state = horizon_state;
    app.manage(horizon_state);

    Ok(())
}

impl HorizonState {
    pub fn new(iroh: IrohState, sender: mpsc::Sender<HorizonChannel>) -> Self {
        HorizonState {
            operations: Mutex::new(HashMap::new()),
            sender,
            iroh,
        }
    }

    pub async fn add_operation(&self, id: String, info: OperationInfo) {
        let mut operations = self.operations.lock().await;
        operations.insert(id, info);
    }

    pub async fn update_ticket(&self, id: &str, ticket: String) -> Result<(), String> {
        let mut operations = self.operations.lock().await;
        if let Some(info) = operations.get_mut(id) {
            info.ticket = Some(ticket);
            Ok(())
        } else {
            Err("Operation not found".to_string())
        }
    }

    pub async fn update_connection(
        &self,
        id: &str,
        connection_id: u64,
        request_id: u64,
    ) -> Result<(), String> {
        let mut operations = self.operations.lock().await;
        if let Some(info) = operations.get_mut(id) {
            info.connection_id = Some(connection_id);
            info.request_id = Some(request_id);
            Ok(())
        } else {
            Err("Operation not found".to_string())
        }
    }

    pub async fn get_operation(&self, id: &str) -> Option<OperationInfo> {
        let operations = self.operations.lock().await;
        operations.get(id).cloned()
    }

    pub async fn get_all_operation(&self) -> Vec<OperationInfo> {
        let operations = self.operations.lock().await;
        let mut values: Vec<OperationInfo> = vec![];
        for v in operations.values() {
            values.push(v.clone());
        }

        values
    }
}

#[derive(Debug)]
pub struct HorizonPushSend {
    pub path: PathBuf,

    pub common: CommonArgs,
}

impl HorizonPushSend {
    pub async fn eval_with_state(
        self,
        state: &HorizonState,
        operation_id: String,
    ) -> Result<bool, AppError> {
        let Self { path, .. } = self;

        let ticket =
            index_and_expose(state.iroh.clone(), path.clone(), state.sender.clone()).await?;
        println!("TICKET:: {:?}", ticket.clone());

        // Update state with the generated ticket
        state
            .update_ticket(&operation_id, ticket.clone().to_string())
            .await
            .map_err(|err| AppError::InternalStateError(err))?;

        println!(
            "horizon-cli receive --url {ticket} --path {}",
            path.to_str().unwrap()
        );

        // TODO; solve this
        tokio::signal::ctrl_c()
            .await
            .map_err(|e| AppError::IOSignalError(e.to_string()))?;
        Ok(true)
    }
}

#[tauri::command]
async fn create_operation(
    state: tauri::State<'_, HorizonState>,
    path: String,
    verbose: u8,
) -> Result<String, String> {
    let operation_id = uuid::Uuid::new_v4().to_string();
    let operation_info = OperationInfo {
        path: PathBuf::from(path),
        ticket: None,
        verbose,
        request_id: None,
        connection_id: None,
    };

    state
        .add_operation(operation_id.clone(), operation_info)
        .await;

    println!(
        "operation:: {:?}",
        state
            .get_operation(operation_id.clone().as_str())
            .await
            .unwrap()
    );
    Ok(operation_id)
}

#[tauri::command(async)]
async fn push_send(
    state: tauri::State<'_, HorizonState>,
    operation_id: String,
) -> Result<bool, String> {
    if let Some(info) = state.get_operation(&operation_id).await {
        let horizon_push_send = HorizonPushSend {
            path: info.path.clone(),
            common: CommonArgs {
                verbose: info.verbose,
                magic_ipv4_addr: None,
                magic_ipv6_addr: None,
                relay: horizon_core::iroh::common::RelayModeOption::Default,
            },
        };

        let a = horizon_push_send
            .eval_with_state(state.inner(), operation_id)
            .await
            .map_err(|err| err.to_string());
        println!("a:: {:?}", a.clone());
        a
    } else {
        Err("Operation ID not found".to_string())
    }
}

// #[tauri::command(async)]
// async fn pull_blob(state: tauri::State<'_, HorizonState>, ticket: String) -> Result<bool, String> {
//     todo!()
// }

// #[tauri::command(async)]
// async fn push_send(
//     state: tauri::State<'_, HorizonState>,
//     operation_id: String,
// ) -> Result<bool, String> {
//     if let Some(info) = state.get_operation(&operation_id).await {
//         let horizon_push_send = HorizonPushSend {
//             path: info.path.clone(),
//             common: CommonArgs {
//                 verbose: info.verbose,
//                 magic_ipv4_addr: None,
//                 magic_ipv6_addr: None,
//                 relay: horizon_core::iroh::common::RelayModeOption::Default,
//             },
//         };

//         let a = horizon_push_send
//             .eval_with_state(state.inner(), operation_id)
//             .await
//             .map_err(|err| err.to_string());
//         println!("a:: {:?}", a.clone());
//         a
//     } else {
//         Err("Operation ID not found".to_string())
//     }
// }
// #[tauri::command]
// async fn get_operation_status(
//     state: tauri::State<'_, Arc<HorizonState>>,
//     operation_id: String,
// ) -> Result<Option<OperationInfo>, String> {
//     Ok(state.get_operation(&operation_id).await)
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (tx, mut rx) = mpsc::channel(100);

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let handle = app.handle().clone();
            let handle2 = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                setup(handle, tx).await.unwrap();
            });
            tauri::async_runtime::spawn(async move {
                while let Some(ev) = rx.recv().await {
                    println!("event: {:?}", ev);
                    handle2.emit("iroh-event", ev).unwrap();
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_operation,
            push_send,
            // get_operation_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
