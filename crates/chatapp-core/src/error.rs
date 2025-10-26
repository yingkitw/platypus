//! Error types for Webag.

use thiserror::Error;

/// Webag result type.
pub type Result<T> = std::result::Result<T, Error>;

/// Webag error type.
#[derive(Error, Debug)]
pub enum Error {
    #[error("Session not found: {0}")]
    SessionNotFound(String),

    #[error("Widget not found: {0}")]
    WidgetNotFound(String),

    #[error("Invalid widget value: {0}")]
    InvalidWidgetValue(String),

    #[error("Script execution error: {0}")]
    ScriptExecutionError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("State error: {0}")]
    StateError(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Other(#[from] anyhow::Error),
}

impl Error {
    /// Create a new state error.
    pub fn state(msg: impl Into<String>) -> Self {
        Error::StateError(msg.into())
    }

    /// Create a new internal error.
    pub fn internal(msg: impl Into<String>) -> Self {
        Error::Internal(msg.into())
    }
}
