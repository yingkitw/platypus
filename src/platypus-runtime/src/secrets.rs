//! Secrets management for Platypus
//! Secure storage and retrieval of sensitive configuration

use std::collections::HashMap;
use std::env;

/// Secret value wrapper
#[derive(Clone, Debug)]
pub struct Secret {
    value: String,
    source: SecretSource,
}

/// Source of the secret
#[derive(Clone, Debug, PartialEq)]
pub enum SecretSource {
    /// From environment variable
    Environment,
    /// From secrets file
    File,
    /// From in-memory store
    Memory,
}

impl Secret {
    /// Create secret from value
    pub fn new(value: impl Into<String>, source: SecretSource) -> Self {
        Secret {
            value: value.into(),
            source,
        }
    }

    /// Get secret value
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Get secret source
    pub fn source(&self) -> &SecretSource {
        &self.source
    }

    /// Mask secret for logging (show only first and last char)
    pub fn masked(&self) -> String {
        if self.value.len() <= 2 {
            "*".repeat(self.value.len())
        } else {
            format!(
                "{}{}{}",
                &self.value[0..1],
                "*".repeat(self.value.len() - 2),
                &self.value[self.value.len() - 1..]
            )
        }
    }
}

/// Secrets manager
#[derive(Clone, Debug)]
pub struct SecretsManager {
    secrets: HashMap<String, Secret>,
}

impl SecretsManager {
    /// Create new secrets manager
    pub fn new() -> Self {
        SecretsManager {
            secrets: HashMap::new(),
        }
    }

    /// Load secret from environment variable
    pub fn load_env(&mut self, key: &str) -> Result<(), String> {
        match env::var(key) {
            Ok(value) => {
                self.secrets.insert(
                    key.to_string(),
                    Secret::new(value, SecretSource::Environment),
                );
                Ok(())
            }
            Err(_) => Err(format!("Environment variable '{}' not found", key)),
        }
    }

    /// Set secret in memory
    pub fn set_secret(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.secrets.insert(
            key.into(),
            Secret::new(value, SecretSource::Memory),
        );
    }

    /// Get secret
    pub fn get_secret(&self, key: &str) -> Option<&Secret> {
        self.secrets.get(key)
    }

    /// Get secret value
    pub fn get(&self, key: &str) -> Option<String> {
        self.secrets.get(key).map(|s| s.value().to_string())
    }

    /// Check if secret exists
    pub fn has_secret(&self, key: &str) -> bool {
        self.secrets.contains_key(key)
    }

    /// Remove secret
    pub fn remove_secret(&mut self, key: &str) -> Option<Secret> {
        self.secrets.remove(key)
    }

    /// Clear all secrets
    pub fn clear(&mut self) {
        self.secrets.clear();
    }

    /// Get all secret keys (without values)
    pub fn keys(&self) -> Vec<String> {
        self.secrets.keys().cloned().collect()
    }

    /// Get secret count
    pub fn count(&self) -> usize {
        self.secrets.len()
    }

    /// Load multiple secrets from environment
    pub fn load_env_batch(&mut self, keys: &[&str]) -> Result<(), String> {
        let mut errors = Vec::new();
        
        for key in keys {
            if let Err(e) = self.load_env(key) {
                errors.push(e);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(format!("Failed to load secrets: {}", errors.join(", ")))
        }
    }

    /// Validate required secrets exist
    pub fn validate_required(&self, required_keys: &[&str]) -> Result<(), String> {
        let missing: Vec<&str> = required_keys
            .iter()
            .filter(|key| !self.has_secret(key))
            .copied()
            .collect();

        if missing.is_empty() {
            Ok(())
        } else {
            Err(format!("Missing required secrets: {}", missing.join(", ")))
        }
    }
}

impl Default for SecretsManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global secrets store
pub struct Secrets;

impl Secrets {
    /// Get secret from environment
    pub fn get_env(key: &str) -> Result<String, String> {
        env::var(key).map_err(|_| format!("Secret '{}' not found", key))
    }

    /// Get optional secret from environment
    pub fn get_env_optional(key: &str) -> Option<String> {
        env::var(key).ok()
    }

    /// Check if secret exists in environment
    pub fn has_env(key: &str) -> bool {
        env::var(key).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_creation() {
        let secret = Secret::new("my_secret_value", SecretSource::Memory);
        assert_eq!(secret.value(), "my_secret_value");
        assert_eq!(secret.source(), &SecretSource::Memory);
    }

    #[test]
    fn test_secret_masking() {
        let secret = Secret::new("password123", SecretSource::Memory);
        let masked = secret.masked();
        assert_eq!(masked, "p*********3");
    }

    #[test]
    fn test_secrets_manager() {
        let mut manager = SecretsManager::new();
        
        manager.set_secret("api_key", "secret_key_123");
        assert!(manager.has_secret("api_key"));
        assert_eq!(manager.get("api_key"), Some("secret_key_123".to_string()));
    }

    #[test]
    fn test_secrets_manager_remove() {
        let mut manager = SecretsManager::new();
        
        manager.set_secret("key1", "value1");
        assert_eq!(manager.count(), 1);
        
        manager.remove_secret("key1");
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_secrets_manager_clear() {
        let mut manager = SecretsManager::new();
        
        manager.set_secret("key1", "value1");
        manager.set_secret("key2", "value2");
        assert_eq!(manager.count(), 2);
        
        manager.clear();
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_secrets_manager_keys() {
        let mut manager = SecretsManager::new();
        
        manager.set_secret("key1", "value1");
        manager.set_secret("key2", "value2");
        
        let keys = manager.keys();
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&"key1".to_string()));
    }

    #[test]
    fn test_secrets_validation() {
        let mut manager = SecretsManager::new();
        
        manager.set_secret("api_key", "key123");
        
        assert!(manager.validate_required(&["api_key"]).is_ok());
        assert!(manager.validate_required(&["missing_key"]).is_err());
    }
}
