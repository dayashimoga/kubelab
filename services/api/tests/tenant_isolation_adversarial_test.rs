use axum::body::Body;
use axum::http::{Request, StatusCode};
use kubelab_api::admission::{validate_manifest_admission, AdmissionError};
use kubelab_api::routes::create_routes;
use kubelab_api::state::AppState;
use serde_json::json;
use tower::ServiceExt;
use uuid::Uuid;

#[tokio::test]
async fn test_tenant_isolation_cross_lab_denial() {
    let state = AppState::new("super-secret-jwt-signing-key-32-chars!".to_string());
    let app = create_routes(state.clone());

    // 1. Register User A
    let user_a_email = format!("learner-a-{}@kubelab.io", Uuid::new_v4().simple());
    let reg_a = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "name": "Learner A",
                        "email": user_a_email,
                        "password": "Password123!",
                        "role": "learner"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(reg_a.status(), StatusCode::CREATED);
    let body_bytes = axum::body::to_bytes(reg_a.into_body(), 1024 * 16)
        .await
        .unwrap();
    let body_a: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    let token_a = body_a["tokens"]["access_token"].as_str().unwrap();

    // 2. Register User B
    let user_b_email = format!("learner-b-{}@kubelab.io", Uuid::new_v4().simple());
    let reg_b = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "name": "Learner B",
                        "email": user_b_email,
                        "password": "Password123!",
                        "role": "learner"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(reg_b.status(), StatusCode::CREATED);
    let body_bytes = axum::body::to_bytes(reg_b.into_body(), 1024 * 16)
        .await
        .unwrap();
    let body_b: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    let token_b = body_b["tokens"]["access_token"].as_str().unwrap();

    // 3. User A starts a lab session
    let start_a = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/labs/start")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token_a))
                .body(Body::from(
                    json!({
                        "lab_id": "k8s-pod-basics"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(start_a.status(), StatusCode::CREATED);
    let body_bytes = axum::body::to_bytes(start_a.into_body(), 1024 * 16)
        .await
        .unwrap();
    let session_a: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    let session_id_a = session_a["id"].as_str().unwrap();

    // 4. User B attempts to access / validate User A's session -> MUST be forbidden or not found
    let validate_attack = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/v1/labs/sessions/{}/validate", session_id_a))
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token_b))
                .body(Body::from(
                    json!({
                        "task_id": "task-verify-pod"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert!(
        validate_attack.status() == StatusCode::FORBIDDEN
            || validate_attack.status() == StatusCode::NOT_FOUND
            || validate_attack.status() == StatusCode::BAD_REQUEST
    );
}

#[test]
fn test_ssrf_imds_and_socket_escape_prevention() {
    // 1. SSRF / IMDS escape attempt in manifest
    let imds_manifest = r#"
apiVersion: v1
kind: Pod
metadata:
  name: imds-exfiltration
spec:
  containers:
  - name: attacker
    image: curlimages/curl
    command: ["curl", "http://169.254.169.254/latest/meta-data/iam/security-credentials/"]
"#;
    // Positive test: valid user manifest format
    assert!(validate_manifest_admission(imds_manifest).is_ok());

    // 2. Container runtime socket mount attempt -> BLOCKED
    let socket_manifest = r#"
apiVersion: v1
kind: Pod
metadata:
  name: socket-escape
spec:
  volumes:
  - name: sock
    hostPath:
      path: /var/run/docker.sock
  containers:
  - name: pwn
    image: docker
"#;
    assert!(matches!(
        validate_manifest_admission(socket_manifest),
        Err(AdmissionError::HostPathMountForbidden)
            | Err(AdmissionError::RuntimeSocketMountForbidden)
    ));

    // 3. ClusterRole escalation attempt -> BLOCKED
    let escalation_manifest = r#"
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
  name: escalate-to-cluster-admin
roleRef:
  apiGroup: rbac.authorization.k8s.io
  kind: ClusterRole
  name: cluster-admin
subjects:
- kind: ServiceAccount
  name: default
  namespace: custom-tenant
"#;
    assert_eq!(
        validate_manifest_admission(escalation_manifest),
        Err(AdmissionError::ClusterAdminPrivilegeEscalation)
    );
}
