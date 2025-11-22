//! Core trait definitions for capability-based architecture.
//!
//! This module defines the fundamental traits that enable:
//! - Capability-based design (each trait represents a capability)
//! - Test-friendly implementations (easy to mock)
//! - Modular composition (traits can be combined)
//! - Extensibility (new capabilities can be added without modifying existing code)

use crate::element::ElementId;
use crate::error::Result;
use serde_json::Value;
use std::any::Any;

/// Core trait for any renderable component.
///
/// This is the foundation trait that all UI elements must implement.
/// It provides the basic interface for rendering and identification.
pub trait Renderable: Send + Sync {
    /// Get the unique identifier for this element.
    fn id(&self) -> ElementId;

    /// Get a human-readable name for this element.
    fn name(&self) -> &str;

    /// Convert to JSON representation for serialization.
    fn to_json(&self) -> Result<Value>;

    /// Get a reference to the element as Any for downcasting.
    fn as_any(&self) -> &dyn Any;
}

/// Trait for elements that can be validated.
///
/// Enables validation of element properties and state.
/// Useful for ensuring data integrity before rendering.
pub trait Validatable: Renderable {
    /// Validate the element's state and properties.
    ///
    /// Returns Ok(()) if valid, Err with validation error message if invalid.
    fn validate(&self) -> Result<()>;

    /// Get validation errors without failing.
    fn validation_errors(&self) -> Vec<String> {
        match self.validate() {
            Ok(()) => vec![],
            Err(e) => vec![e.to_string()],
        }
    }
}

/// Trait for elements that can be styled.
///
/// Enables consistent styling across different element types.
pub trait Styleable: Renderable {
    /// Get the CSS class names for this element.
    fn css_classes(&self) -> Vec<&str>;

    /// Get inline CSS styles for this element.
    fn inline_styles(&self) -> Vec<(&str, &str)>;

    /// Get the theme variant (e.g., "primary", "secondary", "danger").
    fn theme_variant(&self) -> Option<&str> {
        None
    }
}

/// Trait for interactive elements that can handle user input.
///
/// Enables event handling and state updates from user interactions.
pub trait Interactive: Renderable {
    /// Handle a user interaction event.
    ///
    /// Returns the new state after handling the event.
    fn handle_event(&mut self, event: &InteractionEvent) -> Result<()>;

    /// Get the current value of the interactive element.
    fn get_value(&self) -> Result<Value>;

    /// Set the value of the interactive element.
    fn set_value(&mut self, value: Value) -> Result<()>;

    /// Check if this element is disabled.
    fn is_disabled(&self) -> bool {
        false
    }

    /// Enable or disable this element.
    fn set_disabled(&mut self, disabled: bool);
}

/// Trait for elements that can contain child elements.
///
/// Enables hierarchical composition of UI elements.
pub trait Container: Renderable {
    /// Get the IDs of child elements.
    fn children(&self) -> Vec<ElementId>;

    /// Add a child element.
    fn add_child(&mut self, child_id: ElementId) -> Result<()>;

    /// Remove a child element.
    fn remove_child(&mut self, child_id: ElementId) -> Result<()>;

    /// Clear all children.
    fn clear_children(&mut self);

    /// Check if this container has children.
    fn has_children(&self) -> bool {
        !self.children().is_empty()
    }
}

/// Trait for elements that can be cached.
///
/// Enables performance optimization through caching strategies.
pub trait Cacheable: Renderable {
    /// Get the cache key for this element.
    fn cache_key(&self) -> String;

    /// Check if the cached version is still valid.
    fn is_cache_valid(&self) -> bool;

    /// Invalidate the cache.
    fn invalidate_cache(&mut self);

    /// Get the cache TTL in milliseconds.
    fn cache_ttl(&self) -> Option<u64> {
        Some(5 * 60 * 1000) // 5 minutes default
    }
}

/// Trait for elements that can be observed for changes.
///
/// Enables reactive updates when element state changes.
pub trait Observable: Renderable {
    /// Subscribe to changes on this element.
    fn subscribe(&mut self, observer: Box<dyn Observer>) -> Result<()>;

    /// Unsubscribe from changes.
    fn unsubscribe(&mut self) -> Result<()>;

    /// Notify observers of a change.
    fn notify_observers(&self, change: &ElementChange);
}

/// Trait for observing element changes.
pub trait Observer: Send + Sync {
    /// Called when an element changes.
    fn on_change(&self, element_id: ElementId, change: &ElementChange);
}

/// Represents a change to an element.
#[derive(Debug, Clone)]
pub struct ElementChange {
    /// Type of change (e.g., "value_changed", "state_changed").
    pub change_type: String,
    /// Old value (if applicable).
    pub old_value: Option<Value>,
    /// New value (if applicable).
    pub new_value: Option<Value>,
}

/// Trait for elements that support data binding.
///
/// Enables two-way data binding and reactive updates.
pub trait DataBindable: Renderable {
    /// Get the data binding path (e.g., "user.name").
    fn binding_path(&self) -> Option<&str>;

