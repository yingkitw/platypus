//! Modular element implementations organized by category.
//!
//! This module provides concrete implementations of UI elements,
//! organized by category for better maintainability and testability.

pub mod display;
pub mod input;
pub mod layout;
pub mod feedback;
pub mod advanced;
pub mod widgets;
pub mod responsive;
pub mod themeable;
pub mod additional_widgets;
pub mod factory;

pub use display::*;
pub use input::*;
pub use layout::*;
pub use feedback::*;
pub use advanced::*;
pub use widgets::*;
pub use responsive::*;
pub use themeable::*;
pub use additional_widgets::*;
pub use factory::{ElementFactory, ElementBuilder};

use crate::element::ElementId;
use crate::traits::Renderable;
use crate::error::Result;
use serde_json::Value;
use std::any::Any;

/// Base element implementation providing common functionality.
///
/// This struct can be composed into specific element types to avoid
/// code duplication and ensure consistent behavior.
#[derive(Debug, Clone)]
pub struct BaseElement {
    id: ElementId,
    name: String,
    metadata: ElementMetadata,
}

/// Metadata for elements.
#[derive(Debug, Clone, Default)]
pub struct ElementMetadata {
    pub css_classes: Vec<String>,
    pub inline_styles: Vec<(String, String)>,
    pub aria_label: Option<String>,
    pub aria_role: Option<String>,
    pub data_attributes: std::collections::HashMap<String, String>,
}

impl BaseElement {
    /// Create a new base element.
    pub fn new(id: ElementId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            metadata: ElementMetadata::default(),
        }
    }

    /// Get mutable access to metadata.
    pub fn metadata_mut(&mut self) -> &mut ElementMetadata {
        &mut self.metadata
    }

    /// Get immutable access to metadata.
    pub fn metadata(&self) -> &ElementMetadata {
        &self.metadata
    }

    /// Add a CSS class.
    pub fn add_class(&mut self, class: impl Into<String>) {
        self.metadata.css_classes.push(class.into());
    }

    /// Add an inline style.
    pub fn add_style(&mut self, property: impl Into<String>, value: impl Into<String>) {
        self.metadata.inline_styles.push((property.into(), value.into()));
    }

    /// Set ARIA label.
    pub fn set_aria_label(&mut self, label: impl Into<String>) {
        self.metadata.aria_label = Some(label.into());
    }

    /// Set ARIA role.
    pub fn set_aria_role(&mut self, role: impl Into<String>) {
        self.metadata.aria_role = Some(role.into());
    }

    /// Set a data attribute.
    pub fn set_data_attribute(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.data_attributes.insert(key.into(), value.into());
    }
}

impl Renderable for BaseElement {
    fn id(&self) -> ElementId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn to_json(&self) -> Result<Value> {
        Ok(serde_json::json!({
            "id": self.id.inner(),
            "name": self.name,
            "metadata": {
                "css_classes": self.metadata.css_classes,
                "inline_styles": self.metadata.inline_styles,
                "aria_label": self.metadata.aria_label,
                "aria_role": self.metadata.aria_role,
            }
        }))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_element_creation() {
        let element = BaseElement::new(ElementId::new(1), "test");
        assert_eq!(element.id(), ElementId::new(1));
        assert_eq!(element.name(), "test");
    }

    #[test]
    fn test_base_element_metadata() {
        let mut element = BaseElement::new(ElementId::new(1), "test");
        element.add_class("primary");
        element.add_style("color", "red");
        element.set_aria_label("Test Button");

        assert_eq!(element.metadata().css_classes.len(), 1);
        assert_eq!(element.metadata().inline_styles.len(), 1);
        assert_eq!(element.metadata().aria_label, Some("Test Button".to_string()));
    }

    #[test]
    fn test_base_element_to_json() {
        let element = BaseElement::new(ElementId::new(1), "test");
        let json = element.to_json().unwrap();
        assert_eq!(json["id"], 1);
        assert_eq!(json["name"], "test");
    }
}
