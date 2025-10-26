//! St context - Main API for building Webag applications.

use platypus_core::element::{ElementId, ElementType};
use platypus_core::state::DeltaGenerator;

/// The main context for building Webag applications.
/// Provides an API similar to Streamlit's `st` module.
pub struct St {
    delta_gen: DeltaGenerator,
    current_container: Option<ElementId>,
}

impl St {
    /// Create a new St context.
    pub fn new() -> Self {
        St {
            delta_gen: DeltaGenerator::new(),
            current_container: None,
        }
    }

    /// Create from an existing delta generator.
    pub fn with_delta_gen(delta_gen: DeltaGenerator) -> Self {
        St {
            delta_gen,
            current_container: None,
        }
    }

    /// Get the delta generator.
    pub fn delta_gen(&self) -> &DeltaGenerator {
        &self.delta_gen
    }

    /// Display text.
    pub fn write(&mut self, text: impl Into<String>) -> ElementId {
        let text = text.into();
        self.delta_gen.add_element(
            ElementType::Text { value: text },
            self.current_container,
        )
    }

    /// Display markdown.
    pub fn markdown(&mut self, text: impl Into<String>) -> ElementId {
        let text = text.into();
        self.delta_gen.add_element(
            ElementType::Markdown { value: text },
            self.current_container,
        )
    }

    /// Display code.
    pub fn code(&mut self, code: impl Into<String>, language: Option<String>) -> ElementId {
        let code = code.into();
        self.delta_gen.add_element(
            ElementType::Code {
                value: code,
                language,
            },
            self.current_container,
        )
    }

    /// Display a heading.
    pub fn heading(&mut self, text: impl Into<String>, level: u32) -> ElementId {
        let text = text.into();
        self.delta_gen.add_element(
            ElementType::Heading {
                value: text,
                level,
            },
            self.current_container,
        )
    }

    /// Shorthand for level 1 heading.
    pub fn title(&mut self, text: impl Into<String>) -> ElementId {
        self.heading(text, 1)
    }

    /// Shorthand for level 2 heading.
    pub fn header(&mut self, text: impl Into<String>) -> ElementId {
        self.heading(text, 2)
    }

    /// Shorthand for level 3 heading.
    pub fn subheader(&mut self, text: impl Into<String>) -> ElementId {
        self.heading(text, 3)
    }

    /// Create a button.
    pub fn button(&mut self, label: impl Into<String>, key: Option<String>) -> bool {
        let label = label.into();
        self.delta_gen.add_element(
            ElementType::Button {
                label,
                key: key.clone(),
            },
            self.current_container,
        );

        // Check if button was clicked
        if let Some(key) = key {
            self.delta_gen
                .get_widget(&key)
                .map(|v| v.as_bool().unwrap_or(false))
                .unwrap_or(false)
        } else {
            false
        }
    }

    /// Create a text input.
    pub fn text_input(
        &mut self,
        label: impl Into<String>,
        value: impl Into<String>,
        key: Option<String>,
    ) -> String {
        let label = label.into();
        let value = value.into();
        let key_str = key.clone().unwrap_or_else(|| format!("text_input_{}", label));

        self.delta_gen.add_element(
            ElementType::TextInput {
                label,
                value: value.clone(),
                key: key.clone(),
            },
            self.current_container,
        );

        self.delta_gen
            .get_widget(&key_str)
            .and_then(|v| v.as_string().map(|s| s.to_string()))
            .unwrap_or(value)
    }

    /// Create a text area.
    pub fn text_area(
        &mut self,
        label: impl Into<String>,
        value: impl Into<String>,
        key: Option<String>,
    ) -> String {
        let label = label.into();
        let value = value.into();
        let key_str = key.clone().unwrap_or_else(|| format!("text_area_{}", label));

        self.delta_gen.add_element(
            ElementType::TextArea {
                label,
                value: value.clone(),
                key: key.clone(),
            },
            self.current_container,
        );

        self.delta_gen
            .get_widget(&key_str)
            .and_then(|v| v.as_string().map(|s| s.to_string()))
            .unwrap_or(value)
    }

    /// Create a number input.
    pub fn number_input(
        &mut self,
        label: impl Into<String>,
        value: f64,
        key: Option<String>,
    ) -> f64 {
        let label = label.into();
        let key_str = key.clone().unwrap_or_else(|| format!("number_input_{}", label));

        self.delta_gen.add_element(
            ElementType::NumberInput {
                label,
                value,
                key: key.clone(),
            },
            self.current_container,
        );

        self.delta_gen
            .get_widget(&key_str)
            .and_then(|v| v.as_number())
            .unwrap_or(value)
    }

