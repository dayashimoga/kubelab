use crate::state::AppState;
use axum::{
    extract::{FromRef, FromRequestParts, State},
    http::{header, request::Parts, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use kubelab_auth::models::UserRole;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct RegisterDto {
    pub email: String,
    pub name: String,
    pub password: String,
    pub role: Option<UserRole>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub id: Uuid,
    pub email: String,
    pub role: UserRole,
}

pub struct AuthClaims(pub AuthenticatedUser);

#[axum::async_trait]
impl<S> FromRequestParts<S> for AuthClaims
where
    S: Send + Sync,
    AppState: axum::extract::FromRef<S>,
{
    type Rejection = (StatusCode, Json<ErrorResponse>);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(ErrorResponse {
                        error: "Missing Authorization header".to_string(),
                    }),
                )
            })?;

        if !auth_header.starts_with("Bearer ") {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: "Invalid token type, expected Bearer".to_string(),
                }),
            ));
        }

        let token = &auth_header[7..];
        let claims = app_state
            .auth
            .jwt()
            .verify_token(token)
            .map_err(|e| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(ErrorResponse {
                        error: format!("Invalid or expired token: {}", e),
                    }),
                )
            })?;

        let user_id = Uuid::parse_str(&claims.sub).map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: "Invalid user ID in token".to_string(),
                }),
            )
        })?;

        let role = match claims.role.as_str() {
            "admin" => UserRole::Admin,
            "instructor" => UserRole::Instructor,
            _ => UserRole::Learner,
        };

        Ok(AuthClaims(AuthenticatedUser {
            id: user_id,
            email: claims.email,
            role,
        }))
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/me", get(get_me))
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterDto>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    match state
        .auth
        .register(&payload.email, &payload.name, &payload.password, payload.role)
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

async fn get_me(
    State(state): State<AppState>,
    AuthClaims(user): AuthClaims,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    match state.auth.get_user_by_id(&user.id).await {
        Ok(u) => Ok((StatusCode::OK, Json(u))),
        Err(e) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )),
    }
}
