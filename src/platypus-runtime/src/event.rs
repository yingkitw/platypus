//! Event types for user interactions.

use serde::{Deserialize, Serialize};

/// User interaction event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    /// Widget state changed.
    WidgetChanged {
        key: String,
        value: serde_json::Value,
    },
    /// Button clicked.
    ButtonClicked { key: String },
    /// Script rerun requested.
    RerunScript,
    /// Custom event.
    Custom {
        event_type: String,
        data: serde_json::Value,
    },
}

impl Event {
    /// Create a widget change event.
    pub fn widget_changed(key: String, value: serde_json::Value) -> Self {
        Event::WidgetChanged { key, value }
    }

    /// Create a button click event.
    pub fn button_clicked(key: String) -> Self {
        Event::ButtonClicked { key }
    }

    /// Create a custom event.
    pub fn custom(event_type: String, data: serde_json::Value) -> Self {
        Event::Custom { event_type, data }
    }
}
