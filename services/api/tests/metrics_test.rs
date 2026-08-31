use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use kubelab_api::metrics::get_metrics;
use kubelab_api::routes::create_routes;
use kubelab_api::state::AppState;
use tower::ServiceExt;

#[tokio::test]
async fn test_prometheus_metrics_exposition_and_registry_counters() {
    let state = AppState::new("secret-key-32-chars-minimum-length!".to_string());
    let app = create_routes(state.clone());

    // 1. Fetch metrics endpoint
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/metrics")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap(),
        "text/plain; version=0.0.4; charset=utf-8"
    );

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8(body.to_vec()).unwrap();

    // 2. Assert Prometheus comments, types, and metric names
    assert!(body_str.contains("# HELP kubelab_http_requests_total"));
    assert!(body_str.contains("# TYPE kubelab_http_requests_total counter"));
    assert!(body_str.contains("# HELP kubelab_active_sessions"));
    assert!(body_str.contains("# TYPE kubelab_active_sessions gauge"));
    assert!(body_str.contains("# HELP kubelab_terminal_ws_connections"));
    assert!(body_str.contains("# TYPE kubelab_terminal_ws_connections gauge"));

    // 3. Dynamically increment metrics in runtime
    let metrics = get_metrics();
    metrics
        .http_requests_total
        .with_label_values(&["kubelab-api", "201"])
        .inc();
    metrics.active_lab_sessions.set(5.0);

    // 4. Fetch metrics again and verify updated values
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/metrics")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(
        body_str.contains(r#"kubelab_http_requests_total{service="kubelab-api",status="201"} 1"#)
    );
    assert!(body_str.contains("kubelab_active_sessions 5"));
}
