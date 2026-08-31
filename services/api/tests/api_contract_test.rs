use axum::body::Body;
use axum::http::Request;
use axum::http::StatusCode;
use http_body_util::BodyExt;
use kubelab_api::routes::create_routes;
use kubelab_api::state::AppState;
use serde_json::json;
use tower::ServiceExt; // for `oneshot` // for `collect`

#[tokio::test]
async fn test_full_api_contract_and_end_to_end_flow() {
    let state = AppState::new("test-secret-key-32-characters-long!".to_string());
    let app = create_routes(state);

    // 1. Health checks
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/readyz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // 2. Prometheus metrics
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/metrics")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("kubelab_http_requests_total"));

    // 3. User Registration
    let reg_payload = json!({
        "email": "e2e.learner@kubelab.io",
        "name": "E2E Learner",
        "password": "SecurePassword123!",
        "role": "learner"
    });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&reg_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let reg_res: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let token = reg_res["tokens"]["access_token"].as_str().unwrap();

    // 4. Authenticated /me check
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/auth/me")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // 5. Track listing (12 tracks)
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/v1/tracks")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let tracks: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(tracks.as_array().unwrap().len(), 12);

    // 6. Start Lab Session
    let start_payload = json!({ "lab_id": "k8s-pod-basics" });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/labs/start")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&start_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let session_res: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let session_id = session_res["id"].as_str().unwrap();

    // 7. Apply YAML manifest to Sandbox
    let yaml_payload = json!({
        "yaml_content": "apiVersion: v1\nkind: Pod\nmetadata:\n  name: web-server\n  labels:\n    app: frontend\nspec:\n  containers:\n  - name: nginx\n    image: nginx:alpine\n    ports:\n    - containerPort: 80"
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

    // 8. Get sandbox live resources
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
    assert!(!resources.as_array().unwrap().is_empty());

    // 9. Deterministic state validation & grading
    let val_payload = json!({
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
                .body(Body::from(serde_json::to_vec(&val_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let val_res: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(val_res["passed"], true);
    assert_eq!(val_res["score"], 50);

    // 10. AI Tutor query
    let tutor_payload = json!({
        "mode": "explain",
        "user_prompt": "Kubernetes Pod Lifecycle"
    });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/ai-tutor/query")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&tutor_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
