use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use kubelab_api::routes::create_routes;
use kubelab_api::state::AppState;
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn test_registration_input_validation_guards() {
    let state = AppState::new("secret-key-32-chars-minimum-length!".to_string());
    let app = create_routes(state.clone());

    // 1. Empty name should be rejected
    let empty_name_req = json!({
        "email": "valid@kubelab.io",
        "name": "   ",
        "password": "ValidPassword123!"
    });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&empty_name_req).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let res: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(res["error"], "Name cannot be empty");

    // 2. Invalid email format should be rejected
    let invalid_email_req = json!({
        "email": "not-an-email",
        "name": "Valid Name",
        "password": "ValidPassword123!"
    });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&invalid_email_req).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let res: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(res["error"], "Invalid email address format");

    // 3. Short password (< 8 chars) should be rejected
    let short_password_req = json!({
        "email": "valid@kubelab.io",
        "name": "Valid Name",
        "password": "short"
    });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&short_password_req).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let res: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(res["error"], "Password must be at least 8 characters long");

    // 4. Valid inputs succeed with 201 Created
    let valid_req = json!({
        "email": "legit.engineer@kubelab.io",
        "name": "Legit Engineer",
        "password": "SuperSecurePassword2026!"
    });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&valid_req).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
}
