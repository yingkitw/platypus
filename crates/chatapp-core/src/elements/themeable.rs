//! Themeable element implementations with theme support.

use crate::element::ElementId;
use crate::traits::{Renderable, Themeable};
use crate::error::Result;
use serde_json::Value;
use std::any::Any;
use std::collections::HashMap;
use super::BaseElement;

/// Theme configuration.
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub colors: HashMap<String, String>,
    pub fonts: HashMap<String, String>,
    pub spacing: HashMap<String, String>,
}

impl Theme {
    /// Create a new theme.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            colors: HashMap::new(),
            fonts: HashMap::new(),
            spacing: HashMap::new(),
        }
    }

    /// Add a color to the theme.
    pub fn add_color(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.colors.insert(key.into(), value.into());
    }

    /// Add a font to the theme.
    pub fn add_font(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.fonts.insert(key.into(), value.into());
    }

    /// Add spacing to the theme.
    pub fn add_spacing(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.spacing.insert(key.into(), value.into());
    }
}

/// Themeable button element.
#[derive(Debug, Clone)]
pub struct ThemedButtonElement {
    base: BaseElement,
    label: String,
    theme: String,
    available_themes: Vec<String>,
    current_theme: Theme,
}

impl ThemedButtonElement {
    /// Create a new themed button.
    pub fn new(id: ElementId, label: impl Into<String>) -> Self {
        let mut default_theme = Theme::new("light");
        default_theme.add_color("primary", "#0066CC");
        default_theme.add_color("text", "#000000");
        default_theme.add_spacing("padding", "8px 16px");

        Self {
            base: BaseElement::new(id, "themed_button"),
            label: label.into(),
            theme: "light".to_string(),
            available_themes: vec!["light".to_string(), "dark".to_string()],
            current_theme: default_theme,
        }
    }

    /// Get the button label.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Get the current theme colors.
    pub fn theme_colors(&self) -> &HashMap<String, String> {
        &self.current_theme.colors
    }
}

impl Renderable for ThemedButtonElement {
    fn id(&self) -> ElementId {
        self.base.id()
    }

    fn name(&self) -> &str {
        self.base.name()
    }

    fn to_json(&self) -> Result<Value> {
        Ok(serde_json::json!({
            "id": self.id().inner(),
            "type": "themed_button",
            "label": self.label,
            "theme": self.theme,
            "colors": self.current_theme.colors,
        }))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Themeable for ThemedButtonElement {
    fn current_theme(&self) -> &str {
        &self.theme
    }

    fn set_theme(&mut self, theme: &str) -> Result<()> {
        if !self.available_themes.contains(&theme.to_string()) {
            return Err(crate::error::Error::StateError(
                format!("Theme '{}' not available", theme)
            ));
        }

        self.theme = theme.to_string();

        // Update theme colors based on theme name
        match theme {
            "dark" => {
                self.current_theme.colors.insert("primary".to_string(), "#3399FF".to_string());
                self.current_theme.colors.insert("text".to_string(), "#FFFFFF".to_string());
            }
            "light" => {
                self.current_theme.colors.insert("primary".to_string(), "#0066CC".to_string());
                self.current_theme.colors.insert("text".to_string(), "#000000".to_string());
            }
            _ => {}
        }

        Ok(())
    }

    fn available_themes(&self) -> Vec<&str> {
        self.available_themes.iter().map(|s| s.as_str()).collect()
    }
}

/// Themeable text element.
#[derive(Debug, Clone)]
pub struct ThemedTextElement {
    base: BaseElement,
    content: String,
    theme: String,
    available_themes: Vec<String>,
}

impl ThemedTextElement {
    /// Create a new themed text element.
    pub fn new(id: ElementId, content: impl Into<String>) -> Self {
        Self {
            base: BaseElement::new(id, "themed_text"),
            content: content.into(),
            theme: "light".to_string(),
            available_themes: vec!["light".to_string(), "dark".to_string(), "high-contrast".to_string()],
        }
    }

    /// Get the content.
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Get the CSS class for the current theme.
    pub fn theme_css_class(&self) -> String {
        format!("theme-{}", self.theme)
    }
}

impl Renderable for ThemedTextElement {
    fn id(&self) -> ElementId {
        self.base.id()
    }

    fn name(&self) -> &str {
        self.base.name()
    }

    fn to_json(&self) -> Result<Value> {
        Ok(serde_json::json!({
            "id": self.id().inner(),
            "type": "themed_text",
            "content": self.content,
            "theme": self.theme,
            "css_class": self.theme_css_class(),
        }))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Themeable for ThemedTextElement {
    fn current_theme(&self) -> &str {
        &self.theme
    }

    fn set_theme(&mut self, theme: &str) -> Result<()> {
        if !self.available_themes.contains(&theme.to_string()) {
            return Err(crate::error::Error::StateError(
                format!("Theme '{}' not available", theme)
            ));
        }
        self.theme = theme.to_string();
        Ok(())
    }

    fn available_themes(&self) -> Vec<&str> {
        self.available_themes.iter().map(|s| s.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_creation() {
        let mut theme = Theme::new("custom");
        theme.add_color("primary", "#FF0000");
        assert_eq!(theme.colors.get("primary"), Some(&"#FF0000".to_string()));
    }

    #[test]
    fn test_themed_button() {
        let button = ThemedButtonElement::new(ElementId::new(1), "Click me");
        assert_eq!(button.current_theme(), "light");
        assert_eq!(button.available_themes().len(), 2);
    }

    #[test]
    fn test_themed_button_set_theme() {
        let mut button = ThemedButtonElement::new(ElementId::new(1), "Click me");
        button.set_theme("dark").unwrap();
        assert_eq!(button.current_theme(), "dark");
    }

    #[test]
    fn test_themed_button_invalid_theme() {
        let mut button = ThemedButtonElement::new(ElementId::new(1), "Click me");
        assert!(button.set_theme("invalid").is_err());
    }

    #[test]
    fn test_themed_text() {
        let text = ThemedTextElement::new(ElementId::new(1), "Hello");
        assert_eq!(text.current_theme(), "light");
        assert_eq!(text.theme_css_class(), "theme-light");
    }

    #[test]
    fn test_themed_text_set_theme() {
        let mut text = ThemedTextElement::new(ElementId::new(1), "Hello");
        text.set_theme("dark").unwrap();
        assert_eq!(text.current_theme(), "dark");
        assert_eq!(text.theme_css_class(), "theme-dark");
    }
}
