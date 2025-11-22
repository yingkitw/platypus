//! Application state and delta generation.

use crate::element::{Element, ElementId, ElementType, SimpleElement};
use crate::widget::{SimpleWidget, Widget, WidgetValue};
use dashmap::DashMap;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

/// Delta represents an incremental UI update.
#[derive(Debug, Clone)]
pub enum Delta {
    /// Add a new element.
    AddElement {
        id: ElementId,
        element: ElementType,
        parent_id: Option<ElementId>,
    },
    /// Update an existing element.
    UpdateElement { id: ElementId, element: ElementType },
    /// Remove an element.
    RemoveElement { id: ElementId },
    /// Clear all children of a container.
    ClearContainer { id: ElementId },
}

/// Generates UI deltas incrementally.
pub struct DeltaGenerator {
    elements: Arc<DashMap<ElementId, Box<dyn Element>>>,
    widgets: Arc<DashMap<String, Box<dyn Widget>>>,
    deltas: Arc<RwLock<Vec<Delta>>>,
    next_element_id: Arc<RwLock<u64>>,
}

impl DeltaGenerator {
    /// Create a new delta generator.
    pub fn new() -> Self {
        DeltaGenerator {
            elements: Arc::new(DashMap::new()),
            widgets: Arc::new(DashMap::new()),
            deltas: Arc::new(RwLock::new(Vec::new())),
            next_element_id: Arc::new(RwLock::new(1)),
        }
    }

    /// Get next element ID.
    pub fn next_element_id(&self) -> ElementId {
        let mut id = self.next_element_id.write();
        let current = *id;
        *id += 1;
        ElementId::new(current)
    }

    /// Add an element.
    pub fn add_element(
        &self,
        element_type: ElementType,
        parent_id: Option<ElementId>,
    ) -> ElementId {
        let id = self.next_element_id();
        let element = Box::new(SimpleElement::new(id, element_type.clone()));

        self.elements.insert(id, element);
        self.deltas.write().push(Delta::AddElement {
            id,
            element: element_type,
            parent_id,
        });

        id
    }

    /// Update an element.
    pub fn update_element(&self, id: ElementId, element_type: ElementType) {
        self.deltas.write().push(Delta::UpdateElement {
            id,
            element: element_type,
        });
    }

    /// Remove an element.
    pub fn remove_element(&self, id: ElementId) {
        self.elements.remove(&id);
        self.deltas.write().push(Delta::RemoveElement { id });
    }

    /// Clear container.
    pub fn clear_container(&self, id: ElementId) {
        self.deltas.write().push(Delta::ClearContainer { id });
    }

    /// Get all deltas and clear the list.
    pub fn take_deltas(&self) -> Vec<Delta> {
        std::mem::take(&mut *self.deltas.write())
    }

    /// Add or update a widget.
    pub fn set_widget(&self, key: String, value: WidgetValue) {
        let mut widget = SimpleWidget::new(key.clone(), value);
        widget.mark_changed();
        self.widgets.insert(key, Box::new(widget));
    }

    /// Get widget value.
    pub fn get_widget(&self, key: &str) -> Option<WidgetValue> {
        self.widgets.get(key).map(|widget_ref| (*widget_ref).value().clone())
    }

    /// Get all widgets.
    pub fn widgets(&self) -> HashMap<String, WidgetValue> {
        self.widgets
            .iter()
            .map(|entry| (entry.key().clone(), entry.value().value().clone()))
            .collect()
    }

    /// Get element by ID.
    pub fn get_element(&self, id: ElementId) -> Option<Box<dyn Element>> {
        self.elements.get(&id).map(|e| {
            // Clone the element type and create a new box
            let element_type = e.element_type().clone();
            Box::new(SimpleElement::new(id, element_type)) as Box<dyn Element>
        })
    }

    /// Get all elements.
    pub fn elements(&self) -> Vec<(ElementId, ElementType)> {
        self.elements
            .iter()
            .map(|entry| {
                let id = entry.key();
                let element_type = entry.value().element_type().clone();
                (*id, element_type)
            })
            .collect()
    }

    /// Clear all state.
    pub fn clear(&self) {
        self.elements.clear();
        self.widgets.clear();
        self.deltas.write().clear();
    }
}

impl Default for DeltaGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for DeltaGenerator {
    fn clone(&self) -> Self {
        DeltaGenerator {
            elements: Arc::clone(&self.elements),
            widgets: Arc::clone(&self.widgets),
            deltas: Arc::clone(&self.deltas),
            next_element_id: Arc::clone(&self.next_element_id),
        }
    }
}

/// Global application state.
pub struct AppState {
    /// Delta generator.
    pub delta_gen: DeltaGenerator,

    /// Session data.
    pub sessions: Arc<DashMap<String, crate::session::Session>>,
}

impl AppState {
    /// Create new app state.
    pub fn new() -> Self {
        AppState {
            delta_gen: DeltaGenerator::new(),
            sessions: Arc::new(DashMap::new()),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        AppState {
            delta_gen: self.delta_gen.clone(),
            sessions: Arc::clone(&self.sessions),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delta_generator() {
        let r#gen = DeltaGenerator::new();
        let id1 = r#gen.next_element_id();
        let id2 = r#gen.next_element_id();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_add_element() {
        let r#gen = DeltaGenerator::new();
        let id = r#gen.add_element(ElementType::Text { value: "Hello".to_string() }, None);
        assert!(r#gen.get_element(id).is_some());
    }

    #[test]
    fn test_deltas() {
        let r#gen = DeltaGenerator::new();
        r#gen.add_element(ElementType::Text { value: "Hello".to_string() }, None);
        let deltas = r#gen.take_deltas();
        assert_eq!(deltas.len(), 1);
    }

    #[test]
    fn test_widgets() {
        let r#gen = DeltaGenerator::new();
        r#gen.set_widget("test".to_string(), WidgetValue::String("value".to_string()));
        assert_eq!(r#gen.get_widget("test"), Some(WidgetValue::String("value".to_string())));
    }
}
