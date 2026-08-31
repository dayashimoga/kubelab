use crate::state::AppState;
use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/progress", get(get_user_progress))
        .route("/skills/graph", get(get_skill_graph))
}

async fn get_user_progress(State(state): State<AppState>) -> impl IntoResponse {
    let user_id = "test-learner-1";
    let progress = state.progress.get_user_progress(user_id).await;
    Json(progress)
}

async fn get_skill_graph(State(state): State<AppState>) -> impl IntoResponse {
    let graph = state.progress.get_skill_graph();
    Json(graph)
}