    /// Create a slider.
    pub fn slider(
        &mut self,
        label: impl Into<String>,
        min: f64,
        max: f64,
        value: f64,
        key: Option<String>,
    ) -> f64 {
        let label = label.into();
        let key_str = key.clone().unwrap_or_else(|| format!("slider_{}", label));

        self.delta_gen.add_element(
            ElementType::Slider {
                label,
                value,
                min,
                max,
                key: key.clone(),
            },
            self.current_container,
        );

        self.delta_gen
            .get_widget(&key_str)
            .and_then(|v| v.as_number())
            .unwrap_or(value)
    }

    /// Create a checkbox.
    pub fn checkbox(
        &mut self,
        label: impl Into<String>,
        value: bool,
        key: Option<String>,
    ) -> bool {
        let label = label.into();
        let key_str = key.clone().unwrap_or_else(|| format!("checkbox_{}", label));

        self.delta_gen.add_element(
            ElementType::Checkbox {
                label,
                value,
                key: key.clone(),
            },
            self.current_container,
        );

        self.delta_gen
            .get_widget(&key_str)
            .and_then(|v| v.as_bool())
            .unwrap_or(value)
    }

    /// Create a selectbox.
    pub fn selectbox(
        &mut self,
        label: impl Into<String>,
        options: Vec<String>,
        index: usize,
        key: Option<String>,
    ) -> String {
        let label = label.into();
        let default = options.get(index).cloned().unwrap_or_default();
        let key_str = key.clone().unwrap_or_else(|| format!("selectbox_{}", label));

        self.delta_gen.add_element(
            ElementType::Selectbox {
                label,
                options,
                value: Some(default.clone()),
                key: key.clone(),
            },
            self.current_container,
        );

        self.delta_gen
            .get_widget(&key_str)
            .and_then(|v| v.as_string().map(|s| s.to_string()))
            .unwrap_or(default)
    }

    /// Create a multiselect.
    pub fn multiselect(
        &mut self,
        label: impl Into<String>,
        options: Vec<String>,
        default: Vec<String>,
        key: Option<String>,
    ) -> Vec<String> {
        let label = label.into();
        let key_str = key.clone().unwrap_or_else(|| format!("multiselect_{}", label));

        self.delta_gen.add_element(
            ElementType::Multiselect {
                label,
                options,
                values: default.clone(),
                key: key.clone(),
            },
            self.current_container,
        );

        self.delta_gen
            .get_widget(&key_str)
            .and_then(|v| v.as_string_array().map(|a| a.to_vec()))
            .unwrap_or(default)
    }

    /// Display JSON.
    pub fn json(&mut self, value: serde_json::Value) -> ElementId {
        self.delta_gen.add_element(
            ElementType::Json { value },
            self.current_container,
        )
    }

    /// Display an image.
    pub fn image(
        &mut self,
        src: impl Into<String>,
        caption: Option<String>,
        width: Option<u32>,
    ) -> ElementId {
        let src = src.into();
        self.delta_gen.add_element(
            ElementType::Image { src, caption, width },
            self.current_container,
        )
    }

    /// Display success message.
    pub fn success(&mut self, message: impl Into<String>) -> ElementId {
        let message = message.into();
        self.delta_gen.add_element(
            ElementType::Success { message },
            self.current_container,
        )
    }

    /// Display error message.
    pub fn error(&mut self, message: impl Into<String>) -> ElementId {
        let message = message.into();
        self.delta_gen.add_element(
            ElementType::Error { message },
            self.current_container,
        )
    }

    /// Display warning message.
    pub fn warning(&mut self, message: impl Into<String>) -> ElementId {
        let message = message.into();
        self.delta_gen.add_element(
            ElementType::Warning { message },
            self.current_container,
        )
    }

    /// Display info message.
    pub fn info(&mut self, message: impl Into<String>) -> ElementId {
        let message = message.into();
        self.delta_gen.add_element(
            ElementType::Info { message },
            self.current_container,
        )
    }

    /// Display progress bar.
    pub fn progress(&mut self, value: f32) -> ElementId {
        self.delta_gen.add_element(
            ElementType::Progress { value },
            self.current_container,
        )
    }

