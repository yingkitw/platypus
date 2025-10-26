//! Additional widget elements - Slider, Checkbox, Selectbox, Multiselect, etc.

use crate::element::ElementId;
use crate::traits::{Renderable, Validatable, Interactive, InteractionEvent};
use crate::error::Result;
use serde_json::Value;
use std::any::Any;
use super::BaseElement;

/// Slider element for numeric input.
#[derive(Debug, Clone)]
pub struct SliderElement {
    base: BaseElement,
    label: String,
    value: f64,
    min: f64,
    max: f64,
    step: f64,
    disabled: bool,
}

impl SliderElement {
    /// Create a new slider element.
    pub fn new(id: ElementId, label: impl Into<String>, min: f64, max: f64) -> Result<Self> {
        if min >= max {
            return Err(crate::error::Error::StateError("min must be less than max".to_string()));
        }
        Ok(Self {
            base: BaseElement::new(id, "slider"),
            label: label.into(),
            value: min,
            min,
            max,
            step: 1.0,
            disabled: false,
        })
    }

    /// Set the step size.
    pub fn set_step(&mut self, step: f64) {
        self.step = step;
    }

    /// Get the current value.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Get the minimum value.
    pub fn min(&self) -> f64 {
        self.min
    }

    /// Get the maximum value.
    pub fn max(&self) -> f64 {
        self.max
    }
}

impl Renderable for SliderElement {
    fn id(&self) -> ElementId {
        self.base.id()
    }

    fn name(&self) -> &str {
        self.base.name()
    }

    fn to_json(&self) -> Result<Value> {
        Ok(serde_json::json!({
            "id": self.id().inner(),
            "type": "slider",
            "label": self.label,
            "value": self.value,
            "min": self.min,
            "max": self.max,
            "step": self.step,
        }))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Validatable for SliderElement {
    fn validate(&self) -> Result<()> {
        if self.value < self.min || self.value > self.max {
            return Err(crate::error::Error::StateError(
                format!("Value {} is outside range [{}, {}]", self.value, self.min, self.max)
            ));
        }
        Ok(())
    }
}

impl Interactive for SliderElement {
    fn handle_event(&mut self, event: &InteractionEvent) -> Result<()> {
        if event.event_type == "change" {
            if let Some(Value::Number(n)) = &event.data {
                if let Some(f) = n.as_f64() {
                    if f >= self.min && f <= self.max {
                        self.value = f;
                    }
                }
            }
        }
        Ok(())
    }

    fn get_value(&self) -> Result<Value> {
        Ok(Value::Number(serde_json::Number::from_f64(self.value).unwrap()))
    }

    fn set_value(&mut self, value: Value) -> Result<()> {
        if let Some(n) = value.as_f64() {
            if n < self.min || n > self.max {
                return Err(crate::error::Error::StateError(
                    format!("Value {} is outside range [{}, {}]", n, self.min, self.max)
                ));
            }
            self.value = n;
            Ok(())
        } else {
            Err(crate::error::Error::InvalidWidgetValue("Invalid slider value".to_string()))
        }
    }

    fn is_disabled(&self) -> bool {
        self.disabled
    }

    fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }
}

/// Checkbox element for boolean input.
#[derive(Debug, Clone)]
pub struct CheckboxElement {
    base: BaseElement,
    label: String,
    checked: bool,
    disabled: bool,
}

impl CheckboxElement {
    /// Create a new checkbox element.
    pub fn new(id: ElementId, label: impl Into<String>) -> Self {
        Self {
            base: BaseElement::new(id, "checkbox"),
            label: label.into(),
            checked: false,
            disabled: false,
        }
    }

    /// Get the label.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Check if the checkbox is checked.
    pub fn is_checked(&self) -> bool {
        self.checked
    }

    /// Set the checked state.
    pub fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
    }
}

impl Renderable for CheckboxElement {
    fn id(&self) -> ElementId {
        self.base.id()
    }

    fn name(&self) -> &str {
        self.base.name()
    }

    fn to_json(&self) -> Result<Value> {
        Ok(serde_json::json!({
            "id": self.id().inner(),
            "type": "checkbox",
            "label": self.label,
            "checked": self.checked,
        }))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Validatable for CheckboxElement {
    fn validate(&self) -> Result<()> {
        if self.label.is_empty() {
            return Err(crate::error::Error::StateError("Checkbox label cannot be empty".to_string()));
        }
        Ok(())
    }
}

impl Interactive for CheckboxElement {
    fn handle_event(&mut self, event: &InteractionEvent) -> Result<()> {
        if event.event_type == "change" {
            if let Some(Value::Bool(checked)) = &event.data {
                self.checked = *checked;
            }
        }
        Ok(())
    }

    fn get_value(&self) -> Result<Value> {
        Ok(Value::Bool(self.checked))
    }

    fn set_value(&mut self, value: Value) -> Result<()> {
        if let Some(b) = value.as_bool() {
            self.checked = b;
            Ok(())
        } else {
            Err(crate::error::Error::InvalidWidgetValue("Invalid checkbox value".to_string()))
        }
    }

    fn is_disabled(&self) -> bool {
        self.disabled
    }

    fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }
}

/// Selectbox element for single selection from options.
#[derive(Debug, Clone)]
pub struct SelectboxElement {
    base: BaseElement,
    label: String,
    options: Vec<String>,
    selected_index: usize,
    disabled: bool,
}

impl SelectboxElement {
    /// Create a new selectbox element.
    pub fn new(id: ElementId, label: impl Into<String>, options: Vec<String>) -> Result<Self> {
        if options.is_empty() {
            return Err(crate::error::Error::StateError("Selectbox must have at least one option".to_string()));
        }
        Ok(Self {
            base: BaseElement::new(id, "selectbox"),
            label: label.into(),
            options,
            selected_index: 0,
            disabled: false,
        })
    }

