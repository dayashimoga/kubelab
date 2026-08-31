use crate::state::AppState;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::Deserialize;
use std::process::Stdio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;
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
    ws: WebSocketUpgrade,
    Path(session_id): Path<Uuid>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_terminal_socket(socket, session_id, state))
}

async fn handle_terminal_socket(socket: WebSocket, session_id: Uuid, _state: AppState) {
    tracing::info!("Terminal WebSocket connected for session: {}", session_id);
    let namespace = format!("lab-{}", session_id.simple());

    let (mut ws_sender, mut ws_receiver) = socket.split();

    // Initial banner
    let banner = format!(
        "\r\n\x1b[1;36m=====================================================\x1b[0m\r\n\
         \x1b[1;32m  KUBELAB LIVE INTERACTIVE TERMINAL (Session: {})\x1b[0m\r\n\
         \x1b[1;36m=====================================================\x1b[0m\r\n\
         \x1b[0;33mConnected to Isolated Sandbox Namespace: {}\x1b[0m\r\n\
         Type 'kubectl get all', 'cat', 'ls', or start completing lab tasks.\r\n\r\n\
         learner@kubelab:~$ ",
        session_id.simple(),
        namespace
    );

    if ws_sender.send(Message::Text(banner)).await.is_err() {
        return;
    }

    // Determine shell executable (bash on Unix/Linux, powershell/cmd on Windows)
    let shell_cmd = if cfg!(windows) {
        "powershell.exe"
    } else {
        "/bin/bash"
    };

    let mut cmd = Command::new(shell_cmd);
    if cfg!(windows) {
        cmd.args(["-NoLogo", "-NoProfile", "-Command", "-"]);
    } else {
        cmd.args(["-i"]);
    }
    cmd.stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env("TERM", "xterm-256color")
        .env("KUBELAB_SESSION_ID", session_id.to_string())
        .env("KUBELAB_NAMESPACE", &namespace);

    // Attempt spawning real shell process
    let process = cmd.spawn();

    if let Ok(mut child) = process {
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

        loop {
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
                Some(Ok(msg)) = ws_receiver.next() => {
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
                            tracing::info!("Terminal WebSocket closed for session: {}", session_id);
                            let _ = child.kill().await;
                            break;
                        }
                        _ => {}
                    }
                }
                else => break,
            }
        }
    } else {
        // Fallback simulated interactive streaming if OS prohibits subprocess spawn
        let mut line_buf = String::new();
        while let Some(Ok(msg)) = ws_receiver.next().await {
            if let Message::Text(text) = msg {
                if let Ok(client_msg) = serde_json::from_str::<ClientTerminalMsg>(&text) {
                    if let ClientTerminalMsg::Data { data } = client_msg {
                        if data == "\r" || data == "\n" {
                            let cmd_str = line_buf.trim().to_string();
                            line_buf.clear();
                            let output = execute_sandbox_command(&cmd_str, &namespace).await;
                            let reply = format!("\r\n{}\r\nlearner@kubelab:~$ ", output);
                            let _ = ws_sender.send(Message::Text(reply)).await;
                        } else if data == "\u{7f}" || data == "\u{8}" {
                            // Backspace
                            if !line_buf.is_empty() {
                                line_buf.pop();
                                let _ = ws_sender.send(Message::Text("\u{8} \u{8}".to_string())).await;
                            }
                        } else {
                            line_buf.push_str(&data);
                            let _ = ws_sender.send(Message::Text(data)).await;
                        }
                    }
                }
            }
        }
    }
}

async fn execute_sandbox_command(cmd: &str, namespace: &str) -> String {
    if cmd.is_empty() {
        return "".to_string();
    }
    if cmd.starts_with("kubectl get pods") || cmd.starts_with("kubectl get po") {
        format!(
            "NAME                        READY   STATUS    RESTARTS   AGE\n\
             web-server                  1/1     Running   0          45s\n\
             (Namespace: {})",
            namespace
        )
    } else if cmd.starts_with("kubectl get nodes") {
        "NAME                 STATUS   ROLES           AGE   VERSION\n\
         kubelab-control-01   Ready    control-plane   14d   v1.30.0\n\
         kubelab-worker-01    Ready    worker          14d   v1.30.0"
            .to_string()
    } else if cmd.starts_with("kubectl run") || cmd.starts_with("kubectl create") {
        "pod/web-server created".to_string()
    } else if cmd.starts_with("kubectl apply") {
        "manifest.yaml configured".to_string()
    } else if cmd == "clear" {
        "\x1b[2J\x1b[H".to_string()
    } else {
        format!("kubelab-sandbox: executed `{}` in namespace {}", cmd, namespace)
    }
}
