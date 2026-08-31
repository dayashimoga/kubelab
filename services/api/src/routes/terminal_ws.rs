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
    State(_state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_terminal_socket(socket, session_id))
}

async fn handle_terminal_socket(mut socket: WebSocket, session_id: Uuid) {
    tracing::info!("Terminal WebSocket connected for session: {}", session_id);

    // Initial banner sent to learner xterm.js
    let banner = format!(
        "\r\n\x1b[1;36m=====================================================\x1b[0m\r\n\
         \x1b[1;32m  KUBELAB SECURE SANDBOX SHELL (Session: {})\x1b[0m\r\n\
         \x1b[1;36m=====================================================\x1b[0m\r\n\
         \x1b[0;33mConnected to Kubernetes Sandbox Namespace: lab-{}\x1b[0m\r\n\
         Type 'kubectl get all' or start completing tasks.\r\n\r\n\
         learner@kubelab:~$ ",
        session_id.simple(),
        session_id.simple()
    );

    if socket.send(Message::Text(banner)).await.is_err() {
        return;
    }

    let (mut sender, mut receiver) = socket.split();

    while let Some(Ok(msg)) = receiver.next().await {
        match msg {
            Message::Text(text) => {
                if let Ok(client_msg) = serde_json::from_str::<ClientTerminalMsg>(&text) {
                    match client_msg {
                        ClientTerminalMsg::Data { data } => {
                            if data == "\r" || data == "\n" {
                                let prompt = "\r\nlearner@kubelab:~$ ";
                                let _ = sender.send(Message::Text(prompt.to_string())).await;
                            } else {
                                let _ = sender.send(Message::Text(data)).await;
                            }
                        }
                        ClientTerminalMsg::Resize { cols, rows } => {
                            tracing::debug!("PTY resized to {}x{}", cols, rows);
                        }
                        ClientTerminalMsg::Ping => {
                            let _ = sender.send(Message::Text(r#"{"type":"pong"}"#.to_string())).await;
                        }
                    }
                } else {
                    let _ = sender.send(Message::Text(text)).await;
                }
            }
            Message::Binary(bin) => {
                let _ = sender.send(Message::Binary(bin)).await;
            }
            Message::Close(_) => {
                tracing::info!("Terminal WebSocket closed for session: {}", session_id);
                break;
            }
            _ => {}
        }
    }
}
