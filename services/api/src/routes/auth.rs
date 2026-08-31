use crate::state::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RegisterDto {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterDto>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    match state
        .auth
        .register(&payload.email, &payload.name, &payload.password, None)
        .await
    {
        Ok(res) => Ok((StatusCode::CREATED, Json(res))),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )),
    }
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginDto>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    match state.auth.login(&payload.email, &payload.password).await {
        Ok(res) => Ok((StatusCode::OK, Json(res))),
        Err(e) => Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )),
    }
}
