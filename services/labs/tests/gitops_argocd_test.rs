use kubelab_labs::gitops::{GitOpsEvaluator, SyncStatusCode, HealthStatusCode};

#[test]
fn test_argocd_application_status_and_drift_detection() {
    let synced_json = serde_json::json!({
        "metadata": {
            "name": "lab-workloads"
        },
        "spec": {
            "project": "kubelab-project",
            "source": {
                "repoURL": "https://github.com/dayashimoga/kubelab.git",
                "targetRevision": "main"
            }
        },
        "status": {
            "sync": {
                "status": "Synced"
            },
            "health": {
                "status": "Healthy"
            },
            "resources": [
                { "name": "nginx-deployment", "kind": "Deployment", "status": "Synced" },
                { "name": "nginx-service", "kind": "Service", "status": "Synced" }
            ]
        }
    });

    let status = GitOpsEvaluator::parse_status_json(&synced_json).expect("Parse synced status");
    assert_eq!(status.name, "lab-workloads");
    assert_eq!(status.sync_status, SyncStatusCode::Synced);
    assert_eq!(status.health_status, HealthStatusCode::Healthy);
    assert_eq!(status.resources_synced, 2);
    assert!(!status.drift_detected);

    let drift_json = serde_json::json!({
        "metadata": { "name": "lab-workloads" },
        "spec": { "project": "kubelab-project", "source": { "repoURL": "https://github.com/dayashimoga/kubelab.git" } },
        "status": {
            "sync": { "status": "OutOfSync" },
            "health": { "status": "Degraded" },
            "resources": [
                { "name": "nginx-deployment", "kind": "Deployment", "status": "OutOfSync" }
            ]
        }
    });

    let drift_status = GitOpsEvaluator::parse_status_json(&drift_json).expect("Parse drift status");
    assert_eq!(drift_status.sync_status, SyncStatusCode::OutOfSync);
    assert_eq!(drift_status.health_status, HealthStatusCode::Degraded);
    assert!(drift_status.drift_detected);
}
