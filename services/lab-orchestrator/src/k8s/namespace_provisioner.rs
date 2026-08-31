use kube::{Api, Client};
use k8s_openapi::api::core::v1::{Namespace, ResourceQuota, ResourceQuotaSpec};
use k8s_openapi::api::networking::v1::{NetworkPolicy, NetworkPolicySpec};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::{ObjectMeta, LabelSelector};
use std::collections::BTreeMap;
use tracing::info;

pub struct NamespaceProvisioner<'a> {
    client: &'a Client,
}

impl<'a> NamespaceProvisioner<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Provision ephemeral sandbox namespace with strict resource limits and isolation
    pub async fn provision_sandbox_namespace(&self, namespace_name: &str, user_id: &str) -> Result<Namespace, kube::Error> {
        let namespaces: Api<Namespace> = Api::all(self.client.clone());

        let mut labels = BTreeMap::new();
        labels.insert("app.kubernetes.io/managed-by".to_string(), "kubelab-orchestrator".to_string());
        labels.insert("kubelab.io/sandbox".to_string(), "true".to_string());
        labels.insert("kubelab.io/user-id".to_string(), user_id.to_string());

        let ns = Namespace {
            metadata: ObjectMeta {
                name: Some(namespace_name.to_string()),
                labels: Some(labels),
                ..Default::default()
            },
            ..Default::default()
        };

        info!("Creating Kubernetes namespace '{}'...", namespace_name);
        let created_ns = namespaces.create(&Default::default(), &ns).await?;

        // Apply ResourceQuota
        self.apply_resource_quota(namespace_name).await?;

        // Apply Default Deny NetworkPolicy
        self.apply_isolation_network_policy(namespace_name).await?;

        Ok(created_ns)
    }

    async fn apply_resource_quota(&self, namespace_name: &str) -> Result<ResourceQuota, kube::Error> {
        let quotas: Api<ResourceQuota> = Api::namespaced(self.client.clone(), namespace_name);

        let mut hard = BTreeMap::new();
        hard.insert("requests.cpu".to_string(), k8s_openapi::apimachinery::pkg::api::resource::Quantity("2".to_string()));
        hard.insert("requests.memory".to_string(), k8s_openapi::apimachinery::pkg::api::resource::Quantity("2Gi".to_string()));
        hard.insert("limits.cpu".to_string(), k8s_openapi::apimachinery::pkg::api::resource::Quantity("4".to_string()));
        hard.insert("limits.memory".to_string(), k8s_openapi::apimachinery::pkg::api::resource::Quantity("4Gi".to_string()));
        hard.insert("pods".to_string(), k8s_openapi::apimachinery::pkg::api::resource::Quantity("10".to_string()));

        let quota = ResourceQuota {
            metadata: ObjectMeta {
                name: Some("sandbox-quota".to_string()),
                namespace: Some(namespace_name.to_string()),
                ..Default::default()
            },
            spec: Some(ResourceQuotaSpec {
                hard: Some(hard),
                ..Default::default()
            }),
            ..Default::default()
        };

        quotas.create(&Default::default(), &quota).await
    }

    async fn apply_isolation_network_policy(&self, namespace_name: &str) -> Result<NetworkPolicy, kube::Error> {
        let policies: Api<NetworkPolicy> = Api::namespaced(self.client.clone(), namespace_name);

        let policy = NetworkPolicy {
            metadata: ObjectMeta {
                name: Some("tenant-isolation-policy".to_string()),
                namespace: Some(namespace_name.to_string()),
                ..Default::default()
            },
            spec: Some(NetworkPolicySpec {
                pod_selector: LabelSelector::default(),
                policy_types: Some(vec!["Ingress".to_string(), "Egress".to_string()]),
                ..Default::default()
            }),
            ..Default::default()
        };

        policies.create(&Default::default(), &policy).await
    }

    /// Tear down sandbox namespace and cascade-delete all objects
    pub async fn destroy_sandbox_namespace(&self, namespace_name: &str) -> Result<(), kube::Error> {
        let namespaces: Api<Namespace> = Api::all(self.client.clone());
        info!("Tearing down Kubernetes namespace '{}'...", namespace_name);
        namespaces.delete(namespace_name, &Default::default()).await?;
        Ok(())
    }
}
