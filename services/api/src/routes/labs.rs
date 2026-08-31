use crate::events::publisher;
use crate::routes::auth::AuthClaims;
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use kubelab_labs::models::{ApplyManifestRequest, StartLabRequest, ValidateLabRequest};
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
        .route("/labs/sessions/:session_id/apply", post(apply_manifest))
        .route("/labs/sessions/:session_id/resources", get(get_resources))
        .route("/labs/sessions/:session_id/destroy", post(destroy_session))
        .route("/labs/sessions/:session_id", delete(destroy_session))
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
    auth: Option<AuthClaims>,
    Json(payload): Json<StartLabRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let user_id = match auth {
        Some(AuthClaims(user)) => user.id.to_string(),
        None => "default-learner".to_string(),
    };

    let lab_id = payload.lab_id.clone();
    match state.labs.start_session(&user_id, payload).await {
        Ok(session) => {
            // Publish NATS event if event bus is active
            if let (Some(ref bus), Ok(uid)) = (&state.events, Uuid::parse_str(&user_id)) {
                let _ = bus
                    .publisher()
                    .emit_lab_started(&publisher::LabStartedEvent {
                        session_id: session.id,
                        user_id: uid,
                        lab_id: lab_id.clone(),
                        namespace: session.namespace.clone(),
                        timestamp: chrono::Utc::now(),
                    })
                    .await;
            }
            // Persist session to PostgreSQL if active
            if let (Some(ref db), Ok(uid)) = (&state.db, Uuid::parse_str(&user_id)) {
                let _ = db
                    .labs()
                    .create_session(uid, &lab_id, &session.namespace, session.expires_at)
                    .await;
            }

            Ok((StatusCode::CREATED, Json(session)))
        }
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

async fn apply_manifest(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
    Json(payload): Json<ApplyManifestRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    // 1. Server-side zero-trust manifest admission validation
    if let Err(violation) = crate::admission::validate_manifest_admission(&payload.yaml_content) {
        tracing::warn!(
            "Server-side admission rejection for session {}: {}",
            session_id,
            violation
        );
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(ErrorResponse {
                error: format!("Security Admission Rejected: {}", violation),
            }),
        ));
    }

    // 2. Server-side Kubernetes apply
    match state.labs.apply_manifest(&session_id, payload).await {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )),
    }
}

async fn get_resources(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    match state.labs.get_namespace_resources(&session_id).await {
        Ok(resources) => Ok(Json(resources)),
        Err(e) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )),
    }
}

async fn destroy_session(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    match state.labs.destroy_session(&session_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
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
        Ok(result) => {
            // Update session score in PostgreSQL if active
            if let Ok(session) = state.labs.get_session(&session_id).await {
                if let Some(ref db) = state.db {
                    let status_str = if session.completed_at.is_some() {
                        "completed"
                    } else {
                        "in_progress"
                    };
                    let _ = db
                        .labs()
                        .update_score(session_id, session.score as i32, status_str)
                        .await;
                }
                if session.completed_at.is_some() {
                    if let (Some(ref bus), Ok(uid)) =
                        (&state.events, Uuid::parse_str(&session.user_id))
                    {
                        let _ = bus
                            .publisher()
                            .emit_lab_completed(&publisher::LabCompletedEvent {
                                session_id,
                                user_id: uid,
                                lab_id: session.lab_id.clone(),
                                score: session.score as i32,
                                xp_earned: (session.score * 10) as i32,
                                timestamp: chrono::Utc::now(),
                            })
                            .await;
                    }
                }
            }

            Ok(Json(result))
        }
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )),
    }
}
