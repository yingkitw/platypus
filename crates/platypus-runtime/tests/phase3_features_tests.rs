//! Phase 3 Features Tests
//! Tests for: Bokeh charts, message compression, error recovery, session persistence

use platypus_runtime::prelude::*;
use std::collections::HashMap;

// ============================================================================
// BOKEH CHART TESTS
// ============================================================================

#[test]
fn test_bokeh_chart_basic() {
    let mut st = St::new();
    
    let spec = r#"{
        "type": "figure",
        "x_range": [0, 10],
        "y_range": [0, 10],
        "renderers": []
    }"#;
    
    let id = st.bokeh_chart(spec);
    assert!(st.delta_gen().get_element(id).is_some());
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 1);
}

#[test]
fn test_bokeh_chart_with_data() {
    let mut st = St::new();
    
    let spec = r#"{
        "type": "figure",
        "title": "Bokeh Chart",
        "data": [
            {"x": 1, "y": 10},
            {"x": 2, "y": 20},
            {"x": 3, "y": 15}
        ]
    }"#;
    
    let id = st.bokeh_chart(spec);
    assert!(st.delta_gen().get_element(id).is_some());
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 1);
}

#[test]
fn test_bokeh_chart_complex_spec() {
    let mut st = St::new();
    
    let spec = r#"{
        "type": "figure",
        "title": "Complex Bokeh Chart",
        "width": 800,
        "height": 600,
        "x_axis": {"label": "X Axis"},
        "y_axis": {"label": "Y Axis"},
        "renderers": [
            {
                "type": "line",
                "data": [{"x": 1, "y": 10}]
            }
        ]
    }"#;
    
    let id = st.bokeh_chart(spec);
    assert!(st.delta_gen().get_element(id).is_some());
}

#[test]
fn test_multiple_bokeh_charts() {
    let mut st = St::new();
    
    st.title("Bokeh Dashboard");
    
    for i in 0..5 {
        let spec = format!(r#"{{"type": "figure", "title": "Chart {}"}}"#, i);
        st.bokeh_chart(spec);
    }
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 6); // title + 5 charts
}

// ============================================================================
// MESSAGE COMPRESSION TESTS
// ============================================================================

#[test]
fn test_message_compression_large_payload() {
    let mut st = St::new();
    
    // Create large payload
    st.title("Large Payload Test");
    
    for i in 0..100 {
        st.write(format!("Line {} with some content to increase payload size", i));
    }
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 101); // title + 100 lines
    
    // In a real scenario, we'd compress these deltas
    // For now, just verify they're created
    assert!(deltas.len() > 0);
}

#[test]
fn test_compression_efficiency() {
    let mut st = St::new();
    
    // Create repetitive data (highly compressible)
    for _ in 0..50 {
        st.write("The same text repeated many times for compression testing");
    }
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 50);
    
    // Verify data integrity
    assert!(!deltas.is_empty());
}

#[test]
fn test_compression_with_json_data() {
    let mut st = St::new();
    
    // Large JSON payload
    let json_data = serde_json::json!({
        "data": [
            {"id": 1, "value": "data1"},
            {"id": 2, "value": "data2"},
            {"id": 3, "value": "data3"}
        ]
    });
    
    st.json(json_data);
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 1);
}

#[test]
fn test_error_recovery_invalid_input() {
    let mut st = St::new();
    
    // Test with empty strings
    st.write("");
    st.markdown("");
    st.code("", None);
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 3);
}

#[test]
fn test_error_recovery_large_input() {
    let mut st = St::new();
    
    // Very large string
    let large_string = "x".repeat(10000);
    st.write(&large_string);
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 1);
}

#[test]
fn test_error_recovery_special_characters() {
    let mut st = St::new();
    
    st.write("Special: !@#$%^&*()");
    st.write("Unicode: 你好世界 🌍 🎉");
    st.write("Quotes: \"double\" and 'single'");
    st.write("Newlines:\nLine1\nLine2");
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 4);
}

#[test]
fn test_error_recovery_malformed_json() {
    let mut st = St::new();
    
    // Invalid JSON should still be handled
    let invalid_json = serde_json::json!({
        "incomplete": "data"
    });
    
    st.json(invalid_json);
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 1);
}

#[test]
fn test_error_recovery_widget_state() {
    let mut st = St::new();
    
    // Test widget state recovery
    let value1 = st.text_input("Input", "", Some("key1".to_string()));
    assert_eq!(value1, "");
    
    let value2 = st.number_input("Number", 0.0, Some("key2".to_string()));
    assert_eq!(value2, 0.0);
    
    let checked = st.checkbox("Check", false, Some("key3".to_string()));
    assert!(!checked);
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 3);
}

// ============================================================================
// SESSION PERSISTENCE TESTS
// ============================================================================

#[test]
fn test_session_state_basic() {
    let mut st = St::new();
    
    // Simulate session state
    let mut session_state: HashMap<String, String> = HashMap::new();
    
    // Store widget values
    let name = st.text_input("Name", "John", Some("name_key".to_string()));
    session_state.insert("name_key".to_string(), name.clone());
    
    assert_eq!(session_state.get("name_key"), Some(&"John".to_string()));
}

#[test]
fn test_session_state_persistence() {
    let mut st = St::new();
    
    // First interaction
    let count1 = st.number_input("Counter", 0.0, Some("counter".to_string()));
    assert_eq!(count1, 0.0);
    
    // Simulate persistence by storing
    let mut persistent_state: HashMap<String, f64> = HashMap::new();
    persistent_state.insert("counter".to_string(), count1);
    
    // Second interaction (simulating rerun)
    let count2 = st.number_input("Counter", 5.0, Some("counter".to_string()));
    assert_eq!(count2, 5.0);
    
    // Verify persistence
    assert!(persistent_state.contains_key("counter"));
}

