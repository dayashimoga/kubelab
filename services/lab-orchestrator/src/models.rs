use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxNamespace {
    pub namespace: String,
    pub session_id: Uuid,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub cpu_limit: String,
    pub memory_limit: String,
    pub network_policy_applied: bool,
}
