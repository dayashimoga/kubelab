use axum::{
    body::Body,
    http::{header, Request, StatusCode},
};
use kubelab_api::{routes::create_routes, state::AppState};
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn test_endpoint_authorization_matrix_and_rbac() {
    let state = AppState::new("secret-key-for-rbac-testing-min-32-chars-ok".to_string());
    let app = create_routes(state.clone());

    // 1. Public Health Endpoints - Anonymous ALLOW
    let health_resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(health_resp.status(), StatusCode::OK);

    let ready_resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/readyz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(ready_resp.status(), StatusCode::OK);

    // 2. Protected endpoint /v1/auth/me without token - Anonymous DENY (401 Unauthorized)
    let unauth_resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/v1/auth/me")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(unauth_resp.status(), StatusCode::UNAUTHORIZED);

    // 3. Protected endpoint with malformed/forged token - DENY (401 Unauthorized)
    let forged_resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/v1/auth/me")
                .header(header::AUTHORIZATION, "Bearer forged.jwt.token.here")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(forged_resp.status(), StatusCode::UNAUTHORIZED);

    // 4. Register a valid learner and get authentic JWT
    let reg_body = json!({
        "email": "rbac.test@kubelab.io",
        "name": "RBAC Learner",
        "password": "StrongPassword999!",
        "role": "learner"
    });

    let reg_resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/auth/register")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&reg_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(reg_resp.status(), StatusCode::CREATED);

    let body_bytes = axum::body::to_bytes(reg_resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let auth_data: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    let token = auth_data["tokens"]["access_token"].as_str().unwrap();

    // 5. Authenticated request with valid Bearer token - Learner ALLOW
    let me_resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/v1/auth/me")
                .header(header::AUTHORIZATION, format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(me_resp.status(), StatusCode::OK);

    // 6. Logout with valid Bearer token - ALLOW
    let logout_resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/auth/logout")
                .header(header::AUTHORIZATION, format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(logout_resp.status(), StatusCode::OK);
}
