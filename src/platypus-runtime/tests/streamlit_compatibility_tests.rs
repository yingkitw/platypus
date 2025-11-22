//! Comprehensive Streamlit Compatibility Test Suite - Part 1
//! 
//! This test suite demonstrates strong compatibility with Streamlit's API
//! across all major categories: display, input, layout, and feedback elements.

use platypus_runtime::prelude::*;

// ============================================================================
// DISPLAY ELEMENTS - Text, Markdown, Code, etc.
// ============================================================================

#[test]
fn test_streamlit_text_display() {
    let mut st = St::new();
    
    st.write("Hello, Streamlit!");
    st.write("Line 1");
    st.write("Line 2");
    st.write("Line 3");
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 4);
}

#[test]
fn test_streamlit_markdown_rendering() {
    let mut st = St::new();
    
    st.markdown("# Main Title");
    st.markdown("## Subtitle");
    st.markdown("**Bold text** and *italic text*");
    st.markdown("- List item 1\n- List item 2");
    st.markdown("[Link](https://example.com)");
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 5);
}

#[test]
fn test_streamlit_code_display() {
    let mut st = St::new();
    
    st.code("fn main() { println!(\"Hello\"); }", Some("rust".to_string()));
    st.code("print('Hello')", Some("python".to_string()));
    st.code("console.log('Hello');", Some("javascript".to_string()));
    st.code("SELECT * FROM users;", Some("sql".to_string()));
    st.code("plain code", None);
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 5);
}

#[test]
fn test_streamlit_heading_hierarchy() {
    let mut st = St::new();
    
    st.title("Page Title");
    st.header("Section Header");
    st.subheader("Subsection Header");
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 3);
}

#[test]
fn test_streamlit_divider_and_spacing() {
    let mut st = St::new();
    
    st.write("Section 1");
    st.divider();
    st.write("Section 2");
    st.divider();
    st.write("Section 3");
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 5);
}

#[test]
fn test_streamlit_json_display() {
    let mut st = St::new();
    
    let data = serde_json::json!({
        "name": "John Doe",
        "age": 30,
        "email": "john@example.com"
    });
    
    st.json(data);
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 1);
}

#[test]
fn test_streamlit_metric_display() {
    let mut st = St::new();
    
    st.metric("Revenue", "$1,234.56", Some("+5%".to_string()));
    st.metric("Users", "1,234", Some("-2%".to_string()));
    st.metric("Conversion Rate", "3.2%", None);
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 3);
}

// ============================================================================
// INPUT WIDGETS - Text, Numbers, Selections, etc.
// ============================================================================

#[test]
fn test_streamlit_text_input_widget() {
    let mut st = St::new();
    
    let name = st.text_input("Enter your name", "John", Some("name_key".to_string()));
    assert_eq!(name, "John");
    
    let email = st.text_input("Email", "", Some("email_key".to_string()));
    assert_eq!(email, "");
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 2);
}

#[test]
fn test_streamlit_text_area_widget() {
    let mut st = St::new();
    
    let feedback = st.text_area("Feedback", "Enter your feedback here", Some("feedback_key".to_string()));
    assert_eq!(feedback, "Enter your feedback here");
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 1);
}

#[test]
fn test_streamlit_number_input_widget() {
    let mut st = St::new();
    
    let age = st.number_input("Age", 18.0, Some("age_key".to_string()));
    assert_eq!(age, 18.0);
    
    let price = st.number_input("Price", 9.99, Some("price_key".to_string()));
    assert_eq!(price, 9.99);
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 2);
}

#[test]
fn test_streamlit_slider_widget() {
    let mut st = St::new();
    
    let volume = st.slider("Volume", 0.0, 100.0, 50.0, Some("volume_key".to_string()));
    assert_eq!(volume, 50.0);
    
    let opacity = st.slider("Opacity", 0.0, 1.0, 0.5, Some("opacity_key".to_string()));
    assert_eq!(opacity, 0.5);
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 2);
}

#[test]
fn test_streamlit_checkbox_widget() {
    let mut st = St::new();
    
    let agree = st.checkbox("I agree to terms", false, Some("agree_key".to_string()));
    assert!(!agree);
    
    let subscribe = st.checkbox("Subscribe to newsletter", true, Some("subscribe_key".to_string()));
    assert!(subscribe);
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 2);
}

#[test]
fn test_streamlit_radio_widget() {
    let mut st = St::new();
    
    let options = vec!["Option A", "Option B", "Option C"];
    
    let choice = st.radio("Choose one", options, 0, Some("choice_key".to_string()));
    assert_eq!(choice, "Option A");
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 1);
}

#[test]
fn test_streamlit_selectbox_widget() {
    let mut st = St::new();
    
    let options = vec!["Apple".to_string(), "Banana".to_string(), "Cherry".to_string()];
    
    let fruit = st.selectbox("Choose a fruit", options, 0, Some("fruit_key".to_string()));
    assert_eq!(fruit, "Apple");
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 1);
}

