//! WebSocket handler for real-time communication.

use axum::extract::ws::{WebSocket, WebSocketUpgrade, Message};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use platypus_runtime::SessionStore;
use crate::message;
use crate::executor::{ScriptExecutor, AppFn};

/// Handle WebSocket upgrade.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    session_store: Arc<SessionStore>,
    app_fn: Option<AppFn>,
) -> impl axum::response::IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, session_store, app_fn))
}

/// Handle WebSocket connection.
async fn handle_socket(socket: WebSocket, session_store: Arc<SessionStore>, app_fn: Option<AppFn>) {
    let (mut sender, mut receiver) = socket.split();

    // Create a new session
    let session_id = session_store.create_session("app".to_string());
    
    tracing::info!("WebSocket connection established: {}", session_id);

    // Create executor for script execution
    let executor = if let Some(app_fn) = app_fn {
        ScriptExecutor::with_app(session_store.clone(), app_fn)
    } else {
        ScriptExecutor::new(session_store.clone())
    };

    // Execute initial script and send deltas
    match executor.execute_script(session_id) {
        Ok(deltas) => {
            let json_msg = message::deltas_to_json(deltas);
            if let Ok(json_str) = serde_json::to_string(&json_msg) {
                let _ = sender.send(Message::Text(json_str)).await;
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
                                platypus_proto::back_msg::Type::WidgetStateChange(widget_change) => {
                                    let widget_key = &widget_change.widget_key;
                                    tracing::debug!("Widget state change: {}", widget_key);
                                    
                                    // Handle widget change and rerun script
                                    match executor.handle_widget_change(
                                        session_id,
                                        widget_key,
                                        &widget_change.value,
                                    ) {
                                        Ok(deltas) => {
                                            let json_msg = message::deltas_to_json(deltas);
                                            if let Ok(json_str) = serde_json::to_string(&json_msg) {
                                                let _ = sender.send(Message::Text(json_str)).await;
                                            }
                                        }
                                        Err(e) => {
                                            tracing::error!("Script execution error: {}", e);
                                        }
                                    }
                                }
                                platypus_proto::back_msg::Type::RerunScript(_) => {
                                    tracing::debug!("Script rerun requested");
                                    
                                    // Rerun script
                                    match executor.execute_script(session_id) {
                                        Ok(deltas) => {
                                            let json_msg = message::deltas_to_json(deltas);
                                            if let Ok(json_str) = serde_json::to_string(&json_msg) {
                                                let _ = sender.send(Message::Text(json_str)).await;
                                            }
                                        }
                                        Err(e) => {
                                            tracing::error!("Script execution error: {}", e);
                                        }
                                    }
                                }
                                platypus_proto::back_msg::Type::UserInteraction(interaction) => {
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
                
                // Parse JSON message from frontend
                if let Ok(msg) = serde_json::from_str::<serde_json::Value>(&text) {
                    if let Some("widget_change") = msg.get("type").and_then(|v| v.as_str()) {
                        if let (Some(key), Some(value)) = (
                            msg.get("key").and_then(|v| v.as_str()),
                            msg.get("value")
                        ) {
                            tracing::debug!("Widget change: {} = {}", key, value);
                            
                            // Convert value to string for storage
                            let value_str = match value {
                                serde_json::Value::String(s) => s.clone(),
                                serde_json::Value::Number(n) => n.to_string(),
                                serde_json::Value::Bool(b) => b.to_string(),
                                _ => value.to_string(),
                            };
                            
                            // Handle widget change and rerun script
                            match executor.handle_widget_change(session_id, key, &value_str) {
                                Ok(deltas) => {
                                    let json_msg = message::deltas_to_json(deltas);
                                    if let Ok(json_str) = serde_json::to_string(&json_msg) {
                                        let _ = sender.send(Message::Text(json_str)).await;
                                    }
                                }
                                Err(e) => {
                                    tracing::error!("Script execution error: {}", e);
                                }
                            }
                        }
                    } else if let Some("button_click") = msg.get("type").and_then(|v| v.as_str()) {
                        if let Some(key) = msg.get("key").and_then(|v| v.as_str()) {
                            tracing::debug!("Button click: {}", key);
                            
                            // Rerun script on button click
                            match executor.execute_script(session_id) {
                                Ok(deltas) => {
                                    let json_msg = message::deltas_to_json(deltas);
                                    if let Ok(json_str) = serde_json::to_string(&json_msg) {
                                        let _ = sender.send(Message::Text(json_str)).await;
                                    }
                                }
                                Err(e) => {
                                    tracing::error!("Script execution error: {}", e);
                                }
                            }
                        }
                    }
                }
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
