//! HTTP request handlers.

use axum::{
    extract::State,
    response::{Html, IntoResponse},
    Json,
    http::StatusCode,
};
use serde_json::json;
use std::sync::Arc;

use crate::server::ServerState;

/// Health check endpoint.
pub async fn health() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

/// Get app info.
pub async fn app_info(State(state): State<Arc<ServerState>>) -> Json<serde_json::Value> {
    let session_count = state.session_store.session_count();
    
    Json(json!({
        "name": &state.config.app_name,
        "sessions": session_count,
        "uptime": state.start_time.elapsed().as_secs()
    }))
}

/// Serve the main app page.
pub async fn index() -> Html<&'static str> {
    Html(include_str!("../frontend/index.html"))
}

/// Serve favicon.
pub async fn favicon() -> impl IntoResponse {
    // Simple 1x1 transparent PNG favicon
    let favicon_bytes = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48,
        0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00,
        0x00, 0x1F, 0x15, 0xC4, 0x89, 0x00, 0x00, 0x00, 0x0A, 0x49, 0x44, 0x41, 0x54, 0x78,
        0x9C, 0x63, 0x00, 0x01, 0x00, 0x00, 0x05, 0x00, 0x01, 0x0D, 0x0A, 0x2D, 0xB4, 0x00,
        0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ];
    
    (
        StatusCode::OK,
        [("Content-Type", "image/png")],
        favicon_bytes,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health() {
        let response = health().await;
        let json = response.0;
        assert_eq!(json["status"], "ok");
    }
}
