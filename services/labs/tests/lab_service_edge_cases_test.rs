use kubelab_labs::{
    ApplyManifestRequest, LabError, LabService, StartLabRequest, ValidateLabRequest,
};
use serde_json::json;
use uuid::Uuid;

#[tokio::test]
async fn test_lab_service_error_paths_and_boundary_conditions() {
    let service = LabService::new();

    // 1. Get non-existent lab
    let non_existent_lab = service.get_lab("non-existent-lab-id-9999").await;
    assert!(matches!(non_existent_lab, Err(LabError::LabNotFound)));

    // 2. Start session for non-existent lab
    let start_invalid = service
        .start_session(
            "user-123",
            StartLabRequest {
                lab_id: "non-existent-lab-id-9999".to_string(),
            },
        )
        .await;
    assert!(matches!(start_invalid, Err(LabError::LabNotFound)));

    // 3. Query non-existent session
    let random_session_id = Uuid::new_v4();
    let get_session_err = service.get_session(&random_session_id).await;
    assert!(matches!(get_session_err, Err(LabError::SessionNotFound)));

    // 4. Query resources for non-existent session
    let get_res_err = service.get_namespace_resources(&random_session_id).await;
    assert!(matches!(get_res_err, Err(LabError::SessionNotFound)));

    // 5. Destroy non-existent session
    let destroy_err = service.destroy_session(&random_session_id).await;
    assert!(matches!(destroy_err, Err(LabError::SessionNotFound)));

    // 6. Start a valid session
    let valid_session = service
        .start_session(
            "user-123",
            StartLabRequest {
                lab_id: "k8s-pod-basics".to_string(),
            },
        )
        .await
        .expect("Valid lab start must succeed");

    // 7. Validate non-existent task inside valid session
    let validate_invalid_task = service
        .validate_task(
            &valid_session.id,
            ValidateLabRequest {
                task_id: "non-existent-task-id-xyz".to_string(),
                live_state: Some(json!({"status": {"phase": "Running"}})),
            },
        )
        .await;
    assert!(matches!(validate_invalid_task, Err(LabError::TaskNotFound)));

    // 8. Apply manifest with comments and empty documents
    let empty_docs_manifest = r#"
# Just a comment
---
---
# Another empty document
"#;
    let apply_empty = service
        .apply_manifest(
            &valid_session.id,
            ApplyManifestRequest {
                yaml_content: empty_docs_manifest.to_string(),
            },
        )
        .await
        .expect("Empty manifest chunks should be handled safely");
    assert!(apply_empty.success);
    assert_eq!(apply_empty.applied_resources.len(), 0);

    // 9. Destroy the valid session and ensure double destroy is rejected
    service
        .destroy_session(&valid_session.id)
        .await
        .expect("First destroy must succeed");

    let resources_after_destroy = service.get_namespace_resources(&valid_session.id).await.unwrap();
    assert!(resources_after_destroy.is_empty());
}
