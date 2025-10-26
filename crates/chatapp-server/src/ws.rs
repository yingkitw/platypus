//! WebSocket handler for real-time communication.

use crate::error::Result;
use axum::extract::ws::{WebSocket, WebSocketUpgrade, Message};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use chatapp_runtime::SessionStore;

/// Handle WebSocket upgrade.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    session_store: Arc<SessionStore>,
) -> impl axum::response::IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, session_store))
}

/// Handle WebSocket connection.
async fn handle_socket(socket: WebSocket, session_store: Arc<SessionStore>) {
    let (mut sender, mut receiver) = socket.split();

    // Create a new session
    let session_id = session_store.create_session("app".to_string());
    
    tracing::info!("WebSocket connection established: {}", session_id);

    // Send initial session message
    let init_msg = serde_json::json!({
        "type": "session_init",
        "session_id": session_id.to_string()
    });

    if let Ok(msg_str) = serde_json::to_string(&init_msg) {
        let _ = sender.send(Message::Text(msg_str)).await;
    }

    // Handle incoming messages
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                tracing::debug!("Received message: {}", text);
                
                // Echo back for now
                let response = serde_json::json!({
                    "type": "ack",
                    "data": text
                }).to_string();
                let _ = sender.send(Message::Text(response)).await;
            }
            Ok(Message::Close(_)) => {
                tracing::info!("WebSocket closed: {}", session_id);
                let _ = session_store.remove_session(session_id);
                break;
            }
            Err(e) => {
                tracing::error!("WebSocket error: {}", e);
                let _ = session_store.remove_session(session_id);
                break;
            }
            _ => {}
        }
    }
}
