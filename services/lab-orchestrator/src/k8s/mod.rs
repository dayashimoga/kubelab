pub mod manifest_applier;
pub mod namespace_provisioner;

use kube::Client;
use tracing::info;

#[derive(Clone)]
pub struct KubeClusterClient {
    client: Client,
}

impl KubeClusterClient {
    pub async fn try_new() -> Result<Self, kube::Error> {
        info!("Attempting to initialize native Kubernetes client from kubeconfig or in-cluster serviceaccount...");
        let client = Client::try_default().await?;
        info!("Kubernetes client initialized successfully.");
        Ok(Self { client })
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}
