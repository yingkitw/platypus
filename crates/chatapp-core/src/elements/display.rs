//! Display elements - non-interactive elements for showing content.
//!
//! This module contains implementations for text, images, and other
//! content display elements.

use crate::element::ElementId;
use crate::traits::{Renderable, Validatable, Styleable, Cacheable};
use crate::error::Result;
use serde_json::Value;
use std::any::Any;
use super::BaseElement;

/// Text element for displaying plain text.
#[derive(Debug, Clone)]
pub struct TextElement {
    base: BaseElement,
    content: String,
}

impl TextElement {
    /// Create a new text element.
    pub fn new(id: ElementId, content: impl Into<String>) -> Self {
        Self {
            base: BaseElement::new(id, "text"),
            content: content.into(),
        }
    }

    /// Get the text content.
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Set the text content.
    pub fn set_content(&mut self, content: impl Into<String>) {
        self.content = content.into();
    }

    /// Add a CSS class.
    pub fn add_class(&mut self, class: impl Into<String>) {
        self.base.add_class(class);
    }

    /// Add an inline style.
    pub fn add_style(&mut self, property: impl Into<String>, value: impl Into<String>) {
        self.base.add_style(property, value);
    }

    /// Set ARIA label.
    pub fn set_aria_label(&mut self, label: impl Into<String>) {
        self.base.set_aria_label(label);
    }

    /// Set ARIA role.
    pub fn set_aria_role(&mut self, role: impl Into<String>) {
        self.base.set_aria_role(role);
    }

    /// Get metadata.
    pub fn metadata(&self) -> &super::ElementMetadata {
        self.base.metadata()
    }
}

impl Renderable for TextElement {
    fn id(&self) -> ElementId {
        self.base.id()
    }

    fn name(&self) -> &str {
        self.base.name()
    }

