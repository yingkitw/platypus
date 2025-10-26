//! Server error types.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Runtime error: {0}")]
    Runtime(#[from] platypus_runtime::Error),

    #[error("Core error: {0}")]
    Core(#[from] platypus_core::Error),

    #[error("WebSocket error: {0}")]
    WebSocket(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("{0}")]
    Other(#[from] anyhow::Error),
}

impl Error {
    pub fn websocket(msg: impl Into<String>) -> Self {
        Error::WebSocket(msg.into())
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        Error::NotFound(msg.into())
    }

    pub fn bad_request(msg: impl Into<String>) -> Self {
        Error::BadRequest(msg.into())
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        Error::Internal(msg.into())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Error::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            Error::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            Error::WebSocket(msg) => (StatusCode::BAD_REQUEST, msg),
            Error::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        let body = serde_json::json!({
            "error": error_message
        });

        (status, axum::Json(body)).into_response()
    }
}
