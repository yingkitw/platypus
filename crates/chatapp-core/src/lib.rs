//! Chatapp Core - Fundamental types and traits for the web app generator.
//!
//! This crate defines the core abstractions used throughout Chatapp:
//! - Trait-based architecture for capability-driven design
//! - Modular element implementations organized by category
//! - Widget state management with test-friendly interfaces
//! - Session and app state management
//! - Delta generation for UI updates
//!
//! ## Architecture
//!
//! The crate is organized around traits that represent capabilities:
//! - `Renderable`: Core trait for all UI elements
//! - `Validatable`: Elements that can be validated
//! - `Interactive`: Elements that handle user input
//! - `Container`: Elements that can contain children
//! - `Styleable`: Elements that support styling
//! - And many more specialized traits
//!
//! Concrete implementations are organized by category:
//! - `elements::display`: Text, headings, images, etc.
//! - `elements::input`: Buttons, text inputs, etc.
//! - `elements::layout`: Containers, rows, columns, etc.
//! - `elements::feedback`: Success, error, warning messages
//! - `elements::advanced`: Metrics, charts, etc.

pub mod element;
pub mod error;
pub mod session;
pub mod state;
pub mod widget;
pub mod traits;
pub mod traits_impl;
pub mod elements;

pub use element::{Element, ElementType, ElementId};
pub use error::{Error, Result};
pub use session::{Session, SessionId};
pub use state::{AppState, DeltaGenerator};
pub use widget::{Widget, WidgetValue};
pub use traits::{Renderable, Validatable, Interactive, Container, Observable, DataBindable};

/// Prelude module for convenient imports.
pub mod prelude {
    pub use crate::{
        element::{Element, ElementType, ElementId},
        error::{Error, Result},
        session::{Session, SessionId},
        state::{AppState, DeltaGenerator},
        widget::{Widget, WidgetValue},
        traits::{
            Renderable, Validatable, Interactive, Container, Styleable,
            Cacheable, Observable, DataBindable, Serializable, Accessible,
            Responsive, Monitorable, Undoable, Themeable, Localizable,
        },
        elements::{
            display::{TextElement, HeadingElement, ImageElement},
            input::{ButtonElement, TextInputElement},
            layout::ContainerElement,
            feedback::{FeedbackElement, FeedbackType},
            advanced::MetricElement,
        },
    };
}
