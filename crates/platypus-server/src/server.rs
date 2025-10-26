//! Main application server.

use crate::config;
use crate::error::Result;
use crate::executor::AppFn;
use crate::handler;
use crate::ws;
use axum::{
    extract::DefaultBodyLimit,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use platypus_runtime::SessionStore;

/// Server configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Application name.
    pub app_name: String,
    /// Host to bind to.
    pub host: String,
    /// Port to listen on.
    pub port: u16,
    /// Maximum body size (bytes).
    pub max_body_size: u64,
    /// Session timeout (seconds).
    pub session_timeout: u64,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            app_name: config::DEFAULT_APP_NAME.to_string(),
            host: config::DEFAULT_HOST.to_string(),
            port: config::DEFAULT_PORT,
            max_body_size: config::DEFAULT_MAX_BODY_SIZE,
            session_timeout: config::DEFAULT_SESSION_TIMEOUT,
        }
    }
}

/// Server state shared across handlers.
pub struct ServerState {
    /// Server configuration.
    pub config: ServerConfig,
    /// Session store.
    pub session_store: Arc<SessionStore>,
    /// Server start time.
    pub start_time: Instant,
    /// App function.
    pub app_fn: Option<AppFn>,
}

/// Main application server.
pub struct AppServer {
    config: ServerConfig,
    session_store: Arc<SessionStore>,
    app_fn: Option<AppFn>,
}

impl AppServer {
    /// Create a new server with default config.
    pub fn new() -> Self {
        AppServer {
            config: ServerConfig::default(),
            session_store: Arc::new(SessionStore::new()),
            app_fn: None,
        }
    }

    /// Create a new server with custom config.
    pub fn with_config(config: ServerConfig) -> Self {
        AppServer {
            config,
            session_store: Arc::new(SessionStore::new()),
            app_fn: None,
        }
    }

    /// Create a new server with app function.
    pub fn with_app(app_fn: AppFn) -> Self {
        AppServer {
            config: ServerConfig::default(),
            session_store: Arc::new(SessionStore::new()),
            app_fn: Some(app_fn),
        }
    }

    /// Create a new server with custom config and app function.
    pub fn with_config_and_app(config: ServerConfig, app_fn: AppFn) -> Self {
        AppServer {
            config,
            session_store: Arc::new(SessionStore::new()),
            app_fn: Some(app_fn),
        }
    }

    /// Get the server configuration.
    pub fn config(&self) -> &ServerConfig {
        &self.config
    }

    /// Get the session store.
    pub fn session_store(&self) -> &Arc<SessionStore> {
        &self.session_store
    }

    /// Build the router.
    fn build_router(&self) -> Router {
        let state = Arc::new(ServerState {
            config: self.config.clone(),
            session_store: Arc::clone(&self.session_store),
            start_time: Instant::now(),
            app_fn: self.app_fn,
        });

        let session_store = Arc::clone(&self.session_store);
        let app_fn = self.app_fn;

        Router::new()
            // Health check
            .route(config::HEALTH_CHECK_PATH, get(handler::health))
            // App info
            .route(config::APP_INFO_PATH, get(handler::app_info))
            // Main app page
            .route(config::INDEX_PATH, get(handler::index))
            // WebSocket endpoint
            .route(
                config::WEBSOCKET_PATH,
                get(move |ws| ws::ws_handler(ws, Arc::clone(&session_store), app_fn)),
            )
            .layer(DefaultBodyLimit::max(config::max_body_size_usize()))
            .layer(CorsLayer::permissive())
            .layer(TraceLayer::new_for_http())
            .with_state(state)
    }

    /// Start the server.
    pub async fn run(&self) -> Result<()> {
        let addr: SocketAddr = format!("{}:{}", self.config.host, self.config.port)
            .parse()
            .map_err(|e| crate::error::Error::internal(format!("Invalid address: {}", e)))?;

        let router = self.build_router();

        tracing::info!(
            "Starting platypus server on http://{}:{}",
            self.config.host,
            self.config.port
        );

        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .map_err(|e| crate::error::Error::internal(format!("Failed to bind: {}", e)))?;

        axum::serve(listener, router)
            .await
            .map_err(|e| crate::error::Error::internal(format!("Server error: {}", e)))?;

        Ok(())
    }
}

impl Default for AppServer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ServerConfig::default();
        assert_eq!(config.port, config::DEFAULT_PORT);
        assert_eq!(config.host, config::DEFAULT_HOST);
    }

    #[test]
    fn test_server_creation() {
        let server = AppServer::new();
        assert_eq!(server.config.port, config::DEFAULT_PORT);
    }

    #[test]
    fn test_session_store() {
        let server = AppServer::new();
        let session_id = server.session_store.create_session("test".to_string());
        assert!(server.session_store.get_session(session_id).is_ok());
    }
}
