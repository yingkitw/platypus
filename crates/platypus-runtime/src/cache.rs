//! Caching framework for Platypus
//! Provides @st.cache_data and @st.cache_resource decorators

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Cache entry with TTL support
#[derive(Clone, Debug)]
pub struct CacheEntry {
    data: String,
    created_at: Instant,
    ttl: Option<Duration>,
}

impl CacheEntry {
    /// Check if cache entry is still valid
    pub fn is_valid(&self) -> bool {
        match self.ttl {
            Some(ttl) => self.created_at.elapsed() < ttl,
            None => true,
        }
    }
}

/// Cache for data (st.cache_data)
/// Caches function results with automatic invalidation
#[derive(Clone)]
pub struct DataCache {
    cache: Arc<Mutex<HashMap<String, CacheEntry>>>,
}

impl DataCache {
    /// Create a new data cache
    pub fn new() -> Self {
        DataCache {
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Get cached value
    pub fn get(&self, key: &str) -> Option<String> {
        let mut cache = self.cache.lock().unwrap();
        
        if let Some(entry) = cache.get(key) {
            if entry.is_valid() {
                return Some(entry.data.clone());
            } else {
                cache.remove(key);
            }
        }
        None
    }

    /// Set cached value with optional TTL
    pub fn set(&self, key: String, value: String, ttl: Option<Duration>) {
        let mut cache = self.cache.lock().unwrap();
        cache.insert(
            key,
            CacheEntry {
                data: value,
                created_at: Instant::now(),
                ttl,
            },
        );
    }

    /// Clear all cache entries
    pub fn clear(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }

    /// Get cache size
    pub fn size(&self) -> usize {
        let cache = self.cache.lock().unwrap();
        cache.len()
    }

    /// Remove expired entries
    pub fn cleanup(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.retain(|_, entry| entry.is_valid());
    }
}

impl Default for DataCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Cache for resources (st.cache_resource)
/// Caches expensive resources with lifecycle management
#[derive(Clone)]
pub struct ResourceCache {
    cache: Arc<Mutex<HashMap<String, CacheEntry>>>,
}

impl ResourceCache {
    /// Create a new resource cache
    pub fn new() -> Self {
        ResourceCache {
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Get cached resource
    pub fn get(&self, key: &str) -> Option<String> {
        let cache = self.cache.lock().unwrap();
        
        if let Some(entry) = cache.get(key) {
            if entry.is_valid() {
                return Some(entry.data.clone());
            }
        }
        None
    }

    /// Set cached resource (no TTL, persists for session)
    pub fn set(&self, key: String, value: String) {
        let mut cache = self.cache.lock().unwrap();
        cache.insert(
            key,
            CacheEntry {
                data: value,
                created_at: Instant::now(),
                ttl: None,
            },
        );
    }

    /// Clear all resources
    pub fn clear(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }

    /// Get resource count
    pub fn count(&self) -> usize {
        let cache = self.cache.lock().unwrap();
        cache.len()
    }
}

impl Default for ResourceCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Global cache manager
pub struct CacheManager {
    data_cache: DataCache,
    resource_cache: ResourceCache,
}

impl CacheManager {
    /// Create a new cache manager
    pub fn new() -> Self {
        CacheManager {
            data_cache: DataCache::new(),
            resource_cache: ResourceCache::new(),
        }
    }

    /// Get data cache
    pub fn data_cache(&self) -> &DataCache {
        &self.data_cache
    }

    /// Get resource cache
    pub fn resource_cache(&self) -> &ResourceCache {
        &self.resource_cache
    }

    /// Clear all caches
    pub fn clear_all(&self) {
        self.data_cache.clear();
        self.resource_cache.clear();
    }

    /// Get total cache size
    pub fn total_size(&self) -> usize {
        self.data_cache.size() + self.resource_cache.count()
    }
}

impl Default for CacheManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_cache_basic() {
        let cache = DataCache::new();
        
        cache.set("key1".to_string(), "value1".to_string(), None);
        assert_eq!(cache.get("key1"), Some("value1".to_string()));
    }

    #[test]
    fn test_data_cache_ttl() {
        let cache = DataCache::new();
        
        cache.set(
            "key1".to_string(),
            "value1".to_string(),
            Some(Duration::from_millis(100)),
        );
        
        assert_eq!(cache.get("key1"), Some("value1".to_string()));
        std::thread::sleep(Duration::from_millis(150));
        assert_eq!(cache.get("key1"), None);
    }

    #[test]
    fn test_resource_cache() {
        let cache = ResourceCache::new();
        
        cache.set("resource1".to_string(), "data1".to_string());
        assert_eq!(cache.get("resource1"), Some("data1".to_string()));
    }

    #[test]
    fn test_cache_manager() {
        let manager = CacheManager::new();
        
        manager.data_cache().set("key1".to_string(), "value1".to_string(), None);
        manager.resource_cache().set("resource1".to_string(), "data1".to_string());
        
        assert_eq!(manager.total_size(), 2);
        
        manager.clear_all();
        assert_eq!(manager.total_size(), 0);
    }
}
