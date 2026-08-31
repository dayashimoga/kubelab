use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use kubelab_notification::models::NotificationSeverity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct DispatchNotificationDto {
    pub title: String,
    pub message: String,
    pub severity: Option<NotificationSeverity>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/notifications/:user_id", get(get_notifications))
        .route(
            "/notifications/:user_id/dispatch",
            post(dispatch_notification),
        )
}

async fn get_notifications(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    let notifications = state.notification.get_user_notifications(&user_id).await;
    Json(notifications)
}

async fn dispatch_notification(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    Json(payload): Json<DispatchNotificationDto>,
) -> impl IntoResponse {
    let severity = payload.severity.unwrap_or(NotificationSeverity::Info);
    let notification = state
        .notification
        .dispatch(&user_id, &payload.title, &payload.message, severity)
        .await;
    (StatusCode::CREATED, Json(notification))
}
