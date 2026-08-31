use serde::{Deserialize, Serialize};

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
}
