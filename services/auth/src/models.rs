use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Learner,
    Instructor,
    Admin,
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::Learner
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: UserRole,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub email: String,
    pub role: String,
    pub exp: usize,
    pub iat: usize,
    #[serde(default)]
    pub token_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthTokens {
    pub access_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    pub token_type: String,
    pub expires_in: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub user: User,
    pub tokens: AuthTokens,
}
