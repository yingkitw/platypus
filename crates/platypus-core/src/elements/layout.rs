//! Layout elements - container elements for organizing content.

use crate::element::ElementId;
use crate::traits::{Renderable, Container};
use crate::error::Result;
use serde_json::Value;
use std::any::Any;
use super::BaseElement;

/// Container element for grouping other elements.
#[derive(Debug, Clone)]
pub struct ContainerElement {
    base: BaseElement,
    children: Vec<ElementId>,
}

impl ContainerElement {
    /// Create a new container element.
    pub fn new(id: ElementId) -> Self {
        Self {
            base: BaseElement::new(id, "container"),
            children: Vec::new(),
        }
    }
}

impl Renderable for ContainerElement {
    fn id(&self) -> ElementId {
        self.base.id()
    }

    fn name(&self) -> &str {
        self.base.name()
    }

    fn to_json(&self) -> Result<Value> {
        Ok(serde_json::json!({
            "id": self.id().inner(),
            "type": "container",
            "children": self.children.iter().map(|id| id.inner()).collect::<Vec<_>>(),
        }))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Container for ContainerElement {
    fn children(&self) -> Vec<ElementId> {
        self.children.clone()
    }

    fn add_child(&mut self, child_id: ElementId) -> Result<()> {
        self.children.push(child_id);
        Ok(())
    }

    fn remove_child(&mut self, child_id: ElementId) -> Result<()> {
        self.children.retain(|id| *id != child_id);
        Ok(())
    }

    fn clear_children(&mut self) {
        self.children.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_element() {
        let mut container = ContainerElement::new(ElementId::new(1));
        assert!(container.children().is_empty());

        container.add_child(ElementId::new(2)).unwrap();
        container.add_child(ElementId::new(3)).unwrap();

        assert_eq!(container.children().len(), 2);
    }

    #[test]
    fn test_container_remove_child() {
        let mut container = ContainerElement::new(ElementId::new(1));
        container.add_child(ElementId::new(2)).unwrap();
        container.add_child(ElementId::new(3)).unwrap();

        container.remove_child(ElementId::new(2)).unwrap();
        assert_eq!(container.children().len(), 1);
    }
}
