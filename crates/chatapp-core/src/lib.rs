//! Webag Core - Fundamental types and traits for the web app generator.
//!
//! This crate defines the core abstractions used throughout Webag:
//! - Element types and traits
//! - Widget state management
//! - Session and app state
//! - Delta generation for UI updates

pub mod element;
pub mod error;
pub mod session;
pub mod state;
pub mod widget;

pub use element::{Element, ElementType};
pub use error::{Error, Result};
pub use session::{Session, SessionId};
pub use state::{AppState, DeltaGenerator};
pub use widget::{Widget, WidgetValue};

/// Prelude module for convenient imports.
pub mod prelude {
    pub use crate::{
        element::{Element, ElementType},
        error::{Error, Result},
        session::{Session, SessionId},
        state::{AppState, DeltaGenerator},
        widget::{Widget, WidgetValue},
    };
}
