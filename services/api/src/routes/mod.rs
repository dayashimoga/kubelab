pub mod ai_tutor;
pub mod assessment;
pub mod auth;
pub mod labs;
pub mod learning;
pub mod progress;
pub mod terminal_ws;

use crate::state::AppState;
use axum::{routing::get, Json, Router};
use serde_json::json;

pub fn create_routes(state: AppState) -> Router {
    let api_v1 = Router::new()
        .nest("/auth", auth::router())
        .nest("/", learning::router())
        .nest("/", assessment::router())
        .nest("/", labs::router())
        .nest("/", progress::router())
        .nest("/", ai_tutor::router())
        .nest("/", terminal_ws::router());

    Router::new()
        .route("/healthz", get(|| async { Json(json!({"status": "healthy", "service": "kubelab-api"})) }))
        .route("/readyz", get(|| async { Json(json!({"status": "ready", "postgres": "ok", "redis": "ok", "nats": "ok"})) }))
        .nest("/v1", api_v1)
        .with_state(state)
}
