use crate::jwt::JwtService;
use crate::models::{AuthResponse, AuthTokens, User, UserRole};
use crate::password::{hash_password, verify_password};
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("User with this email already exists")]
    UserAlreadyExists,
    #[error("Invalid email or password")]
    InvalidCredentials,
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid or expired refresh token")]
    InvalidRefreshToken,
    #[error("Internal auth error")]
    InternalError,
}

pub struct AuthService {
    users: Arc<RwLock<HashMap<String, User>>>,
    jwt: JwtService,
}

impl AuthService {
    pub fn new(jwt_secret: String) -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
            jwt: JwtService::new(jwt_secret),
        }
    }

    pub fn jwt(&self) -> &JwtService {
        &self.jwt
    }

    pub async fn register(
        &self,
        email: &str,
        name: &str,
        password: &str,
        role: Option<UserRole>,
    ) -> Result<AuthResponse, AuthError> {
        let mut users = self.users.write().await;
        if users.contains_key(email) {
            return Err(AuthError::UserAlreadyExists);
        }

        let password_hash = hash_password(password).map_err(|_| AuthError::InternalError)?;
        let now = Utc::now();

        let user = User {
            id: Uuid::new_v4(),
            email: email.to_string(),
            name: name.to_string(),
            password_hash,
            role: role.unwrap_or(UserRole::Learner),
            avatar_url: None,
            created_at: now,
            updated_at: now,
        };

        let (access_token, refresh_token, expires_in) = self
            .jwt
            .generate_tokens(&user)
            .map_err(|_| AuthError::InternalError)?;

        users.insert(email.to_string(), user.clone());

        Ok(AuthResponse {
            user,
            tokens: AuthTokens {
                access_token,
                refresh_token: Some(refresh_token),
                token_type: "Bearer".to_string(),
                expires_in,
            },
        })
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<AuthResponse, AuthError> {
        let users = self.users.read().await;
        let user = users.get(email).ok_or(AuthError::InvalidCredentials)?;

        let valid = verify_password(password, &user.password_hash)
            .map_err(|_| AuthError::InvalidCredentials)?;

        if !valid {
            return Err(AuthError::InvalidCredentials);
        }

        let (access_token, refresh_token, expires_in) = self
            .jwt
            .generate_tokens(user)
            .map_err(|_| AuthError::InternalError)?;

        Ok(AuthResponse {
            user: user.clone(),
            tokens: AuthTokens {
                access_token,
                refresh_token: Some(refresh_token),
                token_type: "Bearer".to_string(),
                expires_in,
            },
        })
    }

    pub async fn refresh_tokens(&self, refresh_token: &str) -> Result<AuthTokens, AuthError> {
        let claims = self
            .jwt
            .verify_refresh_token(refresh_token)
            .map_err(|_| AuthError::InvalidRefreshToken)?;

        let user_id = Uuid::parse_str(&claims.sub).map_err(|_| AuthError::InvalidRefreshToken)?;
        let user = self.get_user_by_id(&user_id).await?;

        let (access_token, new_refresh_token, expires_in) = self
            .jwt
            .generate_tokens(&user)
            .map_err(|_| AuthError::InternalError)?;

        Ok(AuthTokens {
            access_token,
            refresh_token: Some(new_refresh_token),
            token_type: "Bearer".to_string(),
            expires_in,
        })
    }

    pub async fn get_user_by_id(&self, user_id: &Uuid) -> Result<User, AuthError> {
        let users = self.users.read().await;
        for user in users.values() {
            if &user.id == user_id {
                return Ok(user.clone());
            }
        }
        Err(AuthError::UserNotFound)
    }

    pub fn verify_token(&self, token: &str) -> Result<crate::models::Claims, crate::jwt::JwtError> {
        self.jwt.verify_token(token)
    }
}
