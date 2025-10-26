//! Webag Server - HTTP and WebSocket server for web apps.
//!
//! This crate provides the web server implementation for Webag applications,
//! including HTTP endpoints and WebSocket support for real-time communication.

pub mod error;
pub mod executor;
pub mod handler;
pub mod message;
pub mod server;
pub mod ws;

pub use error::{Error, Result};
pub use server::{AppServer, ServerConfig};

pub mod prelude {
    pub use crate::server::AppServer;
    pub use crate::error::Result;
}
