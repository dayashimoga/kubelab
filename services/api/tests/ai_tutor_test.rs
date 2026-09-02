use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use kubelab_api::routes::create_routes;
use kubelab_api::state::AppState;
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn test_ai_tutor_all_five_modes_with_contextual_replies() {
    let state = AppState::new("secret-key-32-chars-minimum-length!".to_string());
    let app = create_routes(state.clone());

    // 1. Mode: Explain
    let explain_req = json!({
        "mode": "explain",
        "user_prompt": "Kubernetes Ingress vs Gateway API"
    });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/ai-tutor/query")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&explain_req).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let res: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(
        res["reply_markdown"]
            .as_str()
            .unwrap()
            .contains("AI Socratic Tutor Service")
            || res["reply_markdown"]
                .as_str()
                .unwrap()
                .contains("Kubernetes")
    );
    assert!(!res["suggested_followups"].as_array().unwrap().is_empty());

    // 2. Mode: Socratic
    let socratic_req = json!({
        "mode": "socratic",
        "user_prompt": "Why is my pod in CrashLoopBackOff?"
    });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/ai-tutor/query")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&socratic_req).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let res: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(!res["reply_markdown"].as_str().unwrap().is_empty());
    assert!(!res["suggested_followups"].as_array().unwrap().is_empty());

    // 3. Mode: Hint
    let hint_req = json!({
        "mode": "hint",
        "user_prompt": "I cannot connect to my service endpoint",
        "current_lab_task_id": "task-deploy-pod"
    });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/ai-tutor/query")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&hint_req).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let res: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(!res["reply_markdown"].as_str().unwrap().is_empty());

    // 4. Mode: Diagnose
    let diagnose_req = json!({
        "mode": "diagnose",
        "user_prompt": "Fix pod error",
        "current_error_log": "OOMKilled: container exceeded 512Mi limit"
    });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/ai-tutor/query")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&diagnose_req).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let res: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(!res["reply_markdown"].as_str().unwrap().is_empty());

    // 5. Mode: Review
    let review_req = json!({
        "mode": "review",
        "user_prompt": "apiVersion: apps/v1\nkind: Deployment\nmetadata:\n  name: demo"
    });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/ai-tutor/query")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&review_req).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let res: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(!res["reply_markdown"].as_str().unwrap().is_empty());
}
