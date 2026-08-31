use crate::models::SandboxNamespace;
use chrono::{Duration, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum OrchestratorError {
    #[error("Failed to provision namespace")]
    ProvisionError,
    #[error("Namespace not found")]
    NotFound,
}

pub struct LabProvisioner {
    active_sandboxes: Arc<RwLock<HashMap<String, SandboxNamespace>>>,
}

impl Default for LabProvisioner {
    fn default() -> Self {
        Self::new()
    }
}

impl LabProvisioner {
    pub fn new() -> Self {
        Self {
            active_sandboxes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn provision_sandbox(
        &self,
        session_id: Uuid,
        user_id: &str,
        duration_minutes: u32,
    ) -> Result<SandboxNamespace, OrchestratorError> {
        let namespace_name = format!("lab-{}", session_id.simple());
        let now = Utc::now();
        let expires_at = now + Duration::minutes(duration_minutes as i64);

        let sandbox = SandboxNamespace {
            namespace: namespace_name.clone(),
            session_id,
            user_id: user_id.to_string(),
            created_at: now,
            expires_at,
            cpu_limit: "500m".to_string(),
            memory_limit: "512Mi".to_string(),
            network_policy_applied: true,
        };

        let mut map = self.active_sandboxes.write().await;
        map.insert(namespace_name, sandbox.clone());

        Ok(sandbox)
    }

    pub async fn destroy_sandbox(&self, namespace: &str) -> Result<(), OrchestratorError> {
        let mut map = self.active_sandboxes.write().await;
        map.remove(namespace).ok_or(OrchestratorError::NotFound)?;
        Ok(())
    }

    pub async fn list_active_sandboxes(&self) -> Vec<SandboxNamespace> {
        let map = self.active_sandboxes.read().await;
        map.values().cloned().collect()
    }
}
