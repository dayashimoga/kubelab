use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use kubelab_api::routes::create_routes;
use kubelab_api::state::AppState;
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn test_server_side_admission_controller_rejections() {
    let state = AppState::new("secret-key-32-chars-minimum-length!".to_string());
    let app = create_routes(state.clone());

    // 1. Start a session first
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

    // 2. Attempt to apply privileged container -> MUST FAIL with 422 UNPROCESSABLE_ENTITY
    let privileged_yaml = json!({
        "yaml_content": "apiVersion: v1\nkind: Pod\nmetadata:\n  name: root-pod\nspec:\n  containers:\n  - name: exploit\n    image: alpine\n    securityContext:\n      privileged: true"
    });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/v1/labs/sessions/{}/apply", session_id))
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&privileged_yaml).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    // 3. Attempt to apply hostPath mount -> MUST FAIL with 422 UNPROCESSABLE_ENTITY
    let hostpath_yaml = json!({
        "yaml_content": "apiVersion: v1\nkind: Pod\nmetadata:\n  name: hostpath-pod\nspec:\n  volumes:\n  - name: host\n    hostPath:\n      path: /etc"
    });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/v1/labs/sessions/{}/apply", session_id))
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&hostpath_yaml).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    // 4. Attempt to target kube-system namespace -> MUST FAIL with 422 UNPROCESSABLE_ENTITY
    let kube_system_yaml = json!({
        "yaml_content": "apiVersion: v1\nkind: ConfigMap\nmetadata:\n  name: hijack\n  namespace: kube-system\ndata:\n  key: val"
    });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/v1/labs/sessions/{}/apply", session_id))
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&kube_system_yaml).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    // 5. Valid unprivileged pod -> MUST PASS with 200 OK
    let valid_yaml = json!({
        "yaml_content": "apiVersion: v1\nkind: Pod\nmetadata:\n  name: safe-web\nspec:\n  containers:\n  - name: nginx\n    image: nginx:alpine\n    ports:\n    - containerPort: 80"
    });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/v1/labs/sessions/{}/apply", session_id))
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&valid_yaml).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
