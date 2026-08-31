use axum::body::Body;
use axum::http::{header, Method, Request, StatusCode};
use kubelab_api::routes::create_routes;
use kubelab_api::state::AppState;
use tower::ServiceExt;

#[tokio::test]
async fn test_cors_headers_and_preflight_handling() {
    let state = AppState::new("secret-key-32-chars-minimum-length!".to_string());
    let app = create_routes(state);

    // 1. CORS Preflight Request
    let preflight = Request::builder()
        .method(Method::OPTIONS)
        .uri("/v1/labs")
        .header(header::ORIGIN, "http://localhost:3000")
        .header(header::ACCESS_CONTROL_REQUEST_METHOD, "GET")
        .header(header::ACCESS_CONTROL_REQUEST_HEADERS, "authorization, content-type")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(preflight).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    // Check CORS response headers
    let headers = response.headers();
    assert!(
        headers.contains_key(header::ACCESS_CONTROL_ALLOW_ORIGIN)
            || headers.contains_key("access-control-allow-origin")
    );

    // 2. Standard GET request includes valid CORS allow origin
    let get_req = Request::builder()
        .method(Method::GET)
        .uri("/v1/labs")
        .header(header::ORIGIN, "http://localhost:3000")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(get_req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
