use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedSession {
    pub user_id: Uuid,
    pub email: String,
    pub role: String,
    pub session_id: String,
}

#[derive(Clone)]
pub struct SessionStore {
    conn: ConnectionManager,
}

impl SessionStore {
    pub fn new(conn: ConnectionManager) -> Self {
        Self { conn }
    }

    /// Cache active session for 24h
    pub async fn set_session(&mut self, token_id: &str, session: &CachedSession, ttl_seconds: u64) -> Result<(), redis::RedisError> {
        let key = format!("session:{}", token_id);
        let payload = serde_json::to_string(session).unwrap_or_default();
        let _: () = self.conn.set_ex(key, payload, ttl_seconds).await?;
        Ok(())
    }

    /// Get active session
    pub async fn get_session(&mut self, token_id: &str) -> Result<Option<CachedSession>, redis::RedisError> {
        let key = format!("session:{}", token_id);
        let data: Option<String> = self.conn.get(key).await?;
        Ok(data.and_then(|s| serde_json::from_str(&s).ok()))
    }

    /// Blacklist / revoke a token immediately
    pub async fn revoke_token(&mut self, token_id: &str, remaining_ttl_seconds: u64) -> Result<(), redis::RedisError> {
        let key = format!("blacklist:{}", token_id);
        let _: () = self.conn.set_ex(key, "revoked", remaining_ttl_seconds).await?;
        Ok(())
    }

    /// Check if token is blacklisted
    pub async fn is_revoked(&mut self, token_id: &str) -> Result<bool, redis::RedisError> {
        let key = format!("blacklist:{}", token_id);
        let exists: bool = self.conn.exists(key).await?;
        Ok(exists)
    }
}
