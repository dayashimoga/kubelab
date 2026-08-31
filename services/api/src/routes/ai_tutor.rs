use crate::state::AppState;
use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use kubelab_ai_tutor::models::TutorRequest;

pub fn router() -> Router<AppState> {
    Router::new().route("/ai-tutor/query", post(query_tutor))
}

async fn query_tutor(
    State(state): State<AppState>,
    Json(payload): Json<TutorRequest>,
) -> impl IntoResponse {
    let res = state.ai_tutor.query_tutor(&payload).await;
    Json(res)
}
