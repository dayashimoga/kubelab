use axum::body::Body;
use axum::http::{Request, StatusCode};
use kubelab_api::routes::create_routes;
use kubelab_api::state::AppState;
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn test_security_adversarial_attacks_and_hardening() {
    let state = AppState::new("super-secret-jwt-signing-key-32-chars!".to_string());
    let app = create_routes(state);

    // 1. Unauthenticated request to protected /v1/auth/me MUST fail with 401
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/auth/me")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    // 2. Forged / Tampered JWT token MUST fail with 401
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/auth/me")
                .header(
                    "authorization",
                    "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJmb3JnZWQtYWRtaW4iLCJyb2xlIjoiYWRtaW4ifQ.invalid_signature",
                )
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    // 3. Malformed authorization header format MUST fail with 401
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/auth/me")
                .header("authorization", "Basic dXNlcjpwYXNz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    // 4. SQL Injection payload in login input MUST fail gracefully without crash
    let sqli_payload = json!({
        "email": "admin' OR 1=1; DROP TABLE users; --",
        "password": "' OR '1'='1"
    });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&sqli_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    // 5. Path Traversal attack in Lab ID parameter MUST return 404
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/labs/..%2F..%2Fetc%2Fpasswd")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // 6. Accessing non-existent lab session MUST return 404
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/labs/sessions/00000000-0000-0000-0000-000000000000")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
