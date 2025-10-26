//! Element types and traits for UI components.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Unique identifier for elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ElementId(u64);

impl ElementId {
    /// Create a new element ID.
    pub fn new(id: u64) -> Self {
        ElementId(id)
    }

    /// Get the inner value.
    pub fn inner(self) -> u64 {
        self.0
    }
}

impl fmt::Display for ElementId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Enumeration of all supported element types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ElementType {
    // Text elements
    Text { value: String },
    Markdown { value: String },
    Code { value: String, language: Option<String> },
    Heading { value: String, level: u32 },

    // Input widgets
    Button { label: String, key: Option<String> },
    TextInput { label: String, value: String, key: Option<String> },
    TextArea { label: String, value: String, key: Option<String> },
    NumberInput { label: String, value: f64, key: Option<String> },
    Slider { label: String, value: f64, min: f64, max: f64, key: Option<String> },
    Checkbox { label: String, value: bool, key: Option<String> },
    Radio { label: String, options: Vec<String>, value: Option<String>, key: Option<String> },
    Selectbox { label: String, options: Vec<String>, value: Option<String>, key: Option<String> },
    Multiselect { label: String, options: Vec<String>, values: Vec<String>, key: Option<String> },
    DateInput { label: String, value: String, key: Option<String> },
    TimeInput { label: String, value: String, key: Option<String> },
    ColorPicker { label: String, value: String, key: Option<String> },
    FileUploader { label: String, key: Option<String> },

    // Data display
    Json { value: serde_json::Value },
    Dataframe { data: String }, // JSON-encoded dataframe
    Table { headers: Vec<String>, rows: Vec<Vec<String>> },
    CameraInput { label: String, key: Option<String> },

    // Layout
    Container { children: Vec<ElementId> },
    Column { children: Vec<ElementId>, width: Option<f32> },
    Row { children: Vec<ElementId> },
    Tab { label: String, children: Vec<ElementId> },
    Expander { label: String, expanded: bool, children: Vec<ElementId> },

    // Media
    Image { src: String, caption: Option<String>, width: Option<u32> },
    Audio { src: String },
    Video { src: String },

    // Feedback
    Success { message: String },
    Error { message: String },
    Warning { message: String },
    Info { message: String },
    Progress { value: f32 },

    // Advanced Layout
    Tabs { tabs: Vec<(String, Vec<ElementId>)> },
    Sidebar { children: Vec<ElementId> },
    Metric { label: String, value: String, delta: Option<String> },

    // Charts
    LineChart { data: String, title: Option<String> },
    BarChart { data: String, title: Option<String> },
    AreaChart { data: String, title: Option<String> },
    ScatterChart { data: String, title: Option<String> },
    PieChart { data: String, title: Option<String> },
    PlotlyChart { spec: String },
    VegaLiteChart { spec: String },
    BokehChart { spec: String },

    // Other
    Empty,
    Divider,
}

/// Trait for UI elements.
pub trait Element: Send + Sync {
    /// Get the element ID.
    fn id(&self) -> ElementId;

    /// Get the element type.
    fn element_type(&self) -> &ElementType;

    /// Convert to JSON representation.
    fn to_json(&self) -> serde_json::Result<serde_json::Value> {
        serde_json::to_value(self.element_type())
    }
}

/// A simple element implementation.
#[derive(Debug, Clone)]
pub struct SimpleElement {
    id: ElementId,
    element_type: ElementType,
}

impl SimpleElement {
    /// Create a new simple element.
    pub fn new(id: ElementId, element_type: ElementType) -> Self {
        SimpleElement { id, element_type }
    }
}

impl Element for SimpleElement {
    fn id(&self) -> ElementId {
        self.id
    }

    fn element_type(&self) -> &ElementType {
        &self.element_type
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_id() {
        let id = ElementId::new(42);
        assert_eq!(id.inner(), 42);
        assert_eq!(id.to_string(), "42");
    }

    #[test]
    fn test_simple_element() {
        let id = ElementId::new(1);
        let elem = SimpleElement::new(id, ElementType::Text { value: "Hello".to_string() });
        assert_eq!(elem.id(), id);
    }
}
