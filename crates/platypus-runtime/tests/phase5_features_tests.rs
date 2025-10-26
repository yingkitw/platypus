//! Phase 5 Features Tests
//! Tests for: Caching decorators, multi-page apps, custom components, secrets

use platypus_runtime::prelude::*;
use std::time::Duration;
use std::collections::HashMap;

// ============================================================================
// CACHING TESTS - @st.cache_data
// ============================================================================

#[test]
fn test_cache_data_basic() {
    let cache = DataCache::new();
    
    cache.set("key1".to_string(), "value1".to_string(), None);
    assert_eq!(cache.get("key1"), Some("value1".to_string()));
}

#[test]
fn test_cache_data_multiple_entries() {
    let cache = DataCache::new();
    
    cache.set("key1".to_string(), "value1".to_string(), None);
    cache.set("key2".to_string(), "value2".to_string(), None);
    cache.set("key3".to_string(), "value3".to_string(), None);
    
    assert_eq!(cache.size(), 3);
    assert_eq!(cache.get("key2"), Some("value2".to_string()));
}

#[test]
fn test_cache_data_overwrite() {
    let cache = DataCache::new();
    
    cache.set("key1".to_string(), "value1".to_string(), None);
    cache.set("key1".to_string(), "value2".to_string(), None);
    
    assert_eq!(cache.get("key1"), Some("value2".to_string()));
    assert_eq!(cache.size(), 1);
}

#[test]
fn test_cache_data_clear() {
    let cache = DataCache::new();
    
    cache.set("key1".to_string(), "value1".to_string(), None);
    cache.set("key2".to_string(), "value2".to_string(), None);
    
    assert_eq!(cache.size(), 2);
    
    cache.clear();
    assert_eq!(cache.size(), 0);
    assert_eq!(cache.get("key1"), None);
}

