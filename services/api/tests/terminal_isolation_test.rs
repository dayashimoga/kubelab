use chrono::Utc;
use kubelab_api::routes::create_routes;
use kubelab_api::state::AppState;
use kubelab_auth::models::{User, UserRole};
use serde_json::json;
use tokio_tungstenite::connect_async;
use uuid::Uuid;

#[tokio::test]
async fn test_terminal_websocket_auth_and_session_isolation() {
    let secret = "secret-key-32-chars-minimum-length!".to_string();
    let state = AppState::new(secret.clone());
    let app = create_routes(state.clone());

    // Bind real TCP test listener on an ephemeral port
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let local_addr = listener.local_addr().unwrap();
    let server_task = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    // 1. Generate valid JWT tokens for User A and User B
    let user_a_id = Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap();
    let user_a = User {
        id: user_a_id,
        email: "user-a@kubelab.io".to_string(),
        name: "User A".to_string(),
        password_hash: "hash".to_string(),
        role: UserRole::Learner,
        avatar_url: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    let (token_user_a, _) = state.auth.jwt().generate_token(&user_a).unwrap();

    let user_b_id = Uuid::parse_str("22222222-2222-2222-2222-222222222222").unwrap();
    let user_b = User {
        id: user_b_id,
        email: "user-b@kubelab.io".to_string(),
        name: "User B".to_string(),
        password_hash: "hash".to_string(),
        role: UserRole::Learner,
        avatar_url: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    let (token_user_b, _) = state.auth.jwt().generate_token(&user_b).unwrap();

    // 2. Start a lab session for User A
    let start_req = json!({ "lab_id": "k8s-pod-basics" });
    let session_res = state
        .labs
        .start_session(
            &user_a_id.to_string(),
            serde_json::from_value(start_req).unwrap(),
        )
        .await
        .unwrap();
    let session_a_id = session_res.id;

    // 3. Connect to terminal WITHOUT token -> MUST FAIL
    let ws_url_no_token = format!("ws://{}/v1/ws/terminal/{}", local_addr, session_a_id);
    let connect_res = connect_async(&ws_url_no_token).await;
    assert!(connect_res.is_err(), "Expected connection without token to fail");

    // 4. Connect to terminal with INVALID JWT -> MUST FAIL
    let ws_url_invalid_token = format!(
        "ws://{}/v1/ws/terminal/{}?token=invalid.jwt.token",
        local_addr, session_a_id
    );
    let connect_res = connect_async(&ws_url_invalid_token).await;
    assert!(connect_res.is_err(), "Expected connection with invalid token to fail");

    // 5. User B tries to connect to User A's terminal session -> MUST FAIL (403 Forbidden)
    let ws_url_user_b = format!(
        "ws://{}/v1/ws/terminal/{}?token={}",
        local_addr, session_a_id, token_user_b
    );
    let connect_res = connect_async(&ws_url_user_b).await;
    assert!(
        connect_res.is_err(),
        "Expected cross-user terminal connection to be rejected"
    );

    // 6. User A connects to their OWN terminal session -> MUST SUCCEED (101 Switching Protocols)
    let ws_url_user_a = format!(
        "ws://{}/v1/ws/terminal/{}?token={}",
        local_addr, session_a_id, token_user_a
    );
    let connect_res = connect_async(&ws_url_user_a).await;
    assert!(
        connect_res.is_ok(),
        "Expected session owner terminal connection to succeed: {:?}",
        connect_res.err()
    );

    let (mut ws_stream, _) = connect_res.unwrap();
    use futures_util::StreamExt;
    if let Some(Ok(msg)) = ws_stream.next().await {
        let text = msg.to_text().unwrap();
        assert!(text.contains("KUBELAB LIVE INTERACTIVE TERMINAL"));
        assert!(text.contains(&format!("lab-{}", session_a_id.simple())));
    }

    server_task.abort();
}
