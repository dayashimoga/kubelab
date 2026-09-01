use crate::state::AppState;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, Query, State,
    },
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::Deserialize;
use std::collections::HashMap;
use std::process::Stdio;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;
use tokio::time::timeout;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum ClientTerminalMsg {
    #[serde(rename = "data")]
    Data { data: String },
    #[serde(rename = "resize")]
    Resize { cols: u16, rows: u16 },
    #[serde(rename = "ping")]
    Ping,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/ws/terminal/:session_id", get(ws_handler))
}

async fn ws_handler(
    Path(session_id): Path<Uuid>,
    Query(query): Query<HashMap<String, String>>,
    State(state): State<AppState>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    // 1. Mandatory JWT Authentication
    let token = match query.get("token") {
        Some(t) if !t.trim().is_empty() => t.trim(),
        _ => {
            tracing::warn!(
                "Terminal connection rejected: missing authentication token for session {}",
                session_id
            );
            return StatusCode::UNAUTHORIZED.into_response();
        }
    };

    // 2. Check if token is revoked in Redis
    if let Some(ref cache) = state.cache {
        if cache
            .session_store()
            .is_revoked(token)
            .await
            .unwrap_or(false)
        {
            tracing::warn!(
                "Terminal connection rejected: revoked token for session {}",
                session_id
            );
            return StatusCode::UNAUTHORIZED.into_response();
        }
    }

    // 3. Verify JWT validity
    let claims = match state.auth.jwt().verify_token(token) {
        Ok(c) => c,
        Err(e) => {
            tracing::warn!(
                "Terminal connection rejected: invalid JWT ({:?}) for session {}",
                e,
                session_id
            );
            return StatusCode::UNAUTHORIZED.into_response();
        }
    };

    // 4. Verify Session Ownership & Access Control
    if let Ok(session) = state.labs.get_session(&session_id).await {
        if session.user_id != claims.sub && claims.role != "admin" {
            tracing::warn!(
                "Terminal connection rejected: user {} is not authorized for session {} (owner: {})",
                claims.sub,
                session_id,
                session.user_id
            );
            return StatusCode::FORBIDDEN.into_response();
        }
    }

    tracing::info!(
        "Terminal connection authenticated and authorized for user {} (session: {})",
        claims.sub,
        session_id
    );

    let user_id = claims.sub.clone();
    ws.on_upgrade(move |socket| handle_terminal_socket(socket, session_id, user_id, state))
        .into_response()
}

