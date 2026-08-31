use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use kubelab_assessment::models::QuizSubmission;
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/lessons/:lesson_id/quiz", get(get_quiz))
        .route("/quizzes/submit", post(submit_quiz))
}

async fn get_quiz(
    State(state): State<AppState>,
    Path(lesson_id): Path<String>,
) -> impl IntoResponse {
    let questions = state.assessment.get_quiz_by_lesson_id(&lesson_id).await;
    Json(questions)
}

async fn submit_quiz(
    State(state): State<AppState>,
    Json(payload): Json<QuizSubmission>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    match state.assessment.evaluate_submission(&payload).await {
        Ok(result) => Ok(Json(result)),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )),
    }
}
