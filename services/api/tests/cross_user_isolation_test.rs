use kubelab_auth::AuthService;
use kubelab_labs::{ApplyManifestRequest, LabService, StartLabRequest};

#[tokio::test]
async fn test_cross_user_session_and_namespace_isolation() {
    let auth = AuthService::new("secret-key-for-cross-user-testing-min-32-chars".to_string());
    let labs = LabService::new();

    // 1. Register User A and User B
    let user_a = auth
        .register("learner.alice@kubelab.io", "Alice Dev", "StrongPassAlice123!", None)
        .await
        .expect("Alice registration should succeed");

    let user_b = auth
        .register("learner.bob@kubelab.io", "Bob Ops", "StrongPassBob456!", None)
        .await
        .expect("Bob registration should succeed");

    assert_ne!(user_a.user.id, user_b.user.id);

    // 2. Alice starts a lab session
    let alice_session = labs
        .start_session(
            &user_a.user.id.to_string(),
            StartLabRequest {
                lab_id: "k8s-pod-basics".to_string(),
            },
        )
        .await
        .expect("Alice should start lab session");

    // 3. Bob starts the same lab definition in his own session
    let bob_session = labs
        .start_session(
            &user_b.user.id.to_string(),
            StartLabRequest {
                lab_id: "k8s-pod-basics".to_string(),
            },
        )
        .await
        .expect("Bob should start lab session");

    // Assert session IDs and namespaces are completely isolated
    assert_ne!(alice_session.id, bob_session.id);
    assert_ne!(alice_session.namespace, bob_session.namespace);
    assert_eq!(alice_session.user_id, user_a.user.id.to_string());
    assert_eq!(bob_session.user_id, user_b.user.id.to_string());

    // 4. Alice applies a manifest to her session
    let alice_manifest = r#"
apiVersion: v1
kind: Pod
metadata:
  name: nginx-alice
spec:
  containers:
  - name: nginx
    image: nginx:1.25-alpine
"#;
    let alice_apply = labs
        .apply_manifest(
            &alice_session.id,
            ApplyManifestRequest {
                yaml_content: alice_manifest.to_string(),
            },
        )
        .await
        .expect("Alice manifest apply should succeed");

    assert!(alice_apply.success);
    assert_eq!(alice_apply.applied_resources.len(), 1);
    assert_eq!(alice_apply.applied_resources[0].namespace, alice_session.namespace);

    // 5. Verify Bob cannot see Alice's deployed resources
    let bob_resources = labs
        .get_namespace_resources(&bob_session.id)
        .await
        .expect("Bob resources query should succeed");
    
    assert!(
        bob_resources.iter().all(|r| r.name != "nginx-alice"),
        "Bob must not have visibility into Alice's sandbox resources"
    );

    // 6. Alice destroys her session
    labs.destroy_session(&alice_session.id)
        .await
        .expect("Alice destroy session should succeed");

    // 7. Verify Alice's session is destroyed but Bob's session remains active
    let alice_session_after = labs.get_session(&alice_session.id).await.unwrap();
    assert_eq!(alice_session_after.status, kubelab_labs::SessionStatus::Destroyed);

    let bob_session_after = labs.get_session(&bob_session.id).await.unwrap();
    assert_eq!(bob_session_after.status, kubelab_labs::SessionStatus::Ready);
}
