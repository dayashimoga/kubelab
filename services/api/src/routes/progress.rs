use crate::state::AppState;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/progress", get(get_default_progress))
        .route("/progress/:user_id", get(get_user_progress))
        .route("/skills/graph", get(get_skill_graph))
}

async fn get_default_progress(State(state): State<AppState>) -> impl IntoResponse {
    let user_id = "test-learner-1";
    let progress = state.progress.get_user_progress(user_id).await;
    Json(progress)
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
