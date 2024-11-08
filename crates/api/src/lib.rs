use axum::{
    Router,
    routing::{get, post},
    extract::State,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;
use tower_http::trace::TraceLayer;
use tracing::{info, error};

// Re-export key types
pub use hybrid_streaming_auth as auth;
pub use hybrid_streaming_storage as storage;
pub use hybrid_streaming_streaming as streaming;
pub use hybrid_streaming_video as video;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Storage error: {0}")]
    Storage(#[from] storage::StorageError),
    #[error("Auth error: {0}")]
    Auth(#[from] auth::AuthError),
    #[error("Streaming error: {0}")]
    Streaming(#[from] streaming::StreamingError),
    #[error("Video error: {0}")]
    Video(#[from] video::VideoError),
}

// Simple shared state between handlers
#[derive(Clone)]
pub struct ApiState {
    pub storage: Arc<dyn storage::Storage>,
    pub streaming: Arc<dyn streaming::Streaming>,
    pub auth: Arc<dyn auth::Auth>,
}

pub async fn run_api_server(
    state: ApiState,
    addr: &str,
) -> Result<(), ApiError> {
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/upload", post(upload_handler))
        .route("/stream/:id", get(stream_handler))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    info!("API server listening on {}", addr);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

// Handlers
async fn health_check() -> impl IntoResponse {
    Json(HealthResponse { status: "ok".into() })
}

async fn upload_handler(
    State(state): State<ApiState>,
    Json(request): Json<UploadRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let content_id = state.storage.store(request.into()).await?;
    Ok(Json(UploadResponse { content_id }))
}

async fn stream_handler(
    State(state): State<ApiState>,
    path: axum::extract::Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let stream = state.streaming.stream(&path.0).await?;
    Ok(stream)
}

// Basic request/response types
#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
}

#[derive(Debug, Deserialize)]
struct UploadRequest {
    content: Vec<u8>,
}

#[derive(Debug, Serialize)]
struct UploadResponse {
    content_id: String,
}
