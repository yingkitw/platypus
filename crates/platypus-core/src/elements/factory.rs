//! Element factory for convenient element creation.
//!
//! Provides builder-style API for creating elements with fluent interface.

use crate::element::ElementId;
use crate::error::Result;
use super::{
    display::{TextElement, HeadingElement, ImageElement},
    input::{ButtonElement, TextInputElement},
    layout::ContainerElement,
    feedback::{FeedbackElement, FeedbackType},
    advanced::MetricElement,
};

/// Builder for creating elements with fluent API.
pub struct ElementBuilder {
    id: ElementId,
}

impl ElementBuilder {
    /// Create a new element builder.
    pub fn new(id: ElementId) -> Self {
        Self { id }
    }

    /// Create a text element.
    pub fn text(self, content: impl Into<String>) -> TextElement {
        TextElement::new(self.id, content)
    }

    /// Create a heading element.
    pub fn heading(self, content: impl Into<String>, level: u32) -> Result<HeadingElement> {
        HeadingElement::new(self.id, content, level)
    }

    /// Create an image element.
    pub fn image(self, src: impl Into<String>, alt: impl Into<String>) -> ImageElement {
        ImageElement::new(self.id, src, alt)
    }

    /// Create a button element.
    pub fn button(self, label: impl Into<String>) -> ButtonElement {
        ButtonElement::new(self.id, label)
    }

    /// Create a text input element.
    pub fn text_input(self, label: impl Into<String>) -> TextInputElement {
        TextInputElement::new(self.id, label)
    }

    /// Create a container element.
    pub fn container(self) -> ContainerElement {
        ContainerElement::new(self.id)
    }

    /// Create a success feedback element.
    pub fn success(self, message: impl Into<String>) -> FeedbackElement {
        FeedbackElement::new(self.id, FeedbackType::Success, message)
    }

    /// Create an error feedback element.
    pub fn error(self, message: impl Into<String>) -> FeedbackElement {
        FeedbackElement::new(self.id, FeedbackType::Error, message)
    }

    /// Create a warning feedback element.
    pub fn warning(self, message: impl Into<String>) -> FeedbackElement {
        FeedbackElement::new(self.id, FeedbackType::Warning, message)
    }

    /// Create an info feedback element.
    pub fn info(self, message: impl Into<String>) -> FeedbackElement {
        FeedbackElement::new(self.id, FeedbackType::Info, message)
    }

    /// Create a metric element.
    pub fn metric(self, label: impl Into<String>, value: impl Into<String>) -> MetricElement {
        MetricElement::new(self.id, label, value)
    }
}

/// Factory for creating elements.
pub struct ElementFactory;

impl ElementFactory {
    /// Create a new element builder.
    pub fn build(id: ElementId) -> ElementBuilder {
        ElementBuilder::new(id)
    }

    /// Create a text element directly.
    pub fn text(id: ElementId, content: impl Into<String>) -> TextElement {
        TextElement::new(id, content)
    }

    /// Create a heading element directly.
    pub fn heading(id: ElementId, content: impl Into<String>, level: u32) -> Result<HeadingElement> {
        HeadingElement::new(id, content, level)
    }

    /// Create an image element directly.
    pub fn image(id: ElementId, src: impl Into<String>, alt: impl Into<String>) -> ImageElement {
        ImageElement::new(id, src, alt)
    }

    /// Create a button element directly.
    pub fn button(id: ElementId, label: impl Into<String>) -> ButtonElement {
        ButtonElement::new(id, label)
    }

    /// Create a text input element directly.
    pub fn text_input(id: ElementId, label: impl Into<String>) -> TextInputElement {
        TextInputElement::new(id, label)
    }

    /// Create a container element directly.
    pub fn container(id: ElementId) -> ContainerElement {
        ContainerElement::new(id)
    }

    /// Create a success feedback element directly.
    pub fn success(id: ElementId, message: impl Into<String>) -> FeedbackElement {
        FeedbackElement::new(id, FeedbackType::Success, message)
    }

    /// Create an error feedback element directly.
    pub fn error(id: ElementId, message: impl Into<String>) -> FeedbackElement {
        FeedbackElement::new(id, FeedbackType::Error, message)
    }

    /// Create a warning feedback element directly.
    pub fn warning(id: ElementId, message: impl Into<String>) -> FeedbackElement {
        FeedbackElement::new(id, FeedbackType::Warning, message)
    }

    /// Create an info feedback element directly.
    pub fn info(id: ElementId, message: impl Into<String>) -> FeedbackElement {
        FeedbackElement::new(id, FeedbackType::Info, message)
    }

    /// Create a metric element directly.
    pub fn metric(id: ElementId, label: impl Into<String>, value: impl Into<String>) -> MetricElement {
        MetricElement::new(id, label, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_builder() {
        let id = ElementId::new(1);
        let text = ElementBuilder::new(id).text("Hello");
        assert_eq!(text.content(), "Hello");
    }

    #[test]
    fn test_element_builder_button() {
        let id = ElementId::new(1);
        let button = ElementBuilder::new(id).button("Click me");
        assert_eq!(button.label(), "Click me");
    }

    #[test]
    fn test_element_factory() {
        let id = ElementId::new(1);
        let text = ElementFactory::text(id, "Hello");
        assert_eq!(text.content(), "Hello");
    }

    #[test]
    fn test_element_factory_heading() {
        let id = ElementId::new(1);
        let heading = ElementFactory::heading(id, "Title", 1).unwrap();
        assert_eq!(heading.level(), 1);
    }

    #[test]
    fn test_element_factory_feedback() {
        let id = ElementId::new(1);
        let success = ElementFactory::success(id, "Success!");
        assert_eq!(success.feedback_type(), FeedbackType::Success);
    }
}
