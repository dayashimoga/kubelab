use crate::routes::auth::AuthClaims;
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use kubelab_progress::models::ProgressSyncRequest;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/progress", get(get_default_progress))
        .route("/progress/sync", post(sync_default_progress))
        .route("/progress/:user_id", get(get_user_progress))
        .route("/progress/:user_id/sync", post(sync_user_progress))
        .route("/skills/graph", get(get_skill_graph))
}

async fn get_default_progress(State(state): State<AppState>) -> impl IntoResponse {
    let user_id = "test-learner-1";
    let progress = state.progress.get_user_progress(user_id).await;
    Json(progress)
}

async fn sync_default_progress(
    State(state): State<AppState>,
    auth: Option<AuthClaims>,
    Json(payload): Json<ProgressSyncRequest>,
) -> impl IntoResponse {
    let user_id = match auth {
        Some(AuthClaims(user)) => user.id.to_string(),
        None => "test-learner-1".to_string(),
    };
    let response = state.progress.sync_progress(&user_id, payload).await;
    Json(response)
}

async fn sync_user_progress(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    Json(payload): Json<ProgressSyncRequest>,
) -> impl IntoResponse {
    let response = state.progress.sync_progress(&user_id, payload).await;
    Json(response)
}

async fn get_user_progress(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    let progress = state.progress.get_user_progress(&user_id).await;
    Json(progress)
}

async fn get_skill_graph(State(state): State<AppState>) -> impl IntoResponse {
    let graph = state.progress.get_skill_graph();
    Json(graph)
}
