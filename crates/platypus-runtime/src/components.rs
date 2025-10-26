//! Custom component framework for Platypus
//! Allows developers to create reusable custom components

use std::collections::HashMap;
use serde_json::{json, Value};

/// Component metadata
#[derive(Clone, Debug)]
pub struct ComponentMetadata {
    pub name: String,
    pub version: String,
    pub author: Option<String>,
    pub description: Option<String>,
}

impl ComponentMetadata {
    /// Create new component metadata
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        ComponentMetadata {
            name: name.into(),
            version: version.into(),
            author: None,
            description: None,
        }
    }

    /// Set author
    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Set description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// Component property definition
#[derive(Clone, Debug)]
pub struct ComponentProperty {
    pub name: String,
    pub prop_type: String,
    pub required: bool,
    pub default: Option<Value>,
    pub description: Option<String>,
}

impl ComponentProperty {
    /// Create new property
    pub fn new(name: impl Into<String>, prop_type: impl Into<String>) -> Self {
        ComponentProperty {
            name: name.into(),
            prop_type: prop_type.into(),
            required: false,
            default: None,
            description: None,
        }
    }

    /// Mark as required
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// Set default value
    pub fn with_default(mut self, default: Value) -> Self {
        self.default = Some(default);
        self
    }

    /// Set description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// Custom component definition
#[derive(Clone, Debug)]
pub struct CustomComponent {
    metadata: ComponentMetadata,
    properties: HashMap<String, ComponentProperty>,
    render_fn: String, // JSON representation of render function
}

impl CustomComponent {
    /// Create new custom component
    pub fn new(metadata: ComponentMetadata) -> Self {
        CustomComponent {
            metadata,
            properties: HashMap::new(),
            render_fn: String::new(),
        }
    }

    /// Add property
    pub fn add_property(&mut self, property: ComponentProperty) {
        self.properties.insert(property.name.clone(), property);
    }

    /// Get property
    pub fn get_property(&self, name: &str) -> Option<&ComponentProperty> {
        self.properties.get(name)
    }

    /// Get all properties
    pub fn properties(&self) -> Vec<&ComponentProperty> {
        self.properties.values().collect()
    }

    /// Set render function
    pub fn set_render_fn(&mut self, render_fn: String) {
        self.render_fn = render_fn;
    }

    /// Get metadata
    pub fn metadata(&self) -> &ComponentMetadata {
        &self.metadata
    }

    /// Validate component properties
    pub fn validate(&self) -> Result<(), String> {
        for property in self.properties.values() {
            if property.required && property.default.is_none() {
                return Err(format!("Required property '{}' has no default", property.name));
            }
        }
        Ok(())
    }

    /// Export as JSON
    pub fn to_json(&self) -> Value {
        json!({
            "name": self.metadata.name,
            "version": self.metadata.version,
            "author": self.metadata.author,
            "description": self.metadata.description,
            "properties": self.properties.iter().map(|(_, p)| {
                json!({
                    "name": p.name,
                    "type": p.prop_type,
                    "required": p.required,
                    "default": p.default,
                    "description": p.description
                })
            }).collect::<Vec<_>>()
        })
    }
}

/// Component registry for managing custom components
#[derive(Clone, Debug)]
pub struct ComponentRegistry {
    components: HashMap<String, CustomComponent>,
}

impl ComponentRegistry {
    /// Create new component registry
    pub fn new() -> Self {
        ComponentRegistry {
            components: HashMap::new(),
        }
    }

    /// Register component
    pub fn register(&mut self, component: CustomComponent) -> Result<(), String> {
        component.validate()?;
        self.components.insert(component.metadata.name.clone(), component);
        Ok(())
    }

    /// Get component
    pub fn get(&self, name: &str) -> Option<&CustomComponent> {
        self.components.get(name)
    }

    /// Get all components
    pub fn components(&self) -> Vec<&CustomComponent> {
        self.components.values().collect()
    }

    /// Unregister component
    pub fn unregister(&mut self, name: &str) -> Option<CustomComponent> {
        self.components.remove(name)
    }

    /// Get component count
    pub fn count(&self) -> usize {
        self.components.len()
    }

    /// List all component names
    pub fn list(&self) -> Vec<String> {
        self.components.keys().cloned().collect()
    }
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Component instance with props
#[derive(Clone, Debug)]
pub struct ComponentInstance {
    component_name: String,
    props: HashMap<String, Value>,
}

impl ComponentInstance {
    /// Create new component instance
    pub fn new(component_name: impl Into<String>) -> Self {
        ComponentInstance {
            component_name: component_name.into(),
            props: HashMap::new(),
        }
    }

    /// Set prop
    pub fn set_prop(&mut self, name: impl Into<String>, value: Value) {
        self.props.insert(name.into(), value);
    }

    /// Get prop
    pub fn get_prop(&self, name: &str) -> Option<&Value> {
        self.props.get(name)
    }

    /// Get all props
    pub fn props(&self) -> &HashMap<String, Value> {
        &self.props
    }

    /// Get component name
    pub fn component_name(&self) -> &str {
        &self.component_name
    }

    /// Export as JSON
    pub fn to_json(&self) -> Value {
        json!({
            "component": self.component_name,
            "props": self.props
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_metadata() {
        let metadata = ComponentMetadata::new("MyComponent", "1.0.0")
            .with_author("John Doe")
            .with_description("A custom component");
        
        assert_eq!(metadata.name, "MyComponent");
        assert_eq!(metadata.version, "1.0.0");
        assert_eq!(metadata.author, Some("John Doe".to_string()));
    }

    #[test]
    fn test_component_property() {
        let prop = ComponentProperty::new("title", "string")
            .required()
            .with_default(json!("Default Title"))
            .with_description("Component title");
        
        assert_eq!(prop.name, "title");
        assert!(prop.required);
        assert!(prop.default.is_some());
    }

    #[test]
    fn test_custom_component() {
        let metadata = ComponentMetadata::new("TestComponent", "1.0.0");
        let mut component = CustomComponent::new(metadata);
        
        component.add_property(ComponentProperty::new("title", "string").required().with_default(json!("Default")));
        
        assert_eq!(component.properties().len(), 1);
        assert!(component.validate().is_ok());
    }

    #[test]
    fn test_component_registry() {
        let mut registry = ComponentRegistry::new();
        
        let metadata = ComponentMetadata::new("Component1", "1.0.0");
        let component = CustomComponent::new(metadata);
        
        assert!(registry.register(component).is_ok());
        assert_eq!(registry.count(), 1);
    }

    #[test]
    fn test_component_instance() {
        let mut instance = ComponentInstance::new("MyComponent");
        instance.set_prop("title", json!("Hello"));
        
        assert_eq!(instance.component_name(), "MyComponent");
        assert_eq!(instance.get_prop("title"), Some(&json!("Hello")));
    }
}