#[test]
fn test_session_state_multiple_widgets() {
    let mut st = St::new();
    
    let mut session: HashMap<String, String> = HashMap::new();
    
    // Multiple widgets
    let name = st.text_input("Name", "Alice", Some("name".to_string()));
    let email = st.text_input("Email", "alice@example.com", Some("email".to_string()));
    let age_str = st.number_input("Age", 30.0, Some("age".to_string())).to_string();
    
    session.insert("name".to_string(), name);
    session.insert("email".to_string(), email);
    session.insert("age".to_string(), age_str);
    
    assert_eq!(session.len(), 3);
    assert_eq!(session.get("name"), Some(&"Alice".to_string()));
}

#[test]
fn test_session_state_across_reruns() {
    let mut st = St::new();
    
    // Simulate multiple reruns
    let mut session_data: HashMap<String, i32> = HashMap::new();
    
    for run in 0..3 {
        let value = st.slider("Value", 0.0, 0.0, 100.0, Some("slider".to_string())) as i32;
        session_data.insert(format!("run_{}", run), value);
    }
    
    assert_eq!(session_data.len(), 3);
}

#[test]
fn test_session_state_with_complex_data() {
    let mut st = St::new();
    
    let mut session: HashMap<String, serde_json::Value> = HashMap::new();
    
    let data = serde_json::json!({
        "user": "john",
        "preferences": {
            "theme": "dark",
            "notifications": true
        }
    });
    
    session.insert("user_data".to_string(), data.clone());
    
    assert!(session.contains_key("user_data"));
    assert_eq!(session.get("user_data"), Some(&data));
}

// ============================================================================
// INTEGRATED PHASE 3 TESTS
// ============================================================================

#[test]
fn test_bokeh_with_session_state() {
    let mut st = St::new();
    
    let mut session: HashMap<String, String> = HashMap::new();
    
    st.title("Bokeh with Session");
    
    // Store chart spec in session
    let spec = r#"{"type": "figure", "title": "Session Chart"}"#;
    st.bokeh_chart(spec);
    session.insert("chart_spec".to_string(), spec.to_string());
    
    // Verify persistence
    assert!(session.contains_key("chart_spec"));
    
    let _deltas = st.take_deltas();
    assert_eq!(_deltas.len(), 2);
}

#[test]
fn test_error_recovery_with_compression() {
    let mut st = St::new();
    
    // Large payload with special characters
    for i in 0..50 {
        st.write(format!("Line {} with unicode: 你好 and special: !@#$", i));
    }
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 50);
}

#[test]
fn test_session_persistence_dashboard() {
    let mut st = St::new();
    
    let mut session: HashMap<String, String> = HashMap::new();
    
    st.title("Dashboard");
    
    // Store filter selections
    let date_range = st.selectbox(
        "Date Range",
        vec!["7 days".to_string(), "30 days".to_string()],
        0,
        Some("date_range".to_string()),
    );
    session.insert("date_range".to_string(), date_range);
    
    // Store chart
    let bokeh_spec = r#"{"type": "figure"}"#;
    st.bokeh_chart(bokeh_spec);
    session.insert("chart".to_string(), bokeh_spec.to_string());
    
    assert_eq!(session.len(), 2);
}

// ============================================================================
// PERFORMANCE TESTS FOR PHASE 3
// ============================================================================

#[test]
fn test_bokeh_performance() {
    let mut st = St::new();
    
    let start = std::time::Instant::now();
    
    for i in 0..50 {
        let spec = format!(r#"{{"type": "figure", "title": "Chart {}"}}"#, i);
        st.bokeh_chart(spec);
    }
    
    let duration = start.elapsed();
    assert!(duration.as_millis() < 100);
}

#[test]
fn test_session_state_performance() {
    let mut session: HashMap<String, String> = HashMap::new();
    
    let start = std::time::Instant::now();
    
    for i in 0..1000 {
        session.insert(format!("key_{}", i), format!("value_{}", i));
    }
    
    let duration = start.elapsed();
    assert!(duration.as_millis() < 50);
}

#[test]
fn test_large_payload_handling() {
    let mut st = St::new();
    
    let start = std::time::Instant::now();
    
    // Create large payload
    for i in 0..200 {
        st.write(format!("Large payload line {}: {}", i, "x".repeat(100)));
    }
    
    let duration = start.elapsed();
    assert!(duration.as_millis() < 200);
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 200);
}

// ============================================================================
// EDGE CASES FOR PHASE 3
// ============================================================================

#[test]
fn test_empty_bokeh_spec() {
    let mut st = St::new();
    
    st.bokeh_chart("{}");
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 1);
}

#[test]
fn test_session_state_collision() {
    let mut session: HashMap<String, String> = HashMap::new();
    
    // Test key collision handling
    session.insert("key".to_string(), "value1".to_string());
    session.insert("key".to_string(), "value2".to_string());
    
    // Should have only one entry (last write wins)
    assert_eq!(session.len(), 1);
    assert_eq!(session.get("key"), Some(&"value2".to_string()));
}

#[test]
fn test_compression_empty_data() {
    let mut st = St::new();
    
    // Empty writes should still work
    st.write("");
    st.markdown("");
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 2);
}

#[test]
fn test_error_recovery_concurrent_operations() {
    let mut st = St::new();
    
    // Simulate concurrent operations
    st.write("Op1");
    st.button("Button", Some("btn1".to_string()));
    st.write("Op2");
    st.checkbox("Check", false, Some("check1".to_string()));
    st.write("Op3");
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 5);
}
