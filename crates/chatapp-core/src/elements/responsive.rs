//! Responsive element implementations with breakpoint support.

use crate::element::ElementId;
use crate::traits::{Renderable, Responsive, Breakpoint, Layout};
use crate::error::Result;
use serde_json::Value;
use std::any::Any;
use super::BaseElement;

/// Responsive container that adapts to different screen sizes.
#[derive(Debug, Clone)]
pub struct ResponsiveContainerElement {
    base: BaseElement,
    children: Vec<ElementId>,
    breakpoints: Vec<Breakpoint>,
    current_breakpoint: String,
}

impl ResponsiveContainerElement {
    /// Create a new responsive container.
    pub fn new(id: ElementId) -> Self {
        let breakpoints = vec![
            Breakpoint {
                name: "mobile".to_string(),
                min_width: 0,
                max_width: Some(640),
            },
            Breakpoint {
                name: "tablet".to_string(),
                min_width: 641,
                max_width: Some(1024),
            },
            Breakpoint {
                name: "desktop".to_string(),
                min_width: 1025,
                max_width: None,
            },
        ];

        Self {
            base: BaseElement::new(id, "responsive_container"),
            children: Vec::new(),
            breakpoints,
            current_breakpoint: "desktop".to_string(),
        }
    }

    /// Add a child element.
    pub fn add_child(&mut self, child_id: ElementId) {
        self.children.push(child_id);
    }

    /// Get children.
    pub fn children(&self) -> &[ElementId] {
        &self.children
    }

    /// Set the current breakpoint.
    pub fn set_current_breakpoint(&mut self, breakpoint: String) {
        self.current_breakpoint = breakpoint;
    }

    /// Get the current breakpoint.
    pub fn current_breakpoint(&self) -> &str {
        &self.current_breakpoint
    }
}

impl Renderable for ResponsiveContainerElement {
    fn id(&self) -> ElementId {
        self.base.id()
    }

    fn name(&self) -> &str {
        self.base.name()
    }

    fn to_json(&self) -> Result<Value> {
        Ok(serde_json::json!({
            "id": self.id().inner(),
            "type": "responsive_container",
            "children": self.children.iter().map(|id| id.inner()).collect::<Vec<_>>(),
            "current_breakpoint": self.current_breakpoint,
        }))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Responsive for ResponsiveContainerElement {
    fn breakpoints(&self) -> Vec<Breakpoint> {
        self.breakpoints.clone()
    }

    fn layout_for_breakpoint(&self, breakpoint: &str) -> Option<Layout> {
        match breakpoint {
            "mobile" => Some(Layout {
                display: "flex".to_string(),
                flex_direction: Some("column".to_string()),
                width: Some("100%".to_string()),
                height: None,
            }),
            "tablet" => Some(Layout {
                display: "grid".to_string(),
                flex_direction: Some("row".to_string()),
                width: Some("100%".to_string()),
                height: None,
            }),
            "desktop" => Some(Layout {
                display: "grid".to_string(),
                flex_direction: Some("row".to_string()),
                width: Some("1200px".to_string()),
                height: None,
            }),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_responsive_container() {
        let container = ResponsiveContainerElement::new(ElementId::new(1));
        assert_eq!(container.current_breakpoint(), "desktop");
        assert_eq!(container.breakpoints().len(), 3);
    }

    #[test]
    fn test_responsive_breakpoints() {
        let container = ResponsiveContainerElement::new(ElementId::new(1));
        let breakpoints = container.breakpoints();
        assert_eq!(breakpoints[0].name, "mobile");
        assert_eq!(breakpoints[1].name, "tablet");
        assert_eq!(breakpoints[2].name, "desktop");
    }

    #[test]
    fn test_responsive_layout() {
        let container = ResponsiveContainerElement::new(ElementId::new(1));
        let layout = container.layout_for_breakpoint("mobile").unwrap();
        assert_eq!(layout.display, "flex");
        assert_eq!(layout.flex_direction, Some("column".to_string()));
    }

    #[test]
    fn test_responsive_children() {
        let mut container = ResponsiveContainerElement::new(ElementId::new(1));
        container.add_child(ElementId::new(2));
        container.add_child(ElementId::new(3));
        assert_eq!(container.children().len(), 2);
    }
}
