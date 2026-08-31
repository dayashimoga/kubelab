use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use kubelab_labs::models::{StartLabRequest, ValidateLabRequest};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/labs", get(list_labs))
        .route("/labs/:id", get(get_lab))
        .route("/labs/start", post(start_lab))
        .route("/labs/sessions/:session_id", get(get_session))
        .route("/labs/sessions/:session_id/validate", post(validate_task))
}

async fn list_labs(State(state): State<AppState>) -> impl IntoResponse {
    let labs = state.labs.list_labs().await;
    Json(labs)
}

async fn get_lab(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    match state.labs.get_lab(&id).await {
        Ok(lab) => Ok(Json(lab)),
        Err(e) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )),
    }
}

async fn start_lab(
    State(state): State<AppState>,
    Json(payload): Json<StartLabRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    // In production, user_id is extracted from verified auth JWT
    let user_id = "test-learner-1";
    match state.labs.start_session(user_id, payload).await {
        Ok(session) => Ok((StatusCode::CREATED, Json(session))),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )),
    }
}

async fn get_session(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    match state.labs.get_session(&session_id).await {
        Ok(session) => Ok(Json(session)),
        Err(e) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )),
    }
}

async fn validate_task(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
    Json(payload): Json<ValidateLabRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    match state.labs.validate_task(&session_id, payload).await {
        Ok(result) => Ok(Json(result)),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )),
    }
}