    /// Display divider.
    pub fn divider(&mut self) -> ElementId {
        self.delta_gen.add_element(
            ElementType::Divider,
            self.current_container,
        )
    }

    /// Display empty space.
    pub fn empty(&mut self) -> ElementId {
        self.delta_gen.add_element(
            ElementType::Empty,
            self.current_container,
        )
    }

    /// Create a container.
    pub fn container(&mut self) -> Container {
        let id = self.delta_gen.add_element(
            ElementType::Container { children: vec![] },
            self.current_container,
        );
        Container::new(id, self.delta_gen.clone())
    }

    /// Create columns.
    pub fn columns(&mut self, count: usize) -> Vec<Container> {
        let width = 1.0 / count as f32;
        (0..count)
            .map(|_| {
                let id = self.delta_gen.add_element(
                    ElementType::Column {
                        children: vec![],
                        width: Some(width),
                    },
                    self.current_container,
                );
                Container::new(id, self.delta_gen.clone())
            })
            .collect()
    }

    /// Create tabs.
    pub fn tabs(&mut self, labels: Vec<&str>) -> Vec<Container> {
        let tabs_data: Vec<(String, Vec<ElementId>)> = labels
            .iter()
            .map(|label| (label.to_string(), vec![]))
            .collect();

        let id = self.delta_gen.add_element(
            ElementType::Tabs { tabs: tabs_data },
            self.current_container,
        );

        labels
            .iter()
            .map(|_label| {
                let tab_id = self.delta_gen.add_element(
                    ElementType::Container { children: vec![] },
                    Some(id),
                );
                Container::new(tab_id, self.delta_gen.clone())
            })
            .collect()
    }

    /// Create an expander.
    pub fn expander(&mut self, label: impl Into<String>) -> Container {
        let label = label.into();
        let id = self.delta_gen.add_element(
            ElementType::Expander {
                label,
                expanded: false,
                children: vec![],
            },
            self.current_container,
        );
        Container::new(id, self.delta_gen.clone())
    }

    /// Display a metric.
    pub fn metric(
        &mut self,
        label: impl Into<String>,
        value: impl Into<String>,
        delta: Option<String>,
    ) -> ElementId {
        let label = label.into();
        let value = value.into();
        self.delta_gen.add_element(
            ElementType::Metric { label, value, delta },
            self.current_container,
        )
    }

    /// Get sidebar context.
    pub fn sidebar(&mut self) -> Container {
        let id = self.delta_gen.add_element(
            ElementType::Sidebar { children: vec![] },
            None,
        );
        Container::new(id, self.delta_gen.clone())
    }

    /// Create a date input.
    pub fn date_input(
        &mut self,
        label: impl Into<String>,
        value: impl Into<String>,
        key: Option<String>,
    ) -> String {
        let label = label.into();
        let value = value.into();
        let key_str = key.clone().unwrap_or_else(|| format!("date_input_{}", label));

        self.delta_gen.add_element(
            ElementType::DateInput {
                label,
                value: value.clone(),
                key: key.clone(),
            },
            self.current_container,
        );

        self.delta_gen
            .get_widget(&key_str)
            .and_then(|v| v.as_string().map(|s| s.to_string()))
            .unwrap_or(value)
    }

    /// Create a time input.
    pub fn time_input(
        &mut self,
        label: impl Into<String>,
        value: impl Into<String>,
        key: Option<String>,
    ) -> String {
        let label = label.into();
        let value = value.into();
        let key_str = key.clone().unwrap_or_else(|| format!("time_input_{}", label));

        self.delta_gen.add_element(
            ElementType::TimeInput {
                label,
                value: value.clone(),
                key: key.clone(),
            },
            self.current_container,
        );

        self.delta_gen
            .get_widget(&key_str)
            .and_then(|v| v.as_string().map(|s| s.to_string()))
            .unwrap_or(value)
    }

    /// Create a color picker.
    pub fn color_picker(
        &mut self,
        label: impl Into<String>,
        value: impl Into<String>,
        key: Option<String>,
    ) -> String {
        let label = label.into();
        let value = value.into();
        let key_str = key.clone().unwrap_or_else(|| format!("color_picker_{}", label));

        self.delta_gen.add_element(
            ElementType::ColorPicker {
                label,
                value: value.clone(),
                key: key.clone(),
            },
            self.current_container,
        );

        self.delta_gen
            .get_widget(&key_str)
            .and_then(|v| v.as_string().map(|s| s.to_string()))
            .unwrap_or(value)
    }

