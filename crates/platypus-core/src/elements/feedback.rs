//! Feedback elements - elements for displaying feedback messages.

use crate::element::ElementId;
use crate::traits::Renderable;
use crate::error::Result;
use serde_json::Value;
use std::any::Any;
use super::BaseElement;

/// Feedback message element.
#[derive(Debug, Clone)]
pub struct FeedbackElement {
    base: BaseElement,
    feedback_type: FeedbackType,
    message: String,
}

/// Type of feedback message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeedbackType {
    Success,
    Error,
    Warning,
    Info,
}

impl FeedbackElement {
    /// Create a new feedback element.
    pub fn new(id: ElementId, feedback_type: FeedbackType, message: impl Into<String>) -> Self {
        let type_name = match feedback_type {
            FeedbackType::Success => "success",
            FeedbackType::Error => "error",
            FeedbackType::Warning => "warning",
            FeedbackType::Info => "info",
        };

        Self {
            base: BaseElement::new(id, format!("feedback_{}", type_name)),
            feedback_type,
            message: message.into(),
        }
    }

    /// Get the feedback type.
    pub fn feedback_type(&self) -> FeedbackType {
        self.feedback_type
    }

    /// Get the message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl Renderable for FeedbackElement {
    fn id(&self) -> ElementId {
        self.base.id()
    }

    fn name(&self) -> &str {
        self.base.name()
    }

    fn to_json(&self) -> Result<Value> {
        let type_str = match self.feedback_type {
            FeedbackType::Success => "success",
            FeedbackType::Error => "error",
            FeedbackType::Warning => "warning",
            FeedbackType::Info => "info",
        };

        Ok(serde_json::json!({
            "id": self.id().inner(),
            "type": "feedback",
            "feedback_type": type_str,
            "message": self.message,
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
    fn test_feedback_element() {
        let feedback = FeedbackElement::new(ElementId::new(1), FeedbackType::Success, "Operation successful");
        assert_eq!(feedback.feedback_type(), FeedbackType::Success);
        assert_eq!(feedback.message(), "Operation successful");
    }

    #[test]
    fn test_feedback_to_json() {
        let feedback = FeedbackElement::new(ElementId::new(1), FeedbackType::Error, "An error occurred");
        let json = feedback.to_json().unwrap();
        assert_eq!(json["type"], "feedback");
        assert_eq!(json["feedback_type"], "error");
    }
}
