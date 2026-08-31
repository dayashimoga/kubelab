use k8s_openapi::api::core::v1::Namespace;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use std::collections::BTreeMap;

#[test]
fn test_kubernetes_namespace_and_isolation_policy_generation() {
    let mut labels = BTreeMap::new();
    labels.insert("app.kubernetes.io/managed-by".to_string(), "kubelab-orchestrator".to_string());
    labels.insert("kubelab.io/sandbox".to_string(), "true".to_string());

    let ns = Namespace {
        metadata: ObjectMeta {
            name: Some("sandbox-user-42".to_string()),
            labels: Some(labels),
            ..Default::default()
        },
        ..Default::default()
    };

    let serialized = serde_json::to_string(&ns).expect("Serialize Namespace");
    assert!(serialized.contains("sandbox-user-42"));
    assert!(serialized.contains("kubelab-orchestrator"));
}