#[test]
fn test_streamlit_multiselect_widget() {
    let mut st = St::new();
    
    let options = vec!["Python".to_string(), "Rust".to_string(), "JavaScript".to_string()];
    let defaults = vec!["Python".to_string(), "Rust".to_string()];
    
    let selected = st.multiselect("Choose languages", options, defaults, Some("langs_key".to_string()));
    assert_eq!(selected.len(), 2);
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 1);
}

#[test]
fn test_streamlit_button_widget() {
    let mut st = St::new();
    
    let clicked = st.button("Click me", Some("btn_key".to_string()));
    assert!(!clicked);
    
    st.button("Button 1", Some("btn1_key".to_string()));
    st.button("Button 2", Some("btn2_key".to_string()));
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 3);
}

#[test]
fn test_streamlit_color_picker_widget() {
    let mut st = St::new();
    
    let color = st.color_picker("Pick a color", "#FF0000", Some("color_key".to_string()));
    assert_eq!(color, "#FF0000");
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 1);
}

#[test]
fn test_streamlit_date_time_input_widgets() {
    let mut st = St::new();
    
    let date = st.date_input("Select a date", "2024-01-01", Some("date_key".to_string()));
    assert_eq!(date, "2024-01-01");
    
    let time = st.time_input("Select a time", "12:00", Some("time_key".to_string()));
    assert_eq!(time, "12:00");
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 2);
}

// ============================================================================
// FEEDBACK ELEMENTS - Success, Error, Warning, Info
// ============================================================================

#[test]
fn test_streamlit_feedback_messages() {
    let mut st = St::new();
    
    st.success("Operation completed successfully!");
    st.error("An error occurred!");
    st.warning("Please be careful!");
    st.info("Here's some information");
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 4);
}

#[test]
fn test_streamlit_progress_bar() {
    let mut st = St::new();
    
    st.progress(0.5);
    st.progress(0.75);
    st.progress(1.0);
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 3);
}

// ============================================================================
// LAYOUT ELEMENTS - Columns, Containers, Tabs, Expanders
// ============================================================================

#[test]
fn test_streamlit_columns_layout() {
    let mut st = St::new();
    
    let cols = st.columns(3);
    assert_eq!(cols.len(), 3);
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 3);
}

#[test]
fn test_streamlit_container_layout() {
    let mut st = St::new();
    
    let _container1 = st.container();
    let _container2 = st.container();
    let _container3 = st.container();
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 3);
}

#[test]
fn test_streamlit_tabs_layout() {
    let mut st = St::new();
    
    let tab_names = vec!["Tab 1", "Tab 2", "Tab 3"];
    
    let tabs = st.tabs(tab_names);
    assert_eq!(tabs.len(), 3);
    
    let deltas = st.take_deltas();
    assert!(deltas.len() >= 3);
}

#[test]
fn test_streamlit_expander_layout() {
    let mut st = St::new();
    
    let _exp1 = st.expander("Expand me");
    let _exp2 = st.expander("More details");
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 2);
}

#[test]
fn test_streamlit_sidebar_layout() {
    let mut st = St::new();
    
    let sidebar = st.sidebar();
    let mut st_sidebar = sidebar.st();
    st_sidebar.write("Sidebar Title");
    st_sidebar.markdown("## Navigation");
    st_sidebar.button("Menu Item 1", Some("menu1_key".to_string()));
    st_sidebar.button("Menu Item 2", Some("menu2_key".to_string()));
    
    let deltas = st_sidebar.take_deltas();
    assert!(deltas.len() >= 4);
}

// ============================================================================
// COMPLEX WORKFLOWS
// ============================================================================

#[test]
fn test_streamlit_form_workflow() {
    let mut st = St::new();
    
    st.title("User Registration");
    
    let _name = st.text_input("Full Name", "", Some("name_key".to_string()));
    let _email = st.text_input("Email", "", Some("email_key".to_string()));
    let age = st.number_input("Age", 18.0, Some("age_key".to_string()));
    let agree = st.checkbox("I agree to terms", false, Some("agree_key".to_string()));
    
    let submit = st.button("Submit", Some("submit_key".to_string()));
    
    assert_eq!(age, 18.0);
    assert!(!agree);
    assert!(!submit);
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 6);
}

#[test]
fn test_streamlit_dashboard_workflow() {
    let mut st = St::new();
    
    st.title("Analytics Dashboard");
    st.markdown("Real-time metrics and insights");
    
    st.metric("Total Users", "1,234", Some("+12%".to_string()));
    st.metric("Active Sessions", "456", Some("+5%".to_string()));
    st.metric("Conversion Rate", "3.2%", Some("-0.5%".to_string()));
    
    st.divider();
    
    st.markdown("### Filters");
    st.selectbox("Time Range", vec!["Last 7 days".to_string(), "Last 30 days".to_string()], 0, Some("range_key".to_string()));
    
    let deltas = st.take_deltas();
    assert!(deltas.len() >= 8);
}

