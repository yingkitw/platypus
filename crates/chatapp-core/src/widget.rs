//! Widget types and state management.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Type-safe widget value.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum WidgetValue {
    /// String value (text input, selectbox, etc.)
    String(String),
    /// Number value (slider, number input, etc.)
    Number(f64),
    /// Boolean value (checkbox, etc.)
    Bool(bool),
    /// Array of strings (multiselect, etc.)
    StringArray(Vec<String>),
    /// Array of numbers
    NumberArray(Vec<f64>),
    /// Generic JSON value
    Json(Value),
}

impl WidgetValue {
    /// Try to get as string.
    pub fn as_string(&self) -> Option<&str> {
        match self {
            WidgetValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Try to get as number.
    pub fn as_number(&self) -> Option<f64> {
        match self {
            WidgetValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Try to get as boolean.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            WidgetValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Try to get as string array.
    pub fn as_string_array(&self) -> Option<&[String]> {
        match self {
            WidgetValue::StringArray(arr) => Some(arr),
            _ => None,
        }
    }

    /// Try to get as number array.
    pub fn as_number_array(&self) -> Option<&[f64]> {
        match self {
            WidgetValue::NumberArray(arr) => Some(arr),
            _ => None,
        }
    }
}

impl From<String> for WidgetValue {
    fn from(s: String) -> Self {
        WidgetValue::String(s)
    }
}

impl From<&str> for WidgetValue {
    fn from(s: &str) -> Self {
        WidgetValue::String(s.to_string())
    }
}

impl From<f64> for WidgetValue {
    fn from(n: f64) -> Self {
        WidgetValue::Number(n)
    }
}

impl From<bool> for WidgetValue {
    fn from(b: bool) -> Self {
        WidgetValue::Bool(b)
    }
}

impl From<Vec<String>> for WidgetValue {
    fn from(arr: Vec<String>) -> Self {
        WidgetValue::StringArray(arr)
    }
}

impl From<Vec<f64>> for WidgetValue {
    fn from(arr: Vec<f64>) -> Self {
        WidgetValue::NumberArray(arr)
    }
}

/// Trait for widgets with state.
pub trait Widget: Send + Sync {
    /// Get the widget key (unique identifier).
    fn key(&self) -> &str;

    /// Get the current value.
    fn value(&self) -> &WidgetValue;

    /// Set the value.
    fn set_value(&mut self, value: WidgetValue);

    /// Check if value changed this run.
    fn changed(&self) -> bool;

    /// Mark as changed.
    fn mark_changed(&mut self);

    /// Mark as unchanged.
    fn mark_unchanged(&mut self);
}

/// A simple widget implementation.
#[derive(Debug, Clone)]
pub struct SimpleWidget {
    key: String,
    value: WidgetValue,
    changed: bool,
}

impl SimpleWidget {
    /// Create a new widget.
    pub fn new(key: String, value: WidgetValue) -> Self {
        SimpleWidget {
            key,
            value,
            changed: false,
        }
    }
}

impl Widget for SimpleWidget {
    fn key(&self) -> &str {
        &self.key
    }

    fn value(&self) -> &WidgetValue {
        &self.value
    }

    fn set_value(&mut self, value: WidgetValue) {
        self.value = value;
    }

    fn changed(&self) -> bool {
        self.changed
    }

    fn mark_changed(&mut self) {
        self.changed = true;
    }

    fn mark_unchanged(&mut self) {
        self.changed = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_widget_value_conversions() {
        let s: WidgetValue = "hello".into();
        assert_eq!(s.as_string(), Some("hello"));

        let n: WidgetValue = 42.0.into();
        assert_eq!(n.as_number(), Some(42.0));

        let b: WidgetValue = true.into();
        assert_eq!(b.as_bool(), Some(true));
    }

    #[test]
    fn test_simple_widget() {
        let mut widget = SimpleWidget::new("test".to_string(), WidgetValue::String("value".to_string()));
        assert_eq!(widget.key(), "test");
        assert!(!widget.changed());

        widget.mark_changed();
        assert!(widget.changed());
    }
}
