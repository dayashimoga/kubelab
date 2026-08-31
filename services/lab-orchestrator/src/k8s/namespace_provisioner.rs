use k8s_openapi::api::core::v1::{
    LimitRange, LimitRangeItem, LimitRangeSpec, Namespace, ResourceQuota, ResourceQuotaSpec,
};
use k8s_openapi::api::networking::v1::{
    IPBlock, NetworkPolicy, NetworkPolicyEgressRule, NetworkPolicyPeer, NetworkPolicyPort,
    NetworkPolicySpec,
};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::{LabelSelector, ObjectMeta};
use kube::{Api, Client};
use std::collections::BTreeMap;
use tracing::info;

pub struct NamespaceProvisioner<'a> {
    client: &'a Client,
}

impl<'a> NamespaceProvisioner<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Provision ephemeral sandbox namespace with strict PSS, resource limits, and isolation
    pub async fn provision_sandbox_namespace(
        &self,
        namespace_name: &str,
        user_id: &str,
    ) -> Result<Namespace, kube::Error> {
        let namespaces: Api<Namespace> = Api::all(self.client.clone());

        let mut labels = BTreeMap::new();
        labels.insert(
            "app.kubernetes.io/managed-by".to_string(),
            "kubelab-orchestrator".to_string(),
        );
        labels.insert("kubelab.io/sandbox".to_string(), "true".to_string());
        labels.insert("kubelab.io/user-id".to_string(), user_id.to_string());

        // Enforce Pod Security Standards (PSS) Restricted Profile
        labels.insert(
            "pod-security.kubernetes.io/enforce".to_string(),
            "restricted".to_string(),
        );
        labels.insert(
            "pod-security.kubernetes.io/enforce-version".to_string(),
            "latest".to_string(),
        );
        labels.insert(
            "pod-security.kubernetes.io/audit".to_string(),
            "restricted".to_string(),
        );
        labels.insert(
            "pod-security.kubernetes.io/warn".to_string(),
            "restricted".to_string(),
        );

        let ns = Namespace {
            metadata: ObjectMeta {
                name: Some(namespace_name.to_string()),
                labels: Some(labels),
                ..Default::default()
            },
            ..Default::default()
        };

        info!(
            "Creating hardened Kubernetes namespace '{}'...",
            namespace_name
        );
        let created_ns = namespaces.create(&Default::default(), &ns).await?;

        // 1. Apply ResourceQuota
        self.apply_resource_quota(namespace_name).await?;

        // 2. Apply LimitRange (container defaults)
        self.apply_limit_range(namespace_name).await?;

        // 3. Apply Zero-Trust NetworkPolicy
        self.apply_isolation_network_policy(namespace_name).await?;

        Ok(created_ns)
    }

    async fn apply_resource_quota(
        &self,
        namespace_name: &str,
    ) -> Result<ResourceQuota, kube::Error> {
        let quotas: Api<ResourceQuota> = Api::namespaced(self.client.clone(), namespace_name);

        let mut hard = BTreeMap::new();
        hard.insert(
            "requests.cpu".to_string(),
            k8s_openapi::apimachinery::pkg::api::resource::Quantity("2".to_string()),
        );
        hard.insert(
            "requests.memory".to_string(),
            k8s_openapi::apimachinery::pkg::api::resource::Quantity("2Gi".to_string()),
        );
        hard.insert(
            "limits.cpu".to_string(),
            k8s_openapi::apimachinery::pkg::api::resource::Quantity("4".to_string()),
        );
        hard.insert(
            "limits.memory".to_string(),
            k8s_openapi::apimachinery::pkg::api::resource::Quantity("4Gi".to_string()),
        );
        hard.insert(
            "pods".to_string(),
            k8s_openapi::apimachinery::pkg::api::resource::Quantity("10".to_string()),
        );

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

    async fn apply_limit_range(&self, namespace_name: &str) -> Result<LimitRange, kube::Error> {
        let limits_api: Api<LimitRange> = Api::namespaced(self.client.clone(), namespace_name);

        let mut default_limits = BTreeMap::new();
        default_limits.insert(
            "cpu".to_string(),
            k8s_openapi::apimachinery::pkg::api::resource::Quantity("500m".to_string()),
        );
        default_limits.insert(
            "memory".to_string(),
            k8s_openapi::apimachinery::pkg::api::resource::Quantity("512Mi".to_string()),
        );

        let mut default_requests = BTreeMap::new();
        default_requests.insert(
            "cpu".to_string(),
            k8s_openapi::apimachinery::pkg::api::resource::Quantity("100m".to_string()),
        );
        default_requests.insert(
            "memory".to_string(),
            k8s_openapi::apimachinery::pkg::api::resource::Quantity("128Mi".to_string()),
        );

        let item = LimitRangeItem {
            type_: "Container".to_string(),
            default: Some(default_limits),
            default_request: Some(default_requests),
            ..Default::default()
        };

        let limit_range = LimitRange {
            metadata: ObjectMeta {
                name: Some("sandbox-limit-range".to_string()),
                namespace: Some(namespace_name.to_string()),
                ..Default::default()
            },
            spec: Some(LimitRangeSpec { limits: vec![item] }),
        };

        limits_api.create(&Default::default(), &limit_range).await
    }

    async fn apply_isolation_network_policy(
        &self,
        namespace_name: &str,
    ) -> Result<NetworkPolicy, kube::Error> {
        let policies: Api<NetworkPolicy> = Api::namespaced(self.client.clone(), namespace_name);

        // Allow DNS resolution (port 53) while isolating pod-to-pod and blocking metadata APIs
        let dns_port_udp = NetworkPolicyPort {
            port: Some(k8s_openapi::apimachinery::pkg::util::intstr::IntOrString::Int(53)),
            protocol: Some("UDP".to_string()),
            ..Default::default()
        };
        let dns_port_tcp = NetworkPolicyPort {
            port: Some(k8s_openapi::apimachinery::pkg::util::intstr::IntOrString::Int(53)),
            protocol: Some("TCP".to_string()),
            ..Default::default()
        };

        let egress_rule = NetworkPolicyEgressRule {
            ports: Some(vec![dns_port_udp, dns_port_tcp]),
            to: Some(vec![NetworkPolicyPeer {
                ip_block: Some(IPBlock {
                    cidr: "0.0.0.0/0".to_string(),
                    except: Some(vec!["169.254.169.254/32".to_string()]), // Block AWS/GCP/Azure IMDS metadata
                }),
                ..Default::default()
            }]),
        };

        let policy = NetworkPolicy {
            metadata: ObjectMeta {
                name: Some("tenant-isolation-policy".to_string()),
                namespace: Some(namespace_name.to_string()),
                ..Default::default()
            },
            spec: Some(NetworkPolicySpec {
                pod_selector: LabelSelector::default(),
                policy_types: Some(vec!["Ingress".to_string(), "Egress".to_string()]),
                egress: Some(vec![egress_rule]),
                ..Default::default()
            }),
        };

        policies.create(&Default::default(), &policy).await
    }

    /// Tear down sandbox namespace and cascade-delete all objects
    pub async fn destroy_sandbox_namespace(&self, namespace_name: &str) -> Result<(), kube::Error> {
        let namespaces: Api<Namespace> = Api::all(self.client.clone());
        info!("Tearing down Kubernetes namespace '{}'...", namespace_name);
        namespaces
            .delete(namespace_name, &Default::default())
            .await?;
        Ok(())
    }
}
