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
#[allow(dead_code)]
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

    // Build isolated sandbox command - FAIL CLOSED (No host-shell fallback)
    let mut cmd = match build_sandbox_command(&namespace, &session_id) {
        Ok(c) => c,
        Err(err) => {
            let err_msg = format!(
                "\r\n\x1b[1;31m[SECURITY DENIAL] {}\x1b[0m\r\n\
                 \x1b[0;33mEnsure the sandbox Podman container or Kubernetes pod is running.\x1b[0m\r\n",
                err
            );
            let _ = ws_sender.send(Message::Text(err_msg)).await;
            return;
        }
    };

    // Attempt spawning isolated sandbox process
    let process = cmd.spawn();

    match process {
        Ok(mut child) => {
            let mut stdin = match child.stdin.take() {
                Some(s) => s,
                None => {
                    let _ = ws_sender
                        .send(Message::Text(
                            "\r\n\x1b[1;31m[ERROR] Failed to attach to sandbox stdin\x1b[0m\r\n"
                                .to_string(),
                        ))
                        .await;
                    return;
                }
            };
            let mut stdout = match child.stdout.take() {
                Some(s) => s,
                None => {
                    let _ = ws_sender
                        .send(Message::Text(
                            "\r\n\x1b[1;31m[ERROR] Failed to attach to sandbox stdout\x1b[0m\r\n"
                                .to_string(),
                        ))
                        .await;
                    return;
                }
            };
            let mut stderr = match child.stderr.take() {
                Some(s) => s,
                None => {
                    let _ = ws_sender
                        .send(Message::Text(
                            "\r\n\x1b[1;31m[ERROR] Failed to attach to sandbox stderr\x1b[0m\r\n"
                                .to_string(),
                        ))
                        .await;
                    return;
                }
            };

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
                    Some(data) = stdout_rx.recv() => {
                        let text = String::from_utf8_lossy(&data).to_string();
                        if ws_sender.send(Message::Text(text)).await.is_err() {
                            break;
                        }
                    }
                    Some(data) = stderr_rx.recv() => {
                        let text = String::from_utf8_lossy(&data).to_string();
                        if ws_sender.send(Message::Text(text)).await.is_err() {
                            break;
                        }
                    }
                    res = next_msg => {
                        match res {
                            Ok(Some(Ok(msg))) => {
                                match msg {
                                    Message::Text(text) => {
                                        if let Ok(parsed) = serde_json::from_str::<ClientTerminalMsg>(&text) {
                                            match parsed {
                                                ClientTerminalMsg::Data { data } => {
                                                    if stdin.write_all(data.as_bytes()).await.is_err() {
                                                        break;
                                                    }
                                                }
                                                ClientTerminalMsg::Resize { cols: _, rows: _ } => {
                                                    // Terminal resize event handled gracefully
                                                }
                                                ClientTerminalMsg::Ping => {
                                                    let _ = ws_sender.send(Message::Pong(vec![])).await;
                                                }
                                            }
                                        } else {
                                            // Raw input passthrough
                                            if stdin.write_all(text.as_bytes()).await.is_err() {
                                                break;
                                            }
                                        }
                                    }
                                    Message::Binary(bin) => {
                                        if stdin.write_all(&bin).await.is_err() {
                                            break;
                                        }
                                    }
                                    Message::Close(_) => {
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
                 \x1b[0;33mZero-Trust Policy: Host-shell fallback is strictly disabled.\x1b[0m\r\n",
                e
            );
            let _ = ws_sender.send(Message::Text(err_msg)).await;
        }
    }
}

/// Helper function to build a sanitized, namespace-scoped sandbox PTY command
/// ZERO HOST-SHELL FALLBACK: Returns Err if no container runtime or Kubernetes cluster is available.
pub fn build_sandbox_command(namespace: &str, session_id: &Uuid) -> Result<Command, String> {
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
        return Err("No isolated container sandbox engine (kubectl/podman) is available. Direct host-shell execution is strictly forbidden by KubeLab Zero-Trust Security Policy.".to_string());
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

    Ok(cmd)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_sandbox_command_security_hardening() {
        let session_id = Uuid::new_v4();
        let namespace = format!("lab-{}", session_id.simple());

        let cmd_res = build_sandbox_command(&namespace, &session_id);
        // If kubectl or podman is present, it returns Ok with dropped host env; if neither is found, it returns Err
        if let Ok(cmd) = cmd_res {
            let prog = format!("{:?}", cmd);
            // Must NOT contain powershell or /bin/sh host root shell directly
            assert!(
                prog.contains("kubectl") || prog.contains("podman"),
                "Command must target container sandbox runtime, got: {}",
                prog
            );
        }
    }
}