    fn to_json(&self) -> Result<Value> {
        Ok(serde_json::json!({
            "id": self.id().inner(),
            "type": "text",
            "content": self.content,
        }))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Validatable for TextElement {
    fn validate(&self) -> Result<()> {
        if self.content.is_empty() {
            return Err(crate::error::Error::StateError("Text content cannot be empty".to_string()));
        }
        Ok(())
    }
}

impl Styleable for TextElement {
    fn css_classes(&self) -> Vec<&str> {
        self.base.metadata().css_classes.iter().map(|s| s.as_str()).collect()
    }

    fn inline_styles(&self) -> Vec<(&str, &str)> {
        self.base.metadata().inline_styles.iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect()
    }
}

impl Cacheable for TextElement {
    fn cache_key(&self) -> String {
        format!("text_{}", self.id().inner())
    }

    fn is_cache_valid(&self) -> bool {
        true
    }

    fn invalidate_cache(&mut self) {}
}

/// Heading element for displaying headings at different levels.
#[derive(Debug, Clone)]
pub struct HeadingElement {
    base: BaseElement,
    content: String,
    level: u32,
}

impl HeadingElement {
    /// Create a new heading element.
    pub fn new(id: ElementId, content: impl Into<String>, level: u32) -> Result<Self> {
        if level < 1 || level > 6 {
            return Err(crate::error::Error::StateError("Heading level must be between 1 and 6".to_string()));
        }
        Ok(Self {
            base: BaseElement::new(id, format!("heading_{}", level)),
            content: content.into(),
            level,
        })
    }

    /// Get the heading level.
    pub fn level(&self) -> u32 {
        self.level
    }

    /// Get the heading content.
    pub fn content(&self) -> &str {
        &self.content
    }
}

impl Renderable for HeadingElement {
    fn id(&self) -> ElementId {
        self.base.id()
    }

    fn name(&self) -> &str {
        self.base.name()
    }

    fn to_json(&self) -> Result<Value> {
        Ok(serde_json::json!({
            "id": self.id().inner(),
            "type": "heading",
            "level": self.level,
            "content": self.content,
        }))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Validatable for HeadingElement {
    fn validate(&self) -> Result<()> {
        if self.content.is_empty() {
            return Err(crate::error::Error::StateError("Heading content cannot be empty".to_string()));
        }
        if self.level < 1 || self.level > 6 {
            return Err(crate::error::Error::StateError("Heading level must be between 1 and 6".to_string()));
        }
        Ok(())
    }
}

impl Styleable for HeadingElement {
    fn css_classes(&self) -> Vec<&str> {
        self.base.metadata().css_classes.iter().map(|s| s.as_str()).collect()
    }

    fn inline_styles(&self) -> Vec<(&str, &str)> {
        self.base.metadata().inline_styles.iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect()
    }

    fn theme_variant(&self) -> Option<&str> {
        Some("heading")
    }
}

/// Image element for displaying images.
#[derive(Debug, Clone)]
pub struct ImageElement {
    base: BaseElement,
    src: String,
    alt: String,
    width: Option<u32>,
    height: Option<u32>,
}

impl ImageElement {
    /// Create a new image element.
    pub fn new(id: ElementId, src: impl Into<String>, alt: impl Into<String>) -> Self {
        Self {
            base: BaseElement::new(id, "image"),
            src: src.into(),
            alt: alt.into(),
            width: None,
            height: None,
        }
    }

    /// Set the image width.
    pub fn set_width(&mut self, width: u32) {
        self.width = Some(width);
    }

    /// Set the image height.
    pub fn set_height(&mut self, height: u32) {
        self.height = Some(height);
    }

    /// Get the image source.
    pub fn src(&self) -> &str {
        &self.src
    }

    /// Get the alt text.
    pub fn alt(&self) -> &str {
        &self.alt
    }
}

impl Renderable for ImageElement {
    fn id(&self) -> ElementId {
        self.base.id()
    }

    fn name(&self) -> &str {
        self.base.name()
    }

    fn to_json(&self) -> Result<Value> {
        Ok(serde_json::json!({
            "id": self.id().inner(),
            "type": "image",
            "src": self.src,
            "alt": self.alt,
            "width": self.width,
            "height": self.height,
        }))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Validatable for ImageElement {
    fn validate(&self) -> Result<()> {
        if self.src.is_empty() {
            return Err(crate::error::Error::StateError("Image source cannot be empty".to_string()));
        }
        if self.alt.is_empty() {
            return Err(crate::error::Error::StateError("Image alt text cannot be empty".to_string()));
        }
        Ok(())
    }
}

impl Styleable for ImageElement {
    fn css_classes(&self) -> Vec<&str> {
        self.base.metadata().css_classes.iter().map(|s| s.as_str()).collect()
    }

    fn inline_styles(&self) -> Vec<(&str, &str)> {
        self.base.metadata().inline_styles.iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect()
    }
}

impl Cacheable for ImageElement {
    fn cache_key(&self) -> String {
        format!("image_{}", self.src)
    }

    fn is_cache_valid(&self) -> bool {
        true
    }

    fn invalidate_cache(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_element() {
        let mut element = TextElement::new(ElementId::new(1), "Hello, World!");
        assert_eq!(element.content(), "Hello, World!");
        assert!(element.validate().is_ok());

        element.set_content("Updated");
        assert_eq!(element.content(), "Updated");
    }

    #[test]
    fn test_text_element_validation() {
        let element = TextElement::new(ElementId::new(1), "");
        assert!(element.validate().is_err());
    }

    #[test]
    fn test_heading_element() {
        let element = HeadingElement::new(ElementId::new(1), "Title", 1).unwrap();
        assert_eq!(element.level(), 1);
        assert_eq!(element.content(), "Title");
        assert!(element.validate().is_ok());
    }

    #[test]
    fn test_heading_element_invalid_level() {
        assert!(HeadingElement::new(ElementId::new(1), "Title", 7).is_err());
        assert!(HeadingElement::new(ElementId::new(1), "Title", 0).is_err());
    }

    #[test]
    fn test_image_element() {
        let mut element = ImageElement::new(ElementId::new(1), "image.png", "An image");
        element.set_width(100);
        element.set_height(100);

        assert_eq!(element.src(), "image.png");
        assert_eq!(element.alt(), "An image");
        assert!(element.validate().is_ok());
    }

    #[test]
    fn test_image_element_validation() {
        let element = ImageElement::new(ElementId::new(1), "", "");
        assert!(element.validate().is_err());
    }

    #[test]
    fn test_element_to_json() {
        let element = TextElement::new(ElementId::new(1), "Hello");
        let json = element.to_json().unwrap();
        assert_eq!(json["type"], "text");
        assert_eq!(json["content"], "Hello");
    }
}
