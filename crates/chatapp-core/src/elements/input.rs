//! Input elements - interactive elements for user input.
//!
//! This module contains implementations for buttons, text inputs,
//! and other interactive form elements.

use crate::element::ElementId;
use crate::traits::{Renderable, Validatable, Styleable, Interactive, InteractionEvent};
use crate::error::Result;
use serde_json::Value;
use std::any::Any;
use super::BaseElement;

/// Button element for user interactions.
#[derive(Debug, Clone)]
pub struct ButtonElement {
    base: BaseElement,
    label: String,
    disabled: bool,
    click_count: u32,
}

impl ButtonElement {
    /// Create a new button element.
    pub fn new(id: ElementId, label: impl Into<String>) -> Self {
        Self {
            base: BaseElement::new(id, "button"),
            label: label.into(),
            disabled: false,
            click_count: 0,
        }
    }

    /// Get the button label.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Set the button label.
    pub fn set_label(&mut self, label: impl Into<String>) {
        self.label = label.into();
    }

    /// Get the click count.
    pub fn click_count(&self) -> u32 {
        self.click_count
    }

    /// Reset the click count.
    pub fn reset_click_count(&mut self) {
        self.click_count = 0;
    }
}

impl Renderable for ButtonElement {
    fn id(&self) -> ElementId {
        self.base.id()
    }

    fn name(&self) -> &str {
        self.base.name()
    }

    fn to_json(&self) -> Result<Value> {
        Ok(serde_json::json!({
            "id": self.id().inner(),
            "type": "button",
            "label": self.label,
            "disabled": self.disabled,
            "click_count": self.click_count,
        }))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Validatable for ButtonElement {
    fn validate(&self) -> Result<()> {
        if self.label.is_empty() {
            return Err(crate::error::Error::StateError("Button label cannot be empty".to_string()));
        }
        Ok(())
    }
}

impl Styleable for ButtonElement {
    fn css_classes(&self) -> Vec<&str> {
        self.base.metadata().css_classes.iter().map(|s| s.as_str()).collect()
    }

    fn inline_styles(&self) -> Vec<(&str, &str)> {
        self.base.metadata().inline_styles.iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect()
    }

    fn theme_variant(&self) -> Option<&str> {
        Some("button")
    }
}

impl Interactive for ButtonElement {
    fn handle_event(&mut self, event: &InteractionEvent) -> Result<()> {
        if event.event_type == "click" && !self.disabled {
            self.click_count += 1;
            Ok(())
        } else {
            Ok(())
        }
    }

    fn get_value(&self) -> Result<Value> {
        Ok(Value::Number(self.click_count.into()))
    }

    fn set_value(&mut self, value: Value) -> Result<()> {
        if let Some(count) = value.as_u64() {
            self.click_count = count as u32;
            Ok(())
        } else {
            Err(crate::error::Error::InvalidWidgetValue("Invalid value for button click count".to_string()))
        }
    }

    fn is_disabled(&self) -> bool {
        self.disabled
    }

    fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }
}

/// Text input element for single-line text input.
#[derive(Debug, Clone)]
pub struct TextInputElement {
    base: BaseElement,
    label: String,
    value: String,
    placeholder: Option<String>,
    disabled: bool,
    required: bool,
    max_length: Option<usize>,
}

impl TextInputElement {
    /// Create a new text input element.
    pub fn new(id: ElementId, label: impl Into<String>) -> Self {
        Self {
            base: BaseElement::new(id, "text_input"),
            label: label.into(),
            value: String::new(),
            placeholder: None,
            disabled: false,
            required: false,
            max_length: None,
        }
    }

    /// Get the input label.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Get the current value.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Set the placeholder text.
    pub fn set_placeholder(&mut self, placeholder: impl Into<String>) {
        self.placeholder = Some(placeholder.into());
    }

    /// Set the maximum length.
    pub fn set_max_length(&mut self, max_length: usize) {
        self.max_length = Some(max_length);
    }

    /// Set whether the field is required.
    pub fn set_required(&mut self, required: bool) {
        self.required = required;
    }
}

impl Renderable for TextInputElement {
    fn id(&self) -> ElementId {
        self.base.id()
    }

