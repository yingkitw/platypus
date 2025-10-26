//! HTTP request handlers.

use axum::{
    extract::State,
    http::StatusCode,
    response::Html,
    Json,
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
