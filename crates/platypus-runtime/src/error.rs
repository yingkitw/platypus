//! Runtime error types.

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Core error: {0}")]
    Core(#[from] platypus_core::Error),

    #[error("Session error: {0}")]
    Session(String),

    #[error("Event error: {0}")]
    Event(String),

    #[error("Execution error: {0}")]
    Execution(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Other(#[from] anyhow::Error),
}

impl Error {
    pub fn session(msg: impl Into<String>) -> Self {
        Error::Session(msg.into())
    }

    pub fn event(msg: impl Into<String>) -> Self {
        Error::Event(msg.into())
    }

    pub fn execution(msg: impl Into<String>) -> Self {
        Error::Execution(msg.into())
    }
}
