use axum::{
    body::Body,
    extract::State,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use horizon_core::{
    errors::AppError,
    s3::iroh_impl::{HorizonS3BucketTicket, HorizonS3System, SharePermission},
};

use http::Request;

use hyper::Response as HyperResponse;

use s3s::auth::SimpleAuth;
use s3s::{auth::SecretKey, service::S3ServiceBuilder};

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

// use tower::Service;
use tower_http::trace::TraceLayer;

// Re-export key types
pub use horizon_core as core;

#[derive(Error, Debug)]
pub enum ApiError {}

// Simple shared state between handlers
#[derive(Clone)]
pub struct ApiState {
    pub s3: HorizonS3System,
    pub domain_name: String,
    pub access_key: Option<String>,
    pub secret_key: Option<String>,
}

pub struct S3ServiceResponse(HyperResponse<Body>);

impl IntoResponse for S3ServiceResponse {
    fn into_response(self) -> Response {
        self.0.into_response() // Converting Hyper Response to Axum Response
    }
}

fn convert_response(resp: Response<s3s::Body>) -> http::Response<axum::body::Body> {
    let (parts, body) = resp.into_parts();
    let body = axum::body::Body::new(body);
    http::Response::from_parts(parts, body)
}

#[derive(Debug, Clone)]
pub struct Extra {
    pub credentials: Option<s3s::auth::Credentials>,
    pub region: Option<String>,
    pub service: Option<String>,
}

fn convert_request(req: http::Request<Body>) -> Request<s3s::Body> {
    let (parts, body) = req.into_parts();
    let r: s3s::Body = s3s::Body::http_body_unsync(body);
    Request::from_parts(parts, r)
}

pub async fn run_api_server(state: ApiState, addr: &str) -> Result<(), AppError> {
    // Setup S3 service
    let service = {
        let mut b = S3ServiceBuilder::new(state.s3.clone());

        // b.set_host(SingleDomain::new(&state.domain_name).unwrap());
        if let (Some(ak), Some(sk)) = (state.access_key.clone(), state.secret_key.clone()) {
            let sk = SecretKey::from(sk);
            b.set_auth(SimpleAuth::from_single(ak, sk));
            println!("authentication is enabled");
        }

        b.build()
    };

    let s3_service = Arc::new(service);

    let wrapped_s3_service = tower::service_fn(move |req| {
        let svc = s3_service.clone();
        async move {
            let req = convert_request(req);
            let response = svc.call(req).await.unwrap();
            Ok(convert_response(response))
        }
    });

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/share-bucket", post(share_bucket_handler))
        .route("/import-bucket", post(import_bucket_handler))
        .nest_service("/s3", wrapped_s3_service)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    println!("API server listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .map_err(|err| AppError::AppServiceError(err.to_string()))?;
    Ok(())
    // }
}

// Handlers
async fn health_check() -> impl IntoResponse {
    Json(HealthResponse {
        status: "ok".into(),
    })
}

async fn share_bucket_handler(
    State(state): State<ApiState>,
    Json(request): Json<ShareBucketRequest>,
) -> impl IntoResponse {
    let ShareBucketRequest {
        bucket_name,
        permission,
    } = request.into();
    let ticket = state
        .s3
        .share_bucket(bucket_name, permission)
        .await
        .unwrap();
    Json(ShareBucketResponse { ticket })
}

async fn import_bucket_handler(
    State(state): State<ApiState>,
    Json(request): Json<ImportBucketRequest>,
) -> impl IntoResponse {
    let ImportBucketRequest { ticket } = request;
    let _ = state.s3.import_bucket(ticket).await;
    Json(ImportBucketResponse {})
}

#[derive(Debug, Deserialize)]
struct ShareBucketRequest {
    bucket_name: String,
    permission: SharePermission,
}

#[derive(Debug, Serialize)]
struct ShareBucketResponse {
    ticket: HorizonS3BucketTicket,
}

#[derive(Debug, Deserialize)]
struct ImportBucketRequest {
    ticket: HorizonS3BucketTicket,
}

#[derive(Debug, Serialize)]
struct ImportBucketResponse {}
// Basic request/response types
#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
}
