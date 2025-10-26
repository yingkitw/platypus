//! WebSocket handler for real-time communication.

use axum::extract::ws::{WebSocket, WebSocketUpgrade, Message};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use chatapp_runtime::SessionStore;
use crate::message;
use crate::executor::ScriptExecutor;

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

    // Create executor for script execution
    let executor = ScriptExecutor::new(session_store.clone());

    // Send initial session message using proto
    let init_msg = message::create_session_msg(&session_id.to_string(), "");
    if let Ok(bytes) = message::serialize_forward_msg(&init_msg) {
        let _ = sender.send(Message::Binary(bytes)).await;
    }

    // Execute initial script and send deltas
    match executor.execute_script(session_id) {
        Ok(deltas) => {
            let delta_msg = message::create_delta_msg(deltas);
            if let Ok(bytes) = message::serialize_forward_msg(&delta_msg) {
                let _ = sender.send(Message::Binary(bytes)).await;
            }
        }
        Err(e) => {
            tracing::error!("Initial script execution error: {}", e);
        }
    }

    // Handle incoming messages
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Binary(data)) => {
                tracing::debug!("Received binary message: {} bytes", data.len());
                
                // Try to deserialize as BackMsg
                match message::deserialize_back_msg(&data) {
                    Ok(back_msg) => {
                        tracing::debug!("Parsed BackMsg from session: {}", back_msg.session_id);
                        // Handle the message based on its type
                        if let Some(msg_type) = back_msg.r#type {
                            match msg_type {
                                chatapp_proto::back_msg::Type::WidgetStateChange(widget_change) => {
                                    let widget_key = &widget_change.widget_key;
                                    tracing::debug!("Widget state change: {}", widget_key);
                                    
                                    // Handle widget change and rerun script
                                    match executor.handle_widget_change(
                                        session_id,
                                        widget_key,
                                        &widget_change.value,
                                    ) {
                                        Ok(deltas) => {
                                            let delta_msg = message::create_delta_msg(deltas);
                                            if let Ok(bytes) = message::serialize_forward_msg(&delta_msg) {
                                                let _ = sender.send(Message::Binary(bytes)).await;
                                            }
                                        }
                                        Err(e) => {
                                            tracing::error!("Script execution error: {}", e);
                                        }
                                    }
                                }
                                chatapp_proto::back_msg::Type::RerunScript(_) => {
                                    tracing::debug!("Script rerun requested");
                                    
                                    // Rerun script
                                    match executor.execute_script(session_id) {
                                        Ok(deltas) => {
                                            let delta_msg = message::create_delta_msg(deltas);
                                            if let Ok(bytes) = message::serialize_forward_msg(&delta_msg) {
                                                let _ = sender.send(Message::Binary(bytes)).await;
                                            }
                                        }
                                        Err(e) => {
                                            tracing::error!("Script execution error: {}", e);
                                        }
                                    }
                                }
                                chatapp_proto::back_msg::Type::UserInteraction(interaction) => {
                                    tracing::debug!("User interaction: {}", interaction.interaction_type);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        tracing::warn!("Failed to deserialize BackMsg: {}", e);
                    }
                }
            }
            Ok(Message::Text(text)) => {
                tracing::debug!("Received text message: {}", text);
                // For backward compatibility, handle JSON messages
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
