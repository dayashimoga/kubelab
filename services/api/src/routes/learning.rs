use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/tracks", get(list_tracks))
        .route("/tracks/:slug", get(get_track))
        .route("/lessons/:id", get(get_lesson))
}

async fn list_tracks(State(state): State<AppState>) -> impl IntoResponse {
    let tracks = state.learning.list_tracks().await;
    Json(tracks)
}

async fn get_track(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    match state.learning.get_track_by_slug(&slug).await {
        Ok(track) => Ok(Json(track)),
        Err(e) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )),
    }
}

async fn get_lesson(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    match state.learning.get_lesson_by_id(&id).await {
        Ok(lesson) => Ok(Json(lesson)),
        Err(e) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )),
    }
}