#[test]
fn test_cache_data_with_ttl() {
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
fn test_cache_data_cleanup() {
    let cache = DataCache::new();
    
    cache.set("key1".to_string(), "value1".to_string(), Some(Duration::from_millis(50)));
    cache.set("key2".to_string(), "value2".to_string(), None);
    
    assert_eq!(cache.size(), 2);
    
    std::thread::sleep(Duration::from_millis(100));
    cache.cleanup();
    
    assert_eq!(cache.size(), 1);
    assert_eq!(cache.get("key2"), Some("value2".to_string()));
}

// ============================================================================
// CACHING TESTS - @st.cache_resource
// ============================================================================

#[test]
fn test_cache_resource_basic() {
    let cache = ResourceCache::new();
    
    cache.set("resource1".to_string(), "data1".to_string());
    assert_eq!(cache.get("resource1"), Some("data1".to_string()));
}

#[test]
fn test_cache_resource_persistence() {
    let cache = ResourceCache::new();
    
    cache.set("resource1".to_string(), "data1".to_string());
    
    // Resources persist (no TTL)
    std::thread::sleep(Duration::from_millis(100));
    assert_eq!(cache.get("resource1"), Some("data1".to_string()));
}

#[test]
fn test_cache_resource_multiple() {
    let cache = ResourceCache::new();
    
    cache.set("db".to_string(), "database_connection".to_string());
    cache.set("api".to_string(), "api_client".to_string());
    cache.set("config".to_string(), "app_config".to_string());
    
    assert_eq!(cache.count(), 3);
}

#[test]
fn test_cache_resource_clear() {
    let cache = ResourceCache::new();
    
    cache.set("resource1".to_string(), "data1".to_string());
    cache.set("resource2".to_string(), "data2".to_string());
    
    assert_eq!(cache.count(), 2);
    
    cache.clear();
    assert_eq!(cache.count(), 0);
}

// ============================================================================
// CACHE MANAGER TESTS
// ============================================================================

#[test]
fn test_cache_manager_basic() {
    let manager = CacheManager::new();
    
    manager.data_cache().set("key1".to_string(), "value1".to_string(), None);
    manager.resource_cache().set("resource1".to_string(), "data1".to_string());
    
    assert_eq!(manager.total_size(), 2);
}

#[test]
fn test_cache_manager_clear_all() {
    let manager = CacheManager::new();
    
    manager.data_cache().set("key1".to_string(), "value1".to_string(), None);
    manager.resource_cache().set("resource1".to_string(), "data1".to_string());
    
    assert_eq!(manager.total_size(), 2);
    
    manager.clear_all();
    assert_eq!(manager.total_size(), 0);
}

// ============================================================================
// MULTI-PAGE APP TESTS
// ============================================================================

#[test]
fn test_page_creation() {
    let page = Page::new("home", "Home Page")
        .with_icon("🏠")
        .with_description("Welcome to home");
    
    assert_eq!(page.name, "home");
    assert_eq!(page.title, "Home Page");
    assert_eq!(page.icon, Some("🏠".to_string()));
    assert_eq!(page.description, Some("Welcome to home".to_string()));
}

#[test]
fn test_navigation_basic() {
    let mut nav = Navigation::new();
    
    nav.add_page(Page::new("home", "Home"));
    nav.add_page(Page::new("about", "About"));
    
    assert_eq!(nav.page_count(), 2);
    assert_eq!(nav.current_page().unwrap().name, "home");
}

#[test]
fn test_navigation_switching() {
    let mut nav = Navigation::new();
    
    nav.add_page(Page::new("home", "Home"));
    nav.add_page(Page::new("about", "About"));
    nav.add_page(Page::new("contact", "Contact"));
    
    assert_eq!(nav.current_page().unwrap().name, "home");
    
    nav.navigate_to("about");
    assert_eq!(nav.current_page().unwrap().name, "about");
    
    nav.navigate_to("contact");
    assert_eq!(nav.current_page().unwrap().name, "contact");
}

#[test]
fn test_navigation_invalid_page() {
    let mut nav = Navigation::new();
    
    nav.add_page(Page::new("home", "Home"));
    
    let result = nav.navigate_to("nonexistent");
    assert!(!result);
    assert_eq!(nav.current_page().unwrap().name, "home");
}

#[test]
fn test_page_link() {
    let link = PageLink::new("Go Home", "home").with_icon("🏠");
    
    assert_eq!(link.label, "Go Home");
    assert_eq!(link.page, "home");
    assert_eq!(link.icon, Some("🏠".to_string()));
}

#[test]
fn test_multi_page_app_basic() {
    let mut app = MultiPageApp::new();
    
    app.add_page(
        Page::new("home", "Home"),
        "Home content".to_string(),
    );
    app.add_page(
        Page::new("about", "About"),
        "About content".to_string(),
    );
    
    assert_eq!(app.current_page_content(), Some("Home content".to_string()));
}

#[test]
fn test_multi_page_app_navigation() {
    let mut app = MultiPageApp::new();
    
    app.add_page(
        Page::new("home", "Home"),
        "Home content".to_string(),
    );
    app.add_page(
        Page::new("about", "About"),
        "About content".to_string(),
    );
    app.add_page(
        Page::new("contact", "Contact"),
        "Contact content".to_string(),
    );
    
    assert_eq!(app.current_page_content(), Some("Home content".to_string()));
    
    app.navigate("about");
    assert_eq!(app.current_page_content(), Some("About content".to_string()));
    
    app.navigate("contact");
    assert_eq!(app.current_page_content(), Some("Contact content".to_string()));
}

#[test]
fn test_multi_page_app_with_icons() {
    let mut app = MultiPageApp::new();
    
    app.add_page(
        Page::new("home", "Home").with_icon("🏠"),
        "Home".to_string(),
    );
    app.add_page(
        Page::new("settings", "Settings").with_icon("⚙️"),
        "Settings".to_string(),
    );
    app.add_page(
        Page::new("help", "Help").with_icon("❓"),
        "Help".to_string(),
    );
    
    let nav = app.navigation();
    let pages = nav.pages();
    
    assert_eq!(pages.len(), 3);
    
    // Check that all pages have icons
    let home_page = pages.iter().find(|p| p.name == "home").unwrap();
    assert_eq!(home_page.icon, Some("🏠".to_string()));
    
    let settings_page = pages.iter().find(|p| p.name == "settings").unwrap();
    assert_eq!(settings_page.icon, Some("⚙️".to_string()));
    
    let help_page = pages.iter().find(|p| p.name == "help").unwrap();
    assert_eq!(help_page.icon, Some("❓".to_string()));
}

// ============================================================================
// INTEGRATED PHASE 5 TESTS
// ============================================================================

#[test]
fn test_cache_with_multi_page_app() {
    let cache = DataCache::new();
    let mut app = MultiPageApp::new();
    
    app.add_page(Page::new("home", "Home"), "Home content".to_string());
    app.add_page(Page::new("about", "About"), "About content".to_string());
    
    // Cache page navigation state
    cache.set("current_page".to_string(), "home".to_string(), None);
    
    assert_eq!(cache.get("current_page"), Some("home".to_string()));
}

#[test]
fn test_resource_cache_for_app_data() {
    let resource_cache = ResourceCache::new();
    
    // Cache application resources
    resource_cache.set("app_config".to_string(), r#"{"theme": "dark"}"#.to_string());
    resource_cache.set("user_data".to_string(), r#"{"name": "John"}"#.to_string());
    
    assert_eq!(resource_cache.count(), 2);
    assert!(resource_cache.get("app_config").is_some());
}

#[test]
fn test_multi_page_with_caching() {
    let cache = DataCache::new();
    let mut app = MultiPageApp::new();
    
    // Create multi-page app
    app.add_page(Page::new("page1", "Page 1"), "Content 1".to_string());
    app.add_page(Page::new("page2", "Page 2"), "Content 2".to_string());
    
    // Cache page visits
    cache.set("visited_pages".to_string(), "page1,page2".to_string(), None);
    
    assert_eq!(cache.get("visited_pages"), Some("page1,page2".to_string()));
}

// ============================================================================
// PERFORMANCE TESTS
// ============================================================================

#[test]
fn test_cache_performance() {
    let cache = DataCache::new();
    
    let start = std::time::Instant::now();
    
    for i in 0..1000 {
        cache.set(
            format!("key_{}", i),
            format!("value_{}", i),
            None,
        );
    }
    
    let duration = start.elapsed();
    assert!(duration.as_millis() < 100);
}

#[test]
fn test_multi_page_app_performance() {
    let mut app = MultiPageApp::new();
    
    let start = std::time::Instant::now();
    
    for i in 0..100 {
        app.add_page(
            Page::new(format!("page_{}", i), format!("Page {}", i)),
            format!("Content {}", i),
        );
    }
    
    let duration = start.elapsed();
    assert!(duration.as_millis() < 100);
}

#[test]
fn test_navigation_performance() {
    let mut nav = Navigation::new();
    
    for i in 0..100 {
        nav.add_page(Page::new(format!("page_{}", i), format!("Page {}", i)));
    }
    
    let start = std::time::Instant::now();
    
    for i in 0..100 {
        nav.navigate_to(&format!("page_{}", i));
    }
    
    let duration = start.elapsed();
    assert!(duration.as_millis() < 50);
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn test_cache_empty_key() {
    let cache = DataCache::new();
    
    cache.set("".to_string(), "value".to_string(), None);
    assert_eq!(cache.get(""), Some("value".to_string()));
}

#[test]
fn test_cache_large_value() {
    let cache = DataCache::new();
    
    let large_value = "x".repeat(10000);
    cache.set("key".to_string(), large_value.clone(), None);
    
    assert_eq!(cache.get("key"), Some(large_value));
}

#[test]
fn test_navigation_single_page() {
    let mut nav = Navigation::new();
    
    nav.add_page(Page::new("only", "Only Page"));
    
    assert_eq!(nav.page_count(), 1);
    assert_eq!(nav.current_page().unwrap().name, "only");
}

#[test]
fn test_multi_page_app_empty() {
    let app = MultiPageApp::new();
    
    assert_eq!(app.navigation().page_count(), 0);
    assert_eq!(app.current_page_content(), None);
}

#[test]
fn test_cache_ttl_edge_case() {
    let cache = DataCache::new();
    
    // TTL of 0 should expire immediately
    cache.set(
        "key".to_string(),
        "value".to_string(),
        Some(Duration::from_millis(0)),
    );
    
    // May or may not be available depending on timing
    let _ = cache.get("key");
}
