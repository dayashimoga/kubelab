use crate::models::{Claims, User};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JwtError {
    #[error("Token creation failed")]
    CreationError,
    #[error("Invalid or expired token")]
    ValidationError,
}

pub struct JwtService {
    secret: String,
    duration_hours: i64,
}

impl JwtService {
    pub fn new(secret: String) -> Self {
        Self {
            secret,
            duration_hours: 24,
        }
    }

    pub fn generate_token(&self, user: &User) -> Result<(String, usize), JwtError> {
        let (access_token, _, expires_in) = self.generate_tokens(user)?;
        Ok((access_token, expires_in))
    }

    pub fn generate_tokens(&self, user: &User) -> Result<(String, String, usize), JwtError> {
        let now = Utc::now();
        let expires_at = now + Duration::hours(self.duration_hours);
        let exp = expires_at.timestamp() as usize;

        let refresh_expires_at = now + Duration::days(7);
        let refresh_exp = refresh_expires_at.timestamp() as usize;

        let role_str = match user.role {
            crate::models::UserRole::Admin => "admin",
            crate::models::UserRole::Instructor => "instructor",
            crate::models::UserRole::Learner => "learner",
        };

        let access_claims = Claims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            role: role_str.to_string(),
            exp,
            iat: now.timestamp() as usize,
            token_type: Some("access".to_string()),
        };

        let refresh_claims = Claims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            role: role_str.to_string(),
            exp: refresh_exp,
            iat: now.timestamp() as usize,
            token_type: Some("refresh".to_string()),
        };

        let access_token = encode(
            &Header::default(),
            &access_claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|_| JwtError::CreationError)?;

        let refresh_token = encode(
            &Header::default(),
            &refresh_claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|_| JwtError::CreationError)?;

        Ok((access_token, refresh_token, (self.duration_hours * 3600) as usize))
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, JwtError> {
        let decoded = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| JwtError::ValidationError)?;

        Ok(decoded.claims)
    }

    pub fn verify_refresh_token(&self, token: &str) -> Result<Claims, JwtError> {
        let claims = self.verify_token(token)?;
        if let Some(ref t) = claims.token_type {
            if t != "refresh" {
                return Err(JwtError::ValidationError);
            }
        }
        Ok(claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_jwt_generation_and_verification() {
        let jwt = JwtService::new("my-test-secret-key-32-characters-long!".to_string());
        let user = User {
            id: Uuid::new_v4(),
            email: "learner@kubelab.io".to_string(),
            name: "Cloud Learner".to_string(),
            password_hash: "hash".to_string(),
            role: crate::models::UserRole::Learner,
            avatar_url: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let (access, refresh, expires_in) = jwt.generate_tokens(&user).unwrap();
        assert!(expires_in > 0);
        assert!(!access.is_empty());
        assert!(!refresh.is_empty());

        let claims = jwt.verify_token(&access).unwrap();
        assert_eq!(claims.sub, user.id.to_string());
        assert_eq!(claims.email, "learner@kubelab.io");
        assert_eq!(claims.role, "learner");

        let refresh_claims = jwt.verify_refresh_token(&refresh).unwrap();
        assert_eq!(refresh_claims.sub, user.id.to_string());
    }
}
