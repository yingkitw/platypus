//! Final Features Tests - Custom Components & Secrets
//! Tests for the last 2 features completing 100% migration

use platypus_runtime::prelude::*;
use serde_json::json;

// ============================================================================
// CUSTOM COMPONENT TESTS
// ============================================================================

#[test]
fn test_component_metadata() {
    let metadata = ComponentMetadata::new("DataTable", "1.0.0")
        .with_author("Jane Doe")
        .with_description("A reusable data table component");
    
    assert_eq!(metadata.name, "DataTable");
    assert_eq!(metadata.version, "1.0.0");
    assert_eq!(metadata.author, Some("Jane Doe".to_string()));
}

#[test]
fn test_component_property() {
    let prop = ComponentProperty::new("columns", "array")
        .required()
        .with_default(json!([]))
        .with_description("Table columns");
    
    assert_eq!(prop.name, "columns");
    assert_eq!(prop.prop_type, "array");
    assert!(prop.required);
}

#[test]
fn test_custom_component_creation() {
    let metadata = ComponentMetadata::new("Button", "1.0.0");
    let mut component = CustomComponent::new(metadata);
    
    component.add_property(ComponentProperty::new("label", "string").required().with_default(json!("Click")));
    component.add_property(ComponentProperty::new("onClick", "function"));
    
    assert_eq!(component.properties().len(), 2);
    assert!(component.validate().is_ok());
}

#[test]
fn test_custom_component_validation() {
    let metadata = ComponentMetadata::new("Input", "1.0.0");
    let mut component = CustomComponent::new(metadata);
    
    // Add required property without default
    component.add_property(ComponentProperty::new("value", "string").required());
    
    // Should fail validation
    assert!(component.validate().is_err());
}

#[test]
fn test_component_registry() {
    let mut registry = ComponentRegistry::new();
    
    let metadata1 = ComponentMetadata::new("Component1", "1.0.0");
    let component1 = CustomComponent::new(metadata1);
    
    let metadata2 = ComponentMetadata::new("Component2", "1.0.0");
    let component2 = CustomComponent::new(metadata2);
    
    assert!(registry.register(component1).is_ok());
    assert!(registry.register(component2).is_ok());
    assert_eq!(registry.count(), 2);
}

#[test]
fn test_component_registry_retrieval() {
    let mut registry = ComponentRegistry::new();
    
    let metadata = ComponentMetadata::new("MyComponent", "1.0.0");
    let component = CustomComponent::new(metadata);
    
    registry.register(component).unwrap();
    
    assert!(registry.get("MyComponent").is_some());
    assert!(registry.get("NonExistent").is_none());
}

#[test]
fn test_component_registry_list() {
    let mut registry = ComponentRegistry::new();
    
    for i in 0..3 {
        let metadata = ComponentMetadata::new(format!("Component{}", i), "1.0.0");
        let component = CustomComponent::new(metadata);
        registry.register(component).unwrap();
    }
    
    let list = registry.list();
    assert_eq!(list.len(), 3);
}

#[test]
fn test_component_registry_unregister() {
    let mut registry = ComponentRegistry::new();
    
    let metadata = ComponentMetadata::new("TempComponent", "1.0.0");
    let component = CustomComponent::new(metadata);
    
    registry.register(component).unwrap();
    assert_eq!(registry.count(), 1);
    
    registry.unregister("TempComponent");
    assert_eq!(registry.count(), 0);
}

#[test]
fn test_component_instance() {
    let mut instance = ComponentInstance::new("DataTable");
    
    instance.set_prop("columns", json!(["Name", "Age", "Email"]));
    instance.set_prop("rows", json!([["John", 30, "john@example.com"]]));
    
    assert_eq!(instance.component_name(), "DataTable");
    assert!(instance.get_prop("columns").is_some());
}

#[test]
fn test_component_instance_json() {
    let mut instance = ComponentInstance::new("Button");
    instance.set_prop("label", json!("Click Me"));
    
    let json_repr = instance.to_json();
    assert_eq!(json_repr["component"], "Button");
    assert_eq!(json_repr["props"]["label"], "Click Me");
}

