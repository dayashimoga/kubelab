use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Pod;
use kube::api::{Api, DeleteParams, ListParams, Patch, PatchParams};
use kube::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ChaosFaultType {
    PodKill,
    NetworkLatency,
    DnsFailure,
    BadDeploymentImage,
    OomSaturation,
    GitOpsDrift,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChaosInjection {
    pub fault_type: ChaosFaultType,
    pub target_namespace: String,
    pub target_workload: String,
    pub description: String,
    pub active: bool,
}

pub struct ChaosEngine;

impl ChaosEngine {
    pub fn inject_fault(
        fault_type: ChaosFaultType,
        target_namespace: &str,
        target_workload: &str,
    ) -> ChaosInjection {
        let desc = match fault_type {
            ChaosFaultType::PodKill => format!("Terminating random pods in {}/{}", target_namespace, target_workload),
            ChaosFaultType::NetworkLatency => format!("Adding 200ms latency to {}/{}", target_namespace, target_workload),
            ChaosFaultType::DnsFailure => format!("Injecting NXDOMAIN responses in namespace {}", target_namespace),
            ChaosFaultType::BadDeploymentImage => format!("Patching {}/{} to non-existent image", target_namespace, target_workload),
            ChaosFaultType::OomSaturation => format!("Triggering memory leak simulation in {}/{}", target_namespace, target_workload),
            ChaosFaultType::GitOpsDrift => format!("Overriding cluster state directly to cause Argo CD OutOfSync in {}", target_namespace),
        };

        ChaosInjection {
            fault_type,
            target_namespace: target_namespace.to_string(),
            target_workload: target_workload.to_string(),
            description: desc,
            active: true,
        }
    }

    /// Execute live fault injection against Kubernetes cluster
    pub async fn execute_fault_k8s(
        client: &Client,
        fault_type: &ChaosFaultType,
        target_namespace: &str,
        target_workload: &str,
    ) -> Result<ChaosInjection, Box<dyn std::error::Error + Send + Sync>> {
        let injection = Self::inject_fault(fault_type.clone(), target_namespace, target_workload);

        match fault_type {
            ChaosFaultType::PodKill => {
                let pods_api: Api<Pod> = Api::namespaced(client.clone(), target_namespace);
                let list_params = ListParams::default();
                let pods = pods_api.list(&list_params).await?;
                for pod in pods.items {
                    let name = pod.metadata.name.unwrap_or_default();
                    if name.contains(target_workload) || target_workload.is_empty() || target_workload == "*" {
                        info!("Chaos: Terminating pod '{}' in namespace '{}'...", name, target_namespace);
                        let _ = pods_api.delete(&name, &DeleteParams::default()).await?;
                        break;
                    }
                }
            }
            ChaosFaultType::BadDeploymentImage => {
                let deploy_api: Api<Deployment> = Api::namespaced(client.clone(), target_namespace);
                let patch = json!({
                    "spec": {
                        "template": {
                            "spec": {
                                "containers": [{
                                    "name": target_workload,
                                    "image": "docker.io/library/nonexistent-image:fault-injection-v999"
                                }]
                            }
                        }
                    }
                });
                info!("Chaos: Patching deployment '{}' in namespace '{}' to broken image...", target_workload, target_namespace);
                let patch_params = PatchParams::default();
                let _ = deploy_api.patch(target_workload, &patch_params, &Patch::Strategic(patch)).await?;
            }
            _ => {
                info!("Chaos: Emulated fault injection recorded for {}/{}", target_namespace, target_workload);
            }
        }

        Ok(injection)
    }
}