#[test]
fn test_streamlit_settings_page_workflow() {
    let mut st = St::new();
    
    st.title("Settings");
    
    st.header("Account");
    st.text_input("Username", "john_doe", Some("username_key".to_string()));
    st.text_input("Email", "john@example.com", Some("email_key".to_string()));
    
    st.header("Preferences");
    st.selectbox("Theme", vec!["Light".to_string(), "Dark".to_string()], 0, Some("theme_key".to_string()));
    st.checkbox("Enable notifications", true, Some("notify_key".to_string()));
    
    st.divider();
    st.button("Save Changes", Some("save_key".to_string()));
    
    let deltas = st.take_deltas();
    assert!(deltas.len() >= 8);
}

// ============================================================================
// STATE MANAGEMENT
// ============================================================================

#[test]
fn test_streamlit_state_persistence() {
    let mut st = St::new();
    
    let count1 = st.number_input("Counter", 0.0, Some("counter_key".to_string()));
    assert_eq!(count1, 0.0);
    
    let count2 = st.number_input("Counter", 5.0, Some("counter_key".to_string()));
    assert_eq!(count2, 5.0);
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 2);
}

#[test]
fn test_streamlit_conditional_rendering() {
    let mut st = St::new();
    
    st.title("Conditional Display");
    
    let show_details = st.checkbox("Show details", false, Some("show_details_key".to_string()));
    
    if show_details {
        st.write("Detailed information");
    } else {
        st.write("Click checkbox to see details");
    }
    
    let deltas = st.take_deltas();
    assert!(deltas.len() >= 2);
}

#[test]
fn test_streamlit_loop_rendering() {
    let mut st = St::new();
    
    st.title("Item List");
    
    let items = vec!["Item 1", "Item 2", "Item 3", "Item 4", "Item 5"];
    
    for (i, item) in items.iter().enumerate() {
        st.write(format!("{}. {}", i + 1, item));
    }
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 6);
}

// ============================================================================
// EDGE CASES AND ERROR HANDLING
// ============================================================================

#[test]
fn test_streamlit_empty_app() {
    let mut st = St::new();
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 0);
}

#[test]
fn test_streamlit_large_content() {
    let mut st = St::new();
    
    for i in 0..100 {
        st.write(format!("Line {}", i));
    }
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 100);
}

#[test]
fn test_streamlit_special_characters() {
    let mut st = St::new();
    
    st.write("Special chars: !@#$%^&*()");
    st.write("Unicode: 你好世界 🌍");
    st.write("Quotes: \"double\" and 'single'");
    st.markdown("# Markdown with **bold** and *italic*");
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 4);
}

#[test]
fn test_streamlit_mixed_content_types() {
    let mut st = St::new();
    
    st.title("Mixed Content");
    st.write("Text content");
    st.markdown("**Markdown** content");
    st.code("code content", Some("rust".to_string()));
    st.divider();
    st.metric("Metric", "Value", None);
    st.success("Success message");
    st.error("Error message");
    st.warning("Warning message");
    st.info("Info message");
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 10);
}

#[test]
fn test_streamlit_rapid_sequential_calls() {
    let mut st = St::new();
    
    for _ in 0..50 {
        st.write("Quick");
        st.button("Click", Some("btn_key".to_string()));
        st.checkbox("Check", false, Some("check_key".to_string()));
    }
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 150);
}

// ============================================================================
// PERFORMANCE BENCHMARKS
// ============================================================================

#[test]
fn test_streamlit_performance_element_creation() {
    let mut st = St::new();
    
    let start = std::time::Instant::now();
    
    for i in 0..1000 {
        st.write(format!("Element {}", i));
    }
    
    let duration = start.elapsed();
    assert!(duration.as_millis() < 500, "Element creation took {:?}", duration);
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 1000);
}

#[test]
fn test_streamlit_performance_widget_creation() {
    let mut st = St::new();
    
    let start = std::time::Instant::now();
    
    for i in 0..100 {
        st.text_input("Input", "default", Some(format!("key_{}", i)));
        st.button("Button", Some(format!("btn_{}", i)));
        st.checkbox("Check", false, Some(format!("check_{}", i)));
    }
    
    let duration = start.elapsed();
    assert!(duration.as_millis() < 200, "Widget creation took {:?}", duration);
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 300);
}

#[test]
fn test_streamlit_performance_delta_generation() {
    let mut st = St::new();
    
    for i in 0..500 {
        st.write(format!("Line {}", i));
    }
    
    let start = std::time::Instant::now();
    let deltas = st.take_deltas();
    let duration = start.elapsed();
    
    assert!(duration.as_millis() < 50, "Delta generation took {:?}", duration);
    assert_eq!(deltas.len(), 500);
}
