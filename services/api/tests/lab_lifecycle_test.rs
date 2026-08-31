use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use kubelab_api::routes::create_routes;
use kubelab_api::state::AppState;
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn test_full_lab_lifecycle_start_apply_validate_destroy() {
    let state = AppState::new("secret-key-32-chars-minimum-length!".to_string());
    let app = create_routes(state.clone());

    // 1. List labs
    let response = app
        .clone()
        .oneshot(Request::builder().uri("/v1/labs").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let labs: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(!labs.as_array().unwrap().is_empty());

    // 2. Get specific lab definition
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/v1/labs/k8s-pod-basics")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // 3. Start lab session
    let start_req = json!({ "lab_id": "k8s-pod-basics" });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/labs/start")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&start_req).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let session: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let session_id = session["id"].as_str().unwrap();
    assert_eq!(session["status"], "ready");
    assert!(session["namespace"].as_str().unwrap().starts_with("lab-"));

    // 4. Apply multi-document YAML manifest to sandbox
    let yaml_payload = json!({
        "yaml_content": "apiVersion: v1\nkind: Pod\nmetadata:\n  name: web-server\n  labels:\n    app: frontend\nspec:\n  containers:\n  - name: nginx\n    image: nginx:alpine\n    ports:\n    - containerPort: 80\n---\napiVersion: v1\nkind: Service\nmetadata:\n  name: web-svc\nspec:\n  ports:\n  - port: 80\n    targetPort: 80"
    });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/v1/labs/sessions/{}/apply", session_id))
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&yaml_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let apply_res: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(apply_res["applied_resources"].as_array().unwrap().len(), 2);

    // 5. Query live sandbox resources
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/v1/labs/sessions/{}/resources", session_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let resources: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(resources.as_array().unwrap().len(), 2);

    // 6. Validate Task 1 (Pod Running with label app=frontend)
    let val_task1 = json!({
        "task_id": "task-deploy-pod",
        "live_state": {
            "status": { "phase": "Running" },
            "metadata": { "labels": { "app": "frontend" } }
        }
    });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/v1/labs/sessions/{}/validate", session_id))
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&val_task1).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let val1_res: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(val1_res["passed"], true);

    // 7. Validate Task 2 (Container port 80)
    let val_task2 = json!({
        "task_id": "task-verify-port",
        "live_state": {
            "spec": {
                "containers": [
                    { "ports": [{ "containerPort": 80 }] }
                ]
            }
        }
    });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/v1/labs/sessions/{}/validate", session_id))
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&val_task2).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let val2_res: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(val2_res["passed"], true);

    // 8. Verify session status is Completed with 100 points
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/v1/labs/sessions/{}", session_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let final_session: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(final_session["status"], "completed");
    assert_eq!(final_session["score"], 100);

    // 9. Destroy and clean up session
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/v1/labs/sessions/{}/destroy", session_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // 10. Verify destroyed session status
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/v1/labs/sessions/{}", session_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let destroyed_session: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(destroyed_session["status"], "destroyed");
}
