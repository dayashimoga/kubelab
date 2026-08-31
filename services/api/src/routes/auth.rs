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

#[derive(Debug, Deserialize)]
pub struct RefreshTokenDto {
    pub refresh_token: String,
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
    pub token: String,
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

        // Check if token was blacklisted/revoked in Redis
        if let Some(ref cache) = app_state.cache {
            if cache
                .session_store()
                .is_revoked(token)
                .await
                .unwrap_or(false)
            {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(ErrorResponse {
                        error: "Token has been revoked".to_string(),
                    }),
                ));
            }
        }

        let claims = app_state.auth.jwt().verify_token(token).map_err(|e| {
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
            token: token.to_string(),
        }))
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh))
        .route("/logout", post(logout))
        .route("/me", get(get_me))
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterDto>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let email = payload.email.trim();
    let name = payload.name.trim();

    if name.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Name cannot be empty".to_string(),
            }),
        ));
    }

    if !email.contains('@') || !email.contains('.') {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid email address format".to_string(),
            }),
        ));
    }

    if payload.password.len() < 8 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Password must be at least 8 characters long".to_string(),
            }),
        ));
    }

    let role_str = match payload.role.as_ref().unwrap_or(&UserRole::Learner) {
        UserRole::Admin => "admin",
        UserRole::Instructor => "instructor",
        UserRole::Learner => "learner",
    };

    match state
        .auth
        .register(email, name, &payload.password, payload.role)
        .await
    {
        Ok(res) => {
            // If PostgreSQL is configured, persist user
            if let Some(ref db) = state.db {
                let _ = db
                    .users()
                    .create_user(email, name, &payload.password, role_str)
                    .await;
            }
            Ok((StatusCode::CREATED, Json(res)))
        }
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
        Ok(res) => {
            // Cache active session in Redis if connected
            if let Some(ref cache) = state.cache {
                let role_str = match res.user.role {
                    UserRole::Admin => "admin",
                    UserRole::Instructor => "instructor",
                    UserRole::Learner => "learner",
                };
                let cached = crate::cache::session_store::CachedSession {
                    user_id: res.user.id,
                    email: res.user.email.clone(),
                    role: role_str.to_string(),
                    session_id: res.tokens.access_token.clone(),
                };
                let _ = cache
                    .session_store()
                    .set_session(&res.tokens.access_token, &cached, 86400)
                    .await;
            }
            Ok((StatusCode::OK, Json(res)))
        }
        Err(e) => Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )),
    }
}

async fn refresh(
    State(state): State<AppState>,
    Json(payload): Json<RefreshTokenDto>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    match state.auth.refresh_tokens(&payload.refresh_token).await {
        Ok(tokens) => Ok((StatusCode::OK, Json(tokens))),
        Err(e) => Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )),
    }
}

async fn logout(
    State(state): State<AppState>,
    AuthClaims(user): AuthClaims,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    // Add access token to revocation blacklist
    if let Some(ref cache) = state.cache {
        let _ = cache.session_store().revoke_token(&user.token, 86400).await;
    }
    Ok((
        StatusCode::OK,
        Json(serde_json::json!({ "message": "Successfully logged out" })),
    ))
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
