use kube::{
    api::{Api, DynamicObject, Patch, PatchParams},
    discovery::{Discovery, Scope},
    Client,
};
use serde_json::Value;
use tracing::info;

pub struct ManifestApplier<'a> {
    client: &'a Client,
}

impl<'a> ManifestApplier<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Server-side apply any valid Kubernetes YAML manifest into target namespace
    pub async fn apply_yaml_manifest(
        &self,
        namespace: &str,
        yaml_str: &str,
    ) -> Result<Vec<DynamicObject>, Box<dyn std::error::Error + Send + Sync>> {
        let mut applied = Vec::new();
        let discovery = Discovery::new(self.client.clone()).run().await?;

        // Split multi-document YAML manifests
        for doc in yaml_str.split("\n---") {
            let trimmed = doc.trim();
            if trimmed.is_empty() {
                continue;
            }

            let value: Value = serde_yaml::from_str(trimmed)?;
            if value.is_null() {
                continue;
            }

            let obj: DynamicObject = serde_json::from_value(value.clone())?;
            let gvk = if let Some(type_meta) = &obj.types {
                kube::api::GroupVersionKind::try_from(type_meta)?
            } else {
                continue;
            };

            let (ar, caps) = discovery
                .resolve_gvk(&gvk)
                .ok_or_else(|| format!("Could not resolve GVK: {:?}", gvk))?;

            let api: Api<DynamicObject> = if caps.scope == Scope::Cluster {
                Api::all_with(self.client.clone(), &ar)
            } else {
                Api::namespaced_with(self.client.clone(), namespace, &ar)
            };

            let name = obj
                .metadata
                .name
                .clone()
                .unwrap_or_else(|| "unnamed".to_string());
            info!(
                "Applying Kubernetes resource {}/{} in namespace '{}'...",
                gvk.kind, name, namespace
            );

            let patch_params = PatchParams::apply("kubelab-orchestrator").force();
            let patched = api.patch(&name, &patch_params, &Patch::Apply(&obj)).await?;
            applied.push(patched);
        }

        Ok(applied)
    }
}
