//! Advanced elements - complex elements for specialized use cases.

use crate::element::ElementId;
use crate::traits::Renderable;
use crate::error::Result;
use serde_json::Value;
use std::any::Any;
use super::BaseElement;

/// Metric element for displaying key metrics.
#[derive(Debug, Clone)]
pub struct MetricElement {
    base: BaseElement,
    label: String,
    value: String,
    delta: Option<String>,
}

impl MetricElement {
    /// Create a new metric element.
    pub fn new(id: ElementId, label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            base: BaseElement::new(id, "metric"),
            label: label.into(),
            value: value.into(),
            delta: None,
        }
    }

    /// Set the delta (change) value.
    pub fn set_delta(&mut self, delta: impl Into<String>) {
        self.delta = Some(delta.into());
    }

    /// Get the label.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Get the value.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Get the delta.
    pub fn delta(&self) -> Option<&str> {
        self.delta.as_deref()
    }
}

impl Renderable for MetricElement {
    fn id(&self) -> ElementId {
        self.base.id()
    }

    fn name(&self) -> &str {
        self.base.name()
    }

    fn to_json(&self) -> Result<Value> {
        Ok(serde_json::json!({
            "id": self.id().inner(),
            "type": "metric",
            "label": self.label,
            "value": self.value,
            "delta": self.delta,
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
    fn test_metric_element() {
        let mut metric = MetricElement::new(ElementId::new(1), "Revenue", "$1,234.56");
        assert_eq!(metric.label(), "Revenue");
        assert_eq!(metric.value(), "$1,234.56");
        assert!(metric.delta().is_none());

        metric.set_delta("+5%");
        assert_eq!(metric.delta(), Some("+5%"));
    }

    #[test]
    fn test_metric_to_json() {
        let metric = MetricElement::new(ElementId::new(1), "Users", "1,000");
        let json = metric.to_json().unwrap();
        assert_eq!(json["type"], "metric");
        assert_eq!(json["label"], "Users");
    }
}
