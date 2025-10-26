//! Webag Runtime - App execution and state management.
//!
//! This crate provides the runtime engine for executing Webag applications,
//! managing state, handling events, and generating UI deltas.

pub mod context;
pub mod error;
pub mod event;
pub mod session_store;

pub use context::St;
pub use error::{Error, Result};
pub use event::Event;
pub use session_store::SessionStore;

pub mod prelude {
    pub use crate::{context::St, error::Result, session_store::SessionStore};
}
