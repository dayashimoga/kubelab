use crate::k8s::namespace_provisioner::NamespaceProvisioner;
use crate::models::SandboxNamespace;
use chrono::{Duration, Utc};
use kube::Client;
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use tracing::{info, warn};
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum OrchestratorError {
    #[error("Failed to provision namespace: {0}")]
    ProvisionError(String),
    #[error("Namespace not found")]
    NotFound,
    #[error("Kubernetes client error: {0}")]
    KubeError(String),
}

#[derive(Clone)]
pub struct LabProvisioner {
    active_sandboxes: Arc<RwLock<HashMap<String, SandboxNamespace>>>,
    k8s_client: Option<Client>,
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
            k8s_client: None,
        }
    }

    pub fn with_k8s_client(client: Client) -> Self {
        Self {
            active_sandboxes: Arc::new(RwLock::new(HashMap::new())),
            k8s_client: Some(client),
        }
    }

    pub fn client(&self) -> Option<&Client> {
        self.k8s_client.as_ref()
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

        // If real Kubernetes client is available, provision physical namespace with Quota & NetPol
        if let Some(ref client) = self.k8s_client {
            info!(
                "Provisioning live Kubernetes sandbox namespace '{}' for user '{}'...",
                namespace_name, user_id
            );
            let provisioner = NamespaceProvisioner::new(client);
            if let Err(e) = provisioner
                .provision_sandbox_namespace(&namespace_name, user_id)
                .await
            {
                warn!("Live namespace provisioning failed or already exists: {:?}. Proceeding with tracked sandbox state.", e);
            }
        }

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
        if let Some(ref client) = self.k8s_client {
            info!(
                "Tearing down live Kubernetes sandbox namespace '{}'...",
                namespace
            );
            let provisioner = NamespaceProvisioner::new(client);
            if let Err(e) = provisioner.destroy_sandbox_namespace(namespace).await {
                warn!(
                    "Live namespace teardown returned: {:?}. Removing from tracked sandboxes.",
                    e
                );
            }
        }

        let mut map = self.active_sandboxes.write().await;
        map.remove(namespace).ok_or(OrchestratorError::NotFound)?;
        Ok(())
    }

    pub async fn list_active_sandboxes(&self) -> Vec<SandboxNamespace> {
        let map = self.active_sandboxes.read().await;
        map.values().cloned().collect()
    }
}