#[test]
fn test_component_to_json() {
    let metadata = ComponentMetadata::new("Form", "1.0.0")
        .with_description("A form component");
    let mut component = CustomComponent::new(metadata);
    
    component.add_property(ComponentProperty::new("fields", "array").required());
    
    let json_repr = component.to_json();
    assert_eq!(json_repr["name"], "Form");
    assert_eq!(json_repr["version"], "1.0.0");
}

// ============================================================================
// SECRETS MANAGEMENT TESTS
// ============================================================================

#[test]
fn test_secret_creation() {
    let secret = Secret::new("my_api_key_123", SecretSource::Memory);
    
    assert_eq!(secret.value(), "my_api_key_123");
    assert_eq!(secret.source(), &SecretSource::Memory);
}

#[test]
fn test_secret_masking() {
    let secret = Secret::new("password123", SecretSource::Memory);
    let masked = secret.masked();
    
    assert_eq!(masked, "p*********3");
    assert!(!masked.contains("password"));
}

#[test]
fn test_secret_masking_short() {
    let secret = Secret::new("ab", SecretSource::Memory);
    assert_eq!(secret.masked(), "**");
}

#[test]
fn test_secrets_manager_set_get() {
    let mut manager = SecretsManager::new();
    
    manager.set_secret("api_key", "secret_key_123");
    
    assert!(manager.has_secret("api_key"));
    assert_eq!(manager.get("api_key"), Some("secret_key_123".to_string()));
}

#[test]
fn test_secrets_manager_multiple() {
    let mut manager = SecretsManager::new();
    
    manager.set_secret("db_password", "db_pass_123");
    manager.set_secret("api_token", "token_456");
    manager.set_secret("jwt_secret", "jwt_789");
    
    assert_eq!(manager.count(), 3);
}

#[test]
fn test_secrets_manager_remove() {
    let mut manager = SecretsManager::new();
    
    manager.set_secret("key1", "value1");
    assert_eq!(manager.count(), 1);
    
    let removed = manager.remove_secret("key1");
    assert!(removed.is_some());
    assert_eq!(manager.count(), 0);
}

#[test]
fn test_secrets_manager_clear() {
    let mut manager = SecretsManager::new();
    
    manager.set_secret("key1", "value1");
    manager.set_secret("key2", "value2");
    manager.set_secret("key3", "value3");
    
    assert_eq!(manager.count(), 3);
    
    manager.clear();
    assert_eq!(manager.count(), 0);
}

#[test]
fn test_secrets_manager_keys() {
    let mut manager = SecretsManager::new();
    
    manager.set_secret("secret1", "value1");
    manager.set_secret("secret2", "value2");
    
    let keys = manager.keys();
    assert_eq!(keys.len(), 2);
    assert!(keys.contains(&"secret1".to_string()));
    assert!(keys.contains(&"secret2".to_string()));
}

#[test]
fn test_secrets_manager_validation() {
    let mut manager = SecretsManager::new();
    
    manager.set_secret("api_key", "key123");
    manager.set_secret("db_password", "pass123");
    
    // Validate existing secrets
    assert!(manager.validate_required(&["api_key", "db_password"]).is_ok());
    
    // Validate with missing secret
    assert!(manager.validate_required(&["api_key", "missing_secret"]).is_err());
}

#[test]
fn test_secrets_manager_get_secret() {
    let mut manager = SecretsManager::new();
    
    manager.set_secret("api_key", "key123");
    
    let secret = manager.get_secret("api_key");
    assert!(secret.is_some());
    assert_eq!(secret.unwrap().value(), "key123");
}

// ============================================================================
// INTEGRATED TESTS
// ============================================================================

#[test]
fn test_component_with_secrets() {
    let mut manager = SecretsManager::new();
    manager.set_secret("api_endpoint", "https://api.example.com");
    
    let metadata = ComponentMetadata::new("APIClient", "1.0.0");
    let component = CustomComponent::new(metadata);
    
    assert!(manager.has_secret("api_endpoint"));
    assert_eq!(component.metadata().name, "APIClient");
}