    /// Update from bound data.
    fn update_from_data(&mut self, data: &Value) -> Result<()>;

    /// Get the current data for binding.
    fn get_bound_data(&self) -> Result<Value>;
}

/// Trait for elements that can be serialized/deserialized.
///
/// Enables persistence and state management.
pub trait Serializable: Renderable {
    /// Serialize the element to a JSON value.
    fn serialize(&self) -> Result<Value>;

    /// Deserialize the element from a JSON value.
    fn deserialize(&mut self, data: &Value) -> Result<()>;
}

/// Trait for elements that support accessibility features.
///
/// Enables WCAG compliance and accessible UI.
pub trait Accessible: Renderable {
    /// Get the ARIA label for this element.
    fn aria_label(&self) -> Option<&str>;

    /// Get the ARIA role for this element.
    fn aria_role(&self) -> Option<&str>;

    /// Get the ARIA description for this element.
    fn aria_description(&self) -> Option<&str>;

    /// Check if this element is keyboard accessible.
    fn is_keyboard_accessible(&self) -> bool {
        true
    }
}

/// Trait for elements that support responsive design.
///
/// Enables adaptive layouts for different screen sizes.
pub trait Responsive: Renderable {
    /// Get the responsive breakpoint configuration.
    fn breakpoints(&self) -> Vec<Breakpoint>;

    /// Get the layout for a specific breakpoint.
    fn layout_for_breakpoint(&self, breakpoint: &str) -> Option<Layout>;
}

/// Represents a responsive breakpoint.
#[derive(Debug, Clone)]
pub struct Breakpoint {
    pub name: String,
    pub min_width: u32,
    pub max_width: Option<u32>,
}

/// Represents a layout configuration.
#[derive(Debug, Clone)]
pub struct Layout {
    pub display: String,
    pub flex_direction: Option<String>,
    pub width: Option<String>,
    pub height: Option<String>,
}

/// Trait for elements that support performance monitoring.
///
/// Enables performance tracking and optimization.
pub trait Monitorable: Renderable {
    /// Get the render time in milliseconds.
    fn render_time_ms(&self) -> Option<u64>;

    /// Get the memory usage in bytes.
    fn memory_usage_bytes(&self) -> Option<u64>;

    /// Record a performance metric.
    fn record_metric(&mut self, name: &str, value: f64);

    /// Get all recorded metrics.
    fn get_metrics(&self) -> Vec<(String, f64)>;
}

/// Trait for elements that support undo/redo.
///
/// Enables state history management.
pub trait Undoable: Renderable {
    /// Create a snapshot of the current state.
    fn create_snapshot(&self) -> Result<Value>;

    /// Restore from a snapshot.
    fn restore_snapshot(&mut self, snapshot: &Value) -> Result<()>;

    /// Get the undo history.
    fn undo_history(&self) -> Vec<Value>;

    /// Get the redo history.
    fn redo_history(&self) -> Vec<Value>;
}

/// Trait for elements that support theming.
///
/// Enables dynamic theme switching.
pub trait Themeable: Renderable {
    /// Get the current theme name.
    fn current_theme(&self) -> &str;

    /// Set the theme.
    fn set_theme(&mut self, theme: &str) -> Result<()>;

    /// Get available themes.
    fn available_themes(&self) -> Vec<&str>;
}

/// Trait for elements that support localization.
///
/// Enables multi-language support.
pub trait Localizable: Renderable {
    /// Get the current locale.
    fn current_locale(&self) -> &str;

    /// Set the locale.
    fn set_locale(&mut self, locale: &str) -> Result<()>;

    /// Get a localized string.
    fn get_localized_string(&self, key: &str) -> Option<String>;

    /// Get available locales.
    fn available_locales(&self) -> Vec<&str>;
}

/// User interaction event.
#[derive(Debug, Clone)]
pub struct InteractionEvent {
    pub event_type: String,
    pub target_id: ElementId,
    pub data: Option<Value>,
    pub timestamp: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mock implementation for testing.
    struct MockElement {
        id: ElementId,
        name: String,
    }

    impl Renderable for MockElement {
        fn id(&self) -> ElementId {
            self.id
        }

        fn name(&self) -> &str {
            &self.name
        }

        fn to_json(&self) -> Result<Value> {
            Ok(serde_json::json!({
                "id": self.id.inner(),
                "name": self.name
            }))
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    #[test]
    fn test_renderable_trait() {
        let element = MockElement {
            id: ElementId::new(1),
            name: "test".to_string(),
        };

        assert_eq!(element.id(), ElementId::new(1));
        assert_eq!(element.name(), "test");
        assert!(element.to_json().is_ok());
    }

    #[test]
    fn test_interaction_event() {
        let event = InteractionEvent {
            event_type: "click".to_string(),
            target_id: ElementId::new(1),
            data: Some(serde_json::json!({"value": 42})),
            timestamp: 1234567890,
        };

        assert_eq!(event.event_type, "click");
        assert_eq!(event.target_id, ElementId::new(1));
        assert!(event.data.is_some());
    }
}