    /// Get the label.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Get the options.
    pub fn options(&self) -> &[String] {
        &self.options
    }

    /// Get the selected value.
    pub fn selected_value(&self) -> &str {
        &self.options[self.selected_index]
    }

    /// Get the selected index.
    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    /// Set the selected index.
    pub fn set_selected_index(&mut self, index: usize) -> Result<()> {
        if index >= self.options.len() {
            return Err(crate::error::Error::StateError("Index out of bounds".to_string()));
        }
        self.selected_index = index;
        Ok(())
    }
}

impl Renderable for SelectboxElement {
    fn id(&self) -> ElementId {
        self.base.id()
    }

    fn name(&self) -> &str {
        self.base.name()
    }

    fn to_json(&self) -> Result<Value> {
        Ok(serde_json::json!({
            "id": self.id().inner(),
            "type": "selectbox",
            "label": self.label,
            "options": self.options,
            "selected_index": self.selected_index,
        }))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Validatable for SelectboxElement {
    fn validate(&self) -> Result<()> {
        if self.label.is_empty() {
            return Err(crate::error::Error::StateError("Selectbox label cannot be empty".to_string()));
        }
        if self.options.is_empty() {
            return Err(crate::error::Error::StateError("Selectbox must have at least one option".to_string()));
        }
        Ok(())
    }
}

impl Interactive for SelectboxElement {
    fn handle_event(&mut self, event: &InteractionEvent) -> Result<()> {
        if event.event_type == "change" {
            if let Some(Value::Number(n)) = &event.data {
                if let Some(index) = n.as_u64() {
                    self.set_selected_index(index as usize)?;
                }
            }
        }
        Ok(())
    }

    fn get_value(&self) -> Result<Value> {
        Ok(Value::String(self.selected_value().to_string()))
    }

    fn set_value(&mut self, value: Value) -> Result<()> {
        if let Some(s) = value.as_str() {
            if let Some(index) = self.options.iter().position(|opt| opt == s) {
                self.selected_index = index;
                Ok(())
            } else {
                Err(crate::error::Error::StateError("Value not found in options".to_string()))
            }
        } else {
            Err(crate::error::Error::InvalidWidgetValue("Invalid selectbox value".to_string()))
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
    fn test_slider_element() {
        let mut slider = SliderElement::new(ElementId::new(1), "Volume", 0.0, 100.0).unwrap();
        assert_eq!(slider.value(), 0.0);
        assert_eq!(slider.min(), 0.0);
        assert_eq!(slider.max(), 100.0);

        slider.set_value(serde_json::json!(50.0)).unwrap();
        assert_eq!(slider.value(), 50.0);
    }

    #[test]
    fn test_slider_validation() {
        let slider = SliderElement::new(ElementId::new(1), "Volume", 0.0, 100.0).unwrap();
        assert!(slider.validate().is_ok());
    }

    #[test]
    fn test_checkbox_element() {
        let mut checkbox = CheckboxElement::new(ElementId::new(1), "Accept");
        assert!(!checkbox.is_checked());

        checkbox.set_checked(true);
        assert!(checkbox.is_checked());
    }

    #[test]
    fn test_checkbox_value() {
        let mut checkbox = CheckboxElement::new(ElementId::new(1), "Accept");
        checkbox.set_value(serde_json::json!(true)).unwrap();
        assert!(checkbox.is_checked());
    }

    #[test]
    fn test_selectbox_element() {
        let options = vec!["Option 1".to_string(), "Option 2".to_string(), "Option 3".to_string()];
        let selectbox = SelectboxElement::new(ElementId::new(1), "Choose", options).unwrap();
        assert_eq!(selectbox.selected_value(), "Option 1");
        assert_eq!(selectbox.selected_index(), 0);
    }

    #[test]
    fn test_selectbox_set_index() {
        let options = vec!["Option 1".to_string(), "Option 2".to_string()];
        let mut selectbox = SelectboxElement::new(ElementId::new(1), "Choose", options).unwrap();
        selectbox.set_selected_index(1).unwrap();
        assert_eq!(selectbox.selected_value(), "Option 2");
    }
}
