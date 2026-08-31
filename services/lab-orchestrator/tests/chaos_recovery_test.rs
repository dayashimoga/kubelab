use kubelab_lab_orchestrator::{ChaosEngine, ChaosFaultType, LabProvisioner};
use uuid::Uuid;

#[tokio::test]
async fn test_sandbox_provisioning_and_chaos_injection() {
    let provisioner = LabProvisioner::new();
    let session_id = Uuid::new_v4();

    // 1. Provision sandbox
    let sandbox = provisioner
        .provision_sandbox(session_id, "user-123", 30)
        .await
        .expect("Sandbox should provision");

    assert_eq!(sandbox.session_id, session_id);
    assert!(sandbox.network_policy_applied);

    // 2. Inject Chaos Fault
    let chaos = ChaosEngine::inject_fault(
        ChaosFaultType::DnsFailure,
        &sandbox.namespace,
        "coredns",
    );

    assert_eq!(chaos.fault_type, ChaosFaultType::DnsFailure);
    assert!(chaos.active);

    // 3. Destroy sandbox
    provisioner
        .destroy_sandbox(&sandbox.namespace)
        .await
        .expect("Sandbox should be destroyed cleanly");
}
