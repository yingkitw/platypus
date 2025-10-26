//! Additional widget elements - Radio, Multiselect, DatePicker, TimePicker, etc.

use crate::element::ElementId;
use crate::traits::{Renderable, Validatable, Interactive, InteractionEvent};
use crate::error::Result;
use serde_json::Value;
use std::any::Any;
use super::BaseElement;

/// Radio button group element for single selection.
#[derive(Debug, Clone)]
pub struct RadioElement {
    base: BaseElement,
    label: String,
    options: Vec<String>,
    selected_index: usize,
    disabled: bool,
}

impl RadioElement {
    /// Create a new radio element.
    pub fn new(id: ElementId, label: impl Into<String>, options: Vec<String>) -> Result<Self> {
        if options.is_empty() {
            return Err(crate::error::Error::StateError("Radio must have at least one option".to_string()));
        }
        Ok(Self {
            base: BaseElement::new(id, "radio"),
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

impl Renderable for RadioElement {
    fn id(&self) -> ElementId {
        self.base.id()
    }

    fn name(&self) -> &str {
        self.base.name()
    }

    fn to_json(&self) -> Result<Value> {
        Ok(serde_json::json!({
            "id": self.id().inner(),
            "type": "radio",
            "label": self.label,
            "options": self.options,
            "selected_index": self.selected_index,
        }))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Validatable for RadioElement {
    fn validate(&self) -> Result<()> {
        if self.label.is_empty() {
            return Err(crate::error::Error::StateError("Radio label cannot be empty".to_string()));
        }
        if self.options.is_empty() {
            return Err(crate::error::Error::StateError("Radio must have at least one option".to_string()));
        }
        Ok(())
    }
}

impl Interactive for RadioElement {
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
            Err(crate::error::Error::InvalidWidgetValue("Invalid radio value".to_string()))
        }
    }

    fn is_disabled(&self) -> bool {
        self.disabled
    }

    fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }
}

/// Multiselect element for multiple selections.
#[derive(Debug, Clone)]
pub struct MultiselectElement {
    base: BaseElement,
    label: String,
    options: Vec<String>,
    selected_indices: Vec<usize>,
    disabled: bool,
}

impl MultiselectElement {
    /// Create a new multiselect element.
    pub fn new(id: ElementId, label: impl Into<String>, options: Vec<String>) -> Result<Self> {
        if options.is_empty() {
            return Err(crate::error::Error::StateError("Multiselect must have at least one option".to_string()));
        }
        Ok(Self {
            base: BaseElement::new(id, "multiselect"),
            label: label.into(),
            options,
            selected_indices: Vec::new(),
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

    /// Get the selected values.
    pub fn selected_values(&self) -> Vec<&str> {
        self.selected_indices.iter()
            .map(|&i| self.options[i].as_str())
            .collect()
    }

    /// Get the selected indices.
    pub fn selected_indices(&self) -> &[usize] {
        &self.selected_indices
    }

    /// Add a selection.
    pub fn add_selection(&mut self, index: usize) -> Result<()> {
        if index >= self.options.len() {
            return Err(crate::error::Error::StateError("Index out of bounds".to_string()));
        }
        if !self.selected_indices.contains(&index) {
            self.selected_indices.push(index);
        }
        Ok(())
    }

    /// Remove a selection.
    pub fn remove_selection(&mut self, index: usize) -> Result<()> {
        self.selected_indices.retain(|&i| i != index);
        Ok(())
    }

    /// Clear all selections.
    pub fn clear_selections(&mut self) {
        self.selected_indices.clear();
    }
}

impl Renderable for MultiselectElement {
    fn id(&self) -> ElementId {
        self.base.id()
    }

    fn name(&self) -> &str {
        self.base.name()
    }

    fn to_json(&self) -> Result<Value> {
        Ok(serde_json::json!({
            "id": self.id().inner(),
            "type": "multiselect",
            "label": self.label,
            "options": self.options,
            "selected_indices": self.selected_indices,
        }))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Validatable for MultiselectElement {
    fn validate(&self) -> Result<()> {
        if self.label.is_empty() {
            return Err(crate::error::Error::StateError("Multiselect label cannot be empty".to_string()));
        }
        if self.options.is_empty() {
            return Err(crate::error::Error::StateError("Multiselect must have at least one option".to_string()));
        }
        Ok(())
    }
}

impl Interactive for MultiselectElement {
    fn handle_event(&mut self, event: &InteractionEvent) -> Result<()> {
        if event.event_type == "change" {
            if let Some(Value::Array(indices)) = &event.data {
                self.selected_indices.clear();
                for val in indices {
                    if let Some(n) = val.as_u64() {
                        self.add_selection(n as usize)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn get_value(&self) -> Result<Value> {
        Ok(Value::Array(
            self.selected_values()
                .into_iter()
                .map(|s| Value::String(s.to_string()))
                .collect()
        ))
    }

    fn set_value(&mut self, value: Value) -> Result<()> {
        if let Some(arr) = value.as_array() {
            self.selected_indices.clear();
            for val in arr {
                if let Some(s) = val.as_str() {
                    if let Some(index) = self.options.iter().position(|opt| opt == s) {
                        self.selected_indices.push(index);
                    }
                }
            }
            Ok(())
        } else {
            Err(crate::error::Error::InvalidWidgetValue("Invalid multiselect value".to_string()))
        }
    }

    fn is_disabled(&self) -> bool {
        self.disabled
    }

    fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }
}

/// Date picker element.
#[derive(Debug, Clone)]
pub struct DatePickerElement {
    base: BaseElement,
    label: String,
    value: String, // YYYY-MM-DD format
    min_date: Option<String>,
    max_date: Option<String>,
    disabled: bool,
}

impl DatePickerElement {
    /// Create a new date picker element.
    pub fn new(id: ElementId, label: impl Into<String>) -> Self {
        Self {
            base: BaseElement::new(id, "date_picker"),
            label: label.into(),
            value: String::new(),
            min_date: None,
            max_date: None,
            disabled: false,
        }
    }

    /// Get the label.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Get the current date value.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Set the minimum date.
    pub fn set_min_date(&mut self, date: impl Into<String>) {
        self.min_date = Some(date.into());
    }

    /// Set the maximum date.
    pub fn set_max_date(&mut self, date: impl Into<String>) {
        self.max_date = Some(date.into());
    }
}

impl Renderable for DatePickerElement {
    fn id(&self) -> ElementId {
        self.base.id()
    }

    fn name(&self) -> &str {
        self.base.name()
    }

    fn to_json(&self) -> Result<Value> {
        Ok(serde_json::json!({
            "id": self.id().inner(),
            "type": "date_picker",
            "label": self.label,
            "value": self.value,
            "min_date": self.min_date,
            "max_date": self.max_date,
        }))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Validatable for DatePickerElement {
    fn validate(&self) -> Result<()> {
        if self.label.is_empty() {
            return Err(crate::error::Error::StateError("DatePicker label cannot be empty".to_string()));
        }
        Ok(())
    }
}

impl Interactive for DatePickerElement {
    fn handle_event(&mut self, event: &InteractionEvent) -> Result<()> {
        if event.event_type == "change" {
            if let Some(Value::String(date)) = &event.data {
                self.value = date.clone();
            }
        }
        Ok(())
    }

    fn get_value(&self) -> Result<Value> {
        Ok(Value::String(self.value.clone()))
    }

    fn set_value(&mut self, value: Value) -> Result<()> {
        if let Some(s) = value.as_str() {
            self.value = s.to_string();
            Ok(())
        } else {
            Err(crate::error::Error::InvalidWidgetValue("Invalid date value".to_string()))
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
    fn test_radio_element() {
        let options = vec!["Option 1".to_string(), "Option 2".to_string()];
        let radio = RadioElement::new(ElementId::new(1), "Choose", options).unwrap();
        assert_eq!(radio.selected_value(), "Option 1");
    }

    #[test]
    fn test_multiselect_element() {
        let options = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let mut multiselect = MultiselectElement::new(ElementId::new(1), "Select", options).unwrap();
        multiselect.add_selection(0).unwrap();
        multiselect.add_selection(2).unwrap();
        assert_eq!(multiselect.selected_values(), vec!["A", "C"]);
    }

    #[test]
    fn test_date_picker() {
        let mut picker = DatePickerElement::new(ElementId::new(1), "Date");
        picker.set_min_date("2025-01-01");
        picker.set_max_date("2025-12-31");
        assert_eq!(picker.label(), "Date");
    }
}
