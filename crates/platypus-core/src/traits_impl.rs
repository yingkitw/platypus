//! Trait implementations for advanced capabilities.
//!
//! This module provides concrete implementations of advanced traits
//! like Observable and DataBindable.

use crate::element::ElementId;
use crate::traits::{Observable, Observer, ElementChange, DataBindable};
use crate::error::Result;
use serde_json::Value;
use std::sync::Arc;
use parking_lot::Mutex;

/// Default observable implementation.
pub struct DefaultObservable {
    observers: Arc<Mutex<Vec<Box<dyn Observer>>>>,
}

impl DefaultObservable {
    /// Create a new observable.
    pub fn new() -> Self {
        Self {
            observers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Subscribe an observer.
    pub fn subscribe(&mut self, observer: Box<dyn Observer>) -> Result<()> {
        self.observers.lock().push(observer);
        Ok(())
    }

    /// Notify all observers.
    pub fn notify_observers(&self, element_id: ElementId, change: &ElementChange) {
        let observers = self.observers.lock();
        for observer in observers.iter() {
            observer.on_change(element_id, change);
        }
    }
}

impl Default for DefaultObservable {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple data binding implementation.
pub struct SimpleDataBinding {
    binding_path: Option<String>,
    current_data: Option<Value>,
}

impl SimpleDataBinding {
    /// Create a new data binding.
    pub fn new() -> Self {
        Self {
            binding_path: None,
            current_data: None,
        }
    }

    /// Set the binding path.
    pub fn set_binding_path(&mut self, path: impl Into<String>) {
        self.binding_path = Some(path.into());
    }

    /// Get the binding path.
    pub fn binding_path(&self) -> Option<&str> {
        self.binding_path.as_deref()
    }

    /// Update the bound data.
    pub fn update_data(&mut self, data: Value) {
        self.current_data = Some(data);
    }

    /// Get the current bound data.
    pub fn get_data(&self) -> Option<&Value> {
        self.current_data.as_ref()
    }
}

impl Default for SimpleDataBinding {
    fn default() -> Self {
        Self::new()
    }
}

/// Observable button implementation.
pub struct ObservableButton {
    #[allow(dead_code)]
    id: ElementId,
    label: String,
    click_count: u32,
    observable: DefaultObservable,
}

impl ObservableButton {
    /// Create a new observable button.
    pub fn new(id: ElementId, label: impl Into<String>) -> Self {
        Self {
            id,
            label: label.into(),
            click_count: 0,
            observable: DefaultObservable::new(),
        }
    }

    /// Get the button label.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Get the click count.
    pub fn click_count(&self) -> u32 {
        self.click_count
    }

    /// Handle a click event.
    pub fn handle_click(&mut self) {
        self.click_count += 1;
        let change = ElementChange {
            change_type: "click".to_string(),
            old_value: Some(Value::Number((self.click_count - 1).into())),
            new_value: Some(Value::Number(self.click_count.into())),
        };
        self.observable.notify_observers(self.id, &change);
    }

    /// Subscribe to changes.
    pub fn subscribe(&mut self, observer: Box<dyn Observer>) -> Result<()> {
        self.observable.subscribe(observer)
    }
}

/// Data-bindable text input implementation.
pub struct DataBindableTextInput {
    id: ElementId,
    label: String,
    value: String,
    binding: SimpleDataBinding,
}

impl DataBindableTextInput {
    /// Create a new data-bindable text input.
    pub fn new(id: ElementId, label: impl Into<String>) -> Self {
        Self {
            id,
            label: label.into(),
            value: String::new(),
            binding: SimpleDataBinding::new(),
        }
    }

    /// Get the label.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Get the current value.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Set the value.
    pub fn set_value(&mut self, value: impl Into<String>) {
        self.value = value.into();
    }

    /// Set the binding path.
    pub fn set_binding_path(&mut self, path: impl Into<String>) {
        self.binding.set_binding_path(path);
    }

    /// Update from bound data.
    pub fn update_from_data(&mut self, data: &Value) -> Result<()> {
        if let Some(s) = data.as_str() {
            self.value = s.to_string();
            self.binding.update_data(data.clone());
            Ok(())
        } else {
            Err(crate::error::Error::StateError("Invalid data for text input".to_string()))
        }
    }

    /// Get the bound data.
    pub fn get_bound_data(&self) -> Option<&Value> {
        self.binding.get_data()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestObserver {
        changes: Arc<Mutex<Vec<String>>>,
    }

    impl Observer for TestObserver {
        fn on_change(&self, _element_id: ElementId, change: &ElementChange) {
            self.changes.lock().push(change.change_type.clone());
        }
    }

    #[test]
    fn test_observable_button() {
        let mut button = ObservableButton::new(ElementId::new(1), "Click");
        let changes = Arc::new(Mutex::new(Vec::new()));
        let observer = TestObserver {
            changes: changes.clone(),
        };
        button.subscribe(Box::new(observer)).unwrap();

        button.handle_click();
        assert_eq!(button.click_count(), 1);
    }

    #[test]
    fn test_data_bindable_input() {
        let mut input = DataBindableTextInput::new(ElementId::new(1), "Name");
        input.set_binding_path("user.name");

        let data = serde_json::json!("John Doe");
        input.update_from_data(&data).unwrap();
        assert_eq!(input.value(), "John Doe");
    }

    #[test]
    fn test_simple_data_binding() {
        let mut binding = SimpleDataBinding::new();
        binding.set_binding_path("user.email");
        assert_eq!(binding.binding_path(), Some("user.email"));

        binding.update_data(serde_json::json!("john@example.com"));
        assert!(binding.get_data().is_some());
    }
}