    /// Create a file uploader.
    pub fn file_uploader(
        &mut self,
        label: impl Into<String>,
        key: Option<String>,
    ) -> Option<String> {
        let label = label.into();
        let key_str = key.clone().unwrap_or_else(|| format!("file_uploader_{}", label));

        self.delta_gen.add_element(
            ElementType::FileUploader {
                label,
                key: key.clone(),
            },
            self.current_container,
        );

        self.delta_gen
            .get_widget(&key_str)
            .and_then(|v| v.as_string().map(|s| s.to_string()))
    }

    /// Create a radio button group.
    pub fn radio(
        &mut self,
        label: impl Into<String>,
        options: Vec<impl Into<String>>,
        index: usize,
        key: Option<String>,
    ) -> String {
        let label = label.into();
        let options: Vec<String> = options.into_iter().map(|o| o.into()).collect();
        let key_str = key.clone().unwrap_or_else(|| format!("radio_{}", label));
        let default_value = options.get(index).cloned().unwrap_or_default();

        self.delta_gen.add_element(
            ElementType::Radio {
                label,
                options: options.clone(),
                value: Some(default_value.clone()),
                key: key.clone(),
            },
            self.current_container,
        );

        self.delta_gen
            .get_widget(&key_str)
            .and_then(|v| v.as_string().map(|s| s.to_string()))
            .unwrap_or(default_value)
    }

    /// Display a table with headers and rows.
    pub fn table(&mut self, headers: Vec<impl Into<String>>, rows: Vec<Vec<impl Into<String>>>) {
        let headers: Vec<String> = headers.into_iter().map(|h| h.into()).collect();
        let rows: Vec<Vec<String>> = rows
            .into_iter()
            .map(|row| row.into_iter().map(|cell| cell.into()).collect())
            .collect();

        self.delta_gen.add_element(
            ElementType::Table { headers, rows },
            self.current_container,
        );
    }

    /// Display a dataframe from JSON string.
    pub fn dataframe(&mut self, data: impl Into<String>) {
        let data = data.into();
        self.delta_gen.add_element(
            ElementType::Dataframe { data },
            self.current_container,
        );
    }

    /// Create a camera input.
    pub fn camera_input(
        &mut self,
        label: impl Into<String>,
        key: Option<String>,
    ) -> Option<String> {
        let label = label.into();
        let key_str = key.clone().unwrap_or_else(|| format!("camera_{}", label));

        self.delta_gen.add_element(
            ElementType::CameraInput {
                label,
                key: key.clone(),
            },
            self.current_container,
        );

        self.delta_gen
            .get_widget(&key_str)
            .and_then(|v| v.as_string().map(|s| s.to_string()))
    }

    /// Get all deltas.
    pub fn take_deltas(&self) -> Vec<platypus_core::state::Delta> {
        self.delta_gen.take_deltas()
    }
}

impl Default for St {
    fn default() -> Self {
        Self::new()
    }
}

/// A container for organizing elements.
pub struct Container {
    id: ElementId,
    delta_gen: DeltaGenerator,
}

impl Container {
    /// Create a new container.
    pub fn new(id: ElementId, delta_gen: DeltaGenerator) -> Self {
        Container { id, delta_gen }
    }

    /// Get a mutable St context for this container.
    pub fn st(&self) -> St {
        let mut st = St::with_delta_gen(self.delta_gen.clone());
        st.current_container = Some(self.id);
        st
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_st_write() {
        let mut st = St::new();
        let id = st.write("Hello, World!");
        assert!(st.delta_gen.get_element(id).is_some());
    }

    #[test]
    fn test_st_title() {
        let mut st = St::new();
        let id = st.title("My App");
        assert!(st.delta_gen.get_element(id).is_some());
    }

    #[test]
    fn test_st_button() {
        let mut st = St::new();
        let clicked = st.button("Click me", Some("btn".to_string()));
        assert!(!clicked); // Not clicked in test
    }

    #[test]
    fn test_st_text_input() {
        let mut st = St::new();
        let value = st.text_input("Name", "John", Some("name".to_string()));
        assert_eq!(value, "John");
    }

    #[test]
    fn test_st_deltas() {
        let mut st = St::new();
        st.write("Hello");
        let deltas = st.take_deltas();
        assert_eq!(deltas.len(), 1);
    }
}