#[test]
fn test_component_registry_with_secrets() {
    let mut registry = ComponentRegistry::new();
    let mut secrets = SecretsManager::new();
    
    // Register components
    let metadata = ComponentMetadata::new("SecureForm", "1.0.0");
    let component = CustomComponent::new(metadata);
    registry.register(component).unwrap();
    
    // Store secrets
    secrets.set_secret("form_encryption_key", "secret_key_123");
    
    assert_eq!(registry.count(), 1);
    assert!(secrets.has_secret("form_encryption_key"));
}

#[test]
fn test_multiple_components_and_secrets() {
    let mut registry = ComponentRegistry::new();
    let mut secrets = SecretsManager::new();
    
    // Create multiple components
    for i in 0..5 {
        let metadata = ComponentMetadata::new(format!("Component{}", i), "1.0.0");
        let component = CustomComponent::new(metadata);
        registry.register(component).unwrap();
    }
    
    // Store multiple secrets
    for i in 0..5 {
        secrets.set_secret(format!("secret_{}", i), format!("value_{}", i));
    }
    
    assert_eq!(registry.count(), 5);
    assert_eq!(secrets.count(), 5);
}

// ============================================================================
// PERFORMANCE TESTS
// ============================================================================

#[test]
fn test_component_registry_performance() {
    let mut registry = ComponentRegistry::new();
    
    let start = std::time::Instant::now();
    
    for i in 0..100 {
        let metadata = ComponentMetadata::new(format!("Component{}", i), "1.0.0");
        let component = CustomComponent::new(metadata);
        registry.register(component).unwrap();
    }
    
    let duration = start.elapsed();
    assert!(duration.as_millis() < 100);
}

#[test]
fn test_secrets_manager_performance() {
    let mut manager = SecretsManager::new();
    
    let start = std::time::Instant::now();
    
    for i in 0..1000 {
        manager.set_secret(format!("secret_{}", i), format!("value_{}", i));
    }
    
    let duration = start.elapsed();
    assert!(duration.as_millis() < 100);
}

#[test]
fn test_component_instance_performance() {
    let start = std::time::Instant::now();
    
    for i in 0..1000 {
        let mut instance = ComponentInstance::new(format!("Component{}", i));
        instance.set_prop("prop1", json!("value1"));
        instance.set_prop("prop2", json!("value2"));
    }
    
    let duration = start.elapsed();
    assert!(duration.as_millis() < 100);
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn test_empty_component_registry() {
    let registry = ComponentRegistry::new();
    
    assert_eq!(registry.count(), 0);
    assert_eq!(registry.list().len(), 0);
    assert!(registry.get("NonExistent").is_none());
}

#[test]
fn test_empty_secrets_manager() {
    let manager = SecretsManager::new();
    
    assert_eq!(manager.count(), 0);
    assert_eq!(manager.keys().len(), 0);
    assert!(manager.get("NonExistent").is_none());
}

#[test]
fn test_secret_with_special_characters() {
    let secret = Secret::new("p@$$w0rd!#%&*()[]{}|;:,.<>?", SecretSource::Memory);
    
    assert_eq!(secret.value(), "p@$$w0rd!#%&*()[]{}|;:,.<>?");
    assert!(!secret.masked().contains("@$$w0rd"));
}

#[test]
fn test_component_property_with_complex_default() {
    let prop = ComponentProperty::new("config", "object")
        .with_default(json!({
            "theme": "dark",
            "language": "en",
            "features": ["feature1", "feature2"]
        }));
    
    assert!(prop.default.is_some());
}

#[test]
fn test_secrets_manager_duplicate_keys() {
    let mut manager = SecretsManager::new();
    
    manager.set_secret("key", "value1");
    manager.set_secret("key", "value2");
    
    // Should have only one entry (last write wins)
    assert_eq!(manager.count(), 1);
    assert_eq!(manager.get("key"), Some("value2".to_string()));
}