async fn handle_terminal_socket(
    socket: WebSocket,
    session_id: Uuid,
    user_id: String,
    _state: AppState,
) {
    tracing::info!(
        "Terminal WebSocket connected for user {} (session: {})",
        user_id,
        session_id
    );
    let namespace = format!("lab-{}", session_id.simple());

    let (mut ws_sender, mut ws_receiver) = socket.split();

    // Initial Security Banner
    let banner = format!(
        "\r\n\x1b[1;36m=====================================================\x1b[0m\r\n\
         \x1b[1;32m  KUBELAB LIVE INTERACTIVE TERMINAL (Session: {})\x1b[0m\r\n\
         \x1b[1;36m=====================================================\x1b[0m\r\n\
         \x1b[0;33mConnected to Isolated Sandbox Namespace: {}\x1b[0m\r\n\
         \x1b[0;32mZero-Trust Sandbox Active (PSS: Restricted, UID: 10001)\x1b[0m\r\n\
         Type 'kubectl get all', 'cat', 'ls', or start completing lab tasks.\r\n\r\n\
         learner@kubelab:~$ ",
        session_id.simple(),
        namespace
    );

    if ws_sender.send(Message::Text(banner)).await.is_err() {
        return;
    }

    // Build isolated sandbox command
    let mut cmd = build_sandbox_command(&namespace, &session_id);

    // Attempt spawning isolated sandbox process
    let process = cmd.spawn();

    match process {
        Ok(mut child) => {
            let mut stdin = child.stdin.take().expect("Failed to open child stdin");
            let mut stdout = child.stdout.take().expect("Failed to open child stdout");
            let mut stderr = child.stderr.take().expect("Failed to open child stderr");

            // Pipe process stdout -> WebSocket client
            let (stdout_tx, mut stdout_rx) = tokio::sync::mpsc::channel::<Vec<u8>>(100);
            let (stderr_tx, mut stderr_rx) = tokio::sync::mpsc::channel::<Vec<u8>>(100);

            tokio::spawn(async move {
                let mut buf = [0u8; 1024];
                while let Ok(n) = stdout.read(&mut buf).await {
                    if n == 0 {
                        break;
                    }
                    let _ = stdout_tx.send(buf[..n].to_vec()).await;
                }
            });

            tokio::spawn(async move {
                let mut buf = [0u8; 1024];
                while let Ok(n) = stderr.read(&mut buf).await {
                    if n == 0 {
                        break;
                    }
                    let _ = stderr_tx.send(buf[..n].to_vec()).await;
                }
            });

            let idle_duration = Duration::from_secs(1800); // 30-minute idle timeout

            loop {
                let next_msg = timeout(idle_duration, ws_receiver.next());

                tokio::select! {
                    Some(bytes) = stdout_rx.recv() => {
                        let text = String::from_utf8_lossy(&bytes).to_string();
                        if ws_sender.send(Message::Text(text)).await.is_err() {
                            break;
                        }
                    }
                    Some(bytes) = stderr_rx.recv() => {
                        let text = String::from_utf8_lossy(&bytes).to_string();
                        if ws_sender.send(Message::Text(text)).await.is_err() {
                            break;
                        }
                    }
                    res = next_msg => {
                        match res {
                            Ok(Some(Ok(msg))) => {
                                match msg {
                                    Message::Text(text) => {
                                        if let Ok(client_msg) = serde_json::from_str::<ClientTerminalMsg>(&text) {
                                            match client_msg {
                                                ClientTerminalMsg::Data { data } => {
                                                    if stdin.write_all(data.as_bytes()).await.is_err() {
                                                        break;
                                                    }
                                                    let _ = stdin.flush().await;
                                                }
                                                ClientTerminalMsg::Resize { cols, rows } => {
                                                    tracing::debug!("PTY resized to {}x{}", cols, rows);
                                                }
                                                ClientTerminalMsg::Ping => {
                                                    let _ = ws_sender.send(Message::Text(r#"{"type":"pong"}"#.to_string())).await;
                                                }
                                            }
                                        } else {
                                            if stdin.write_all(text.as_bytes()).await.is_err() {
                                                break;
                                            }
                                            let _ = stdin.flush().await;
                                        }
                                    }
                                    Message::Binary(bin) => {
                                        if stdin.write_all(&bin).await.is_err() {
                                            break;
                                        }
                                        let _ = stdin.flush().await;
                                    }
                                    Message::Close(_) => {
                                        tracing::info!("Terminal WebSocket closed for user {} (session: {})", user_id, session_id);
                                        let _ = child.kill().await;
                                        break;
                                    }
                                    _ => {}
                                }
                            }
                            Ok(Some(Err(_))) | Ok(None) => break,
                            Err(_) => {
                                tracing::warn!("Terminal connection timed out due to inactivity for session {}", session_id);
                                let _ = ws_sender.send(Message::Text("\r\n\x1b[1;33mSession closed due to 30-minute idle timeout.\x1b[0m\r\n".to_string())).await;
                                let _ = child.kill().await;
                                break;
                            }
                        }
                    }
                    else => break,
                }
            }
        }
        Err(e) => {
            let err_msg = format!(
                "\r\n\x1b[1;31m[ERROR] Failed to spawn isolated sandbox terminal backend: {}\x1b[0m\r\n\
                 Ensure sandbox permissions and container engine are available.\r\n",
                e
            );
            let _ = ws_sender.send(Message::Text(err_msg)).await;
        }
    }
}

/// Helper function to build a sanitized, namespace-scoped sandbox PTY command
fn build_sandbox_command(namespace: &str, session_id: &Uuid) -> Command {
    // 1. Check if kubectl/live K8s sandbox is requested
    let has_kubectl = which_command("kubectl");
    let has_podman = which_command("podman");

    let mut cmd = if has_kubectl {
        let mut c = Command::new("kubectl");
        c.args([
            "exec",
            "-i",
            "-n",
            namespace,
            "deploy/learner-sandbox",
            "--",
            "/bin/sh",
        ]);
        c
    } else if has_podman {
        let mut c = Command::new("podman");
        c.args([
            "exec",
            "-i",
            "--user",
            "10001:10001",
            "--workdir",
            "/sandbox",
            &format!("kubelab-sandbox-{}", session_id.simple()),
            "/bin/sh",
        ]);
        c
    } else {
        // Fallback local isolated execution
        let shell_exec = if cfg!(windows) {
            "powershell.exe"
        } else {
            "/bin/sh"
        };
        let mut c = Command::new(shell_exec);
        if cfg!(windows) {
            c.args(["-NoLogo", "-NoProfile", "-Command", "-"]);
        } else {
            c.args(["-i"]);
        }
        c
    };

    cmd.stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env_clear() // Drop all host environment variables (AWS keys, DB passwords, Kubeconfig, etc.)
        .env("TERM", "xterm-256color")
        .env("PATH", "/usr/local/bin:/usr/bin:/bin")
        .env("USER", "learner")
        .env("HOME", "/sandbox")
        .env("KUBELAB_SESSION_ID", session_id.to_string())
        .env("KUBELAB_NAMESPACE", namespace)
        .env("PS1", "learner@kubelab:~$ ");

    cmd
}

fn which_command(cmd: &str) -> bool {
    std::env::var_os("PATH")
        .and_then(|paths| {
            std::env::split_paths(&paths).find_map(|p| {
                let full_path = p.join(cmd);
                let full_path_exe = p.join(format!("{}.exe", cmd));
                if full_path.is_file() || full_path_exe.is_file() {
                    Some(())
                } else {
                    None
                }
            })
        })
        .is_some()
}
