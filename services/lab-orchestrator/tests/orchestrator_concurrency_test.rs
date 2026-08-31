use kubelab_lab_orchestrator::LabProvisioner;
use uuid::Uuid;

#[tokio::test]
async fn test_concurrent_sandbox_provisioning_and_isolation() {
    let provisioner = LabProvisioner::new();

    let user_a = "user-alice-101";
    let user_b = "user-bob-202";
    let session_a = Uuid::new_v4();
    let session_b = Uuid::new_v4();

    // 1. Provision two independent sandboxes concurrently
    let (sandbox_a, sandbox_b) = tokio::join!(
        provisioner.provision_sandbox(session_a, user_a, 30),
        provisioner.provision_sandbox(session_b, user_b, 45)
    );

    let sb_a = sandbox_a.expect("Sandbox A should provision successfully");
    let sb_b = sandbox_b.expect("Sandbox B should provision successfully");

    assert_ne!(
        sb_a.namespace, sb_b.namespace,
        "Namespaces must be completely unique"
    );
    assert_eq!(sb_a.user_id, user_a);
    assert_eq!(sb_b.user_id, user_b);
    assert!(sb_a.network_policy_applied);
    assert!(sb_b.network_policy_applied);

    // 2. Verify active sandboxes list
    let active = provisioner.list_active_sandboxes().await;
    assert_eq!(active.len(), 2);

    // 3. Destroy sandbox A and verify B remains unaffected
    let destroy_a = provisioner.destroy_sandbox(&sb_a.namespace).await;
    assert!(destroy_a.is_ok());

    let remaining = provisioner.list_active_sandboxes().await;
    assert_eq!(remaining.len(), 1);
    assert_eq!(remaining[0].namespace, sb_b.namespace);

    // 4. Destroying non-existent / already destroyed sandbox should return error
    let double_destroy = provisioner.destroy_sandbox(&sb_a.namespace).await;
    assert!(
        double_destroy.is_err(),
        "Idempotent teardown check: non-existent sandbox must error"
    );

    // 5. Cleanup sandbox B
    let destroy_b = provisioner.destroy_sandbox(&sb_b.namespace).await;
    assert!(destroy_b.is_ok());
    assert_eq!(provisioner.list_active_sandboxes().await.len(), 0);
}
