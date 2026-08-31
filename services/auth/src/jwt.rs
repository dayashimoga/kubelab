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
        let now = Utc::now();
        let expires_at = now + Duration::hours(self.duration_hours);
        let exp = expires_at.timestamp() as usize;

        let role_str = match user.role {
            crate::models::UserRole::Admin => "admin",
            crate::models::UserRole::Instructor => "instructor",
            crate::models::UserRole::Learner => "learner",
        };

        let claims = Claims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            role: role_str.to_string(),
            exp,
            iat: now.timestamp() as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|_| JwtError::CreationError)?;

        Ok((token, (self.duration_hours * 3600) as usize))
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

        let (token, expires_in) = jwt.generate_token(&user).unwrap();
        assert!(expires_in > 0);

        let claims = jwt.verify_token(&token).unwrap();
        assert_eq!(claims.sub, user.id.to_string());
        assert_eq!(claims.email, "learner@kubelab.io");
        assert_eq!(claims.role, "learner");
    }
}
