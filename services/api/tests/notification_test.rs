use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use kubelab_api::routes::create_routes;
use kubelab_api::state::AppState;
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn test_notification_dispatch_and_retrieval() {
    let state = AppState::new("secret-key-32-chars-minimum-length!".to_string());
    let app = create_routes(state.clone());

    let user_id = "user-notify-test-123";

    // 1. Initial notifications should be empty
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/v1/notifications/{}", user_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let notifs: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(notifs.as_array().unwrap().is_empty());

    // 2. Dispatch a warning notification
    let dispatch_payload = json!({
        "title": "Cluster Resource Alert",
        "message": "Pod memory utilization reached 85% of namespace quota.",
        "severity": "warning"
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/v1/notifications/{}/dispatch", user_id))
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&dispatch_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let created: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(created["title"], "Cluster Resource Alert");
    assert_eq!(created["severity"], "warning");
    assert_eq!(created["read"], false);

    // 3. Dispatch a success badge notification
    let badge_payload = json!({
        "title": "Badge Unlocked: Pod Pilot!",
        "message": "You completed your first Kubernetes deployment lab.",
        "severity": "success"
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/v1/notifications/{}/dispatch", user_id))
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&badge_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // 4. Retrieve all notifications for user
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/v1/notifications/{}", user_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let notifs: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(notifs.as_array().unwrap().len(), 2);
}