    fn name(&self) -> &str {
        self.base.name()
    }

    fn to_json(&self) -> Result<Value> {
        Ok(serde_json::json!({
            "id": self.id().inner(),
            "type": "text_input",
            "label": self.label,
            "value": self.value,
            "placeholder": self.placeholder,
            "required": self.required,
            "max_length": self.max_length,
        }))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Validatable for TextInputElement {
    fn validate(&self) -> Result<()> {
        if self.label.is_empty() {
            return Err(crate::error::Error::StateError("Input label cannot be empty".to_string()));
        }
        if self.required && self.value.is_empty() {
            return Err(crate::error::Error::StateError("This field is required".to_string()));
        }
        if let Some(max_len) = self.max_length {
            if self.value.len() > max_len {
                return Err(crate::error::Error::StateError(format!("Value exceeds maximum length of {}", max_len)));
            }
        }
        Ok(())
    }
}

impl Styleable for TextInputElement {
    fn css_classes(&self) -> Vec<&str> {
        self.base.metadata().css_classes.iter().map(|s| s.as_str()).collect()
    }

    fn inline_styles(&self) -> Vec<(&str, &str)> {
        self.base.metadata().inline_styles.iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect()
    }
}

impl Interactive for TextInputElement {
    fn handle_event(&mut self, event: &InteractionEvent) -> Result<()> {
        if event.event_type == "change" {
            if let Some(Value::String(new_value)) = &event.data {
                if let Some(max_len) = self.max_length {
                    if new_value.len() > max_len {
                        return Err(crate::error::Error::StateError(format!("Value exceeds maximum length of {}", max_len)));
                    }
                }
                self.value = new_value.clone();
            }
        }
        Ok(())
    }

    fn get_value(&self) -> Result<Value> {
        Ok(Value::String(self.value.clone()))
    }

    fn set_value(&mut self, value: Value) -> Result<()> {
        if let Some(s) = value.as_str() {
            if let Some(max_len) = self.max_length {
                if s.len() > max_len {
                    return Err(crate::error::Error::StateError(format!("Value exceeds maximum length of {}", max_len)));
                }
            }
            self.value = s.to_string();
            Ok(())
        } else {
            Err(crate::error::Error::InvalidWidgetValue("Invalid value for text input".to_string()))
        }
    }

    fn is_disabled(&self) -> bool {
        self.disabled
    }

    fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_element() {
        let mut button = ButtonElement::new(ElementId::new(1), "Click me");
        assert_eq!(button.label(), "Click me");
        assert_eq!(button.click_count(), 0);
        assert!(!button.is_disabled());

        let event = InteractionEvent {
            event_type: "click".to_string(),
            target_id: ElementId::new(1),
            data: None,
            timestamp: 0,
        };

        button.handle_event(&event).unwrap();
        assert_eq!(button.click_count(), 1);
    }

    #[test]
    fn test_button_disabled() {
        let mut button = ButtonElement::new(ElementId::new(1), "Click me");
        button.set_disabled(true);

        let event = InteractionEvent {
            event_type: "click".to_string(),
            target_id: ElementId::new(1),
            data: None,
            timestamp: 0,
        };

        button.handle_event(&event).unwrap();
        assert_eq!(button.click_count(), 0); // Should not increment when disabled
    }

    #[test]
    fn test_text_input_element() {
        let mut input = TextInputElement::new(ElementId::new(1), "Name");
        assert_eq!(input.label(), "Name");
        assert_eq!(input.value(), "");

        input.set_value(Value::String("John".to_string())).unwrap();
        assert_eq!(input.value(), "John");
    }

    #[test]
    fn test_text_input_max_length() {
        let mut input = TextInputElement::new(ElementId::new(1), "Name");
        input.set_max_length(5);

        let result = input.set_value(Value::String("TooLong".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn test_text_input_required() {
        let mut input = TextInputElement::new(ElementId::new(1), "Name");
        input.set_required(true);

        assert!(input.validate().is_err());

        input.set_value(Value::String("John".to_string())).unwrap();
        assert!(input.validate().is_ok());
    }
}
