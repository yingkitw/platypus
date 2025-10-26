//! Integration tests for Chatapp runtime
//! Tests Streamlit API compatibility and behavior

use platypus_runtime::prelude::*;

#[test]
fn test_basic_display_elements() {
    let mut st = St::new();
    
    // Test write
    let id1 = st.write("Hello, World!");
    assert!(st.delta_gen().get_element(id1).is_some());
    
    // Test markdown
    let id2 = st.markdown("# Heading");
    assert!(st.delta_gen().get_element(id2).is_some());
    
    // Test code
    let id3 = st.code("fn main() {}", Some("rust".to_string()));
    assert!(st.delta_gen().get_element(id3).is_some());
    
    // Verify deltas were generated
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 3);
}

#[test]
fn test_heading_hierarchy() {
    let mut st = St::new();
    
    let title_id = st.title("Main Title");
    let header_id = st.header("Sub Header");
    let subheader_id = st.subheader("Sub Sub Header");
    
    // All should be created
    assert!(st.delta_gen().get_element(title_id).is_some());
    assert!(st.delta_gen().get_element(header_id).is_some());
    assert!(st.delta_gen().get_element(subheader_id).is_some());
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 3);
}

#[test]
fn test_text_input_widget() {
    let mut st = St::new();
    
    // First call - default value
    let value1 = st.text_input("Name", "John", Some("name_key".to_string()));
    assert_eq!(value1, "John");
    
    // Simulate user input by setting widget state
    st.delta_gen().set_widget(
        "name_key".to_string(),
        platypus_core::widget::WidgetValue::String("Jane".to_string()),
    );
    
    // Second call - should get updated value
    let value2 = st.text_input("Name", "John", Some("name_key".to_string()));
    assert_eq!(value2, "Jane");
}

#[test]
fn test_number_input_widget() {
    let mut st = St::new();
    
    let value1 = st.number_input("Age", 25.0, Some("age_key".to_string()));
    assert_eq!(value1, 25.0);
    
    // Simulate user input
    st.delta_gen().set_widget(
        "age_key".to_string(),
        platypus_core::widget::WidgetValue::Number(30.0),
    );
    
    let value2 = st.number_input("Age", 25.0, Some("age_key".to_string()));
    assert_eq!(value2, 30.0);
}

#[test]
fn test_slider_widget() {
    let mut st = St::new();
    
    let value1 = st.slider("Score", 0.0, 100.0, 50.0, Some("score_key".to_string()));
    assert_eq!(value1, 50.0);
    
    // Simulate user input
    st.delta_gen().set_widget(
        "score_key".to_string(),
        platypus_core::widget::WidgetValue::Number(75.0),
    );
    
    let value2 = st.slider("Score", 0.0, 100.0, 50.0, Some("score_key".to_string()));
    assert_eq!(value2, 75.0);
}

#[test]
fn test_checkbox_widget() {
    let mut st = St::new();
    
    let checked1 = st.checkbox("Agree", false, Some("agree_key".to_string()));
    assert!(!checked1);
    
    // Simulate user input
    st.delta_gen().set_widget(
        "agree_key".to_string(),
        platypus_core::widget::WidgetValue::Bool(true),
    );
    
    let checked2 = st.checkbox("Agree", false, Some("agree_key".to_string()));
    assert!(checked2);
}

#[test]
fn test_selectbox_widget() {
    let mut st = St::new();
    
    let options = vec!["Option A".to_string(), "Option B".to_string(), "Option C".to_string()];
    let selected1 = st.selectbox("Choose", options.clone(), 0, Some("choice_key".to_string()));
    assert_eq!(selected1, "Option A");
    
    // Simulate user input
    st.delta_gen().set_widget(
        "choice_key".to_string(),
        platypus_core::widget::WidgetValue::String("Option B".to_string()),
    );
    
    let selected2 = st.selectbox("Choose", options, 0, Some("choice_key".to_string()));
    assert_eq!(selected2, "Option B");
}

#[test]
fn test_multiselect_widget() {
    let mut st = St::new();
    
    let options = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    let selected1 = st.multiselect("Choose", options.clone(), vec![], Some("multi_key".to_string()));
    assert!(selected1.is_empty());
    
    // Simulate user input
    st.delta_gen().set_widget(
        "multi_key".to_string(),
        platypus_core::widget::WidgetValue::StringArray(vec!["A".to_string(), "C".to_string()]),
    );
    
    let selected2 = st.multiselect("Choose", options, vec![], Some("multi_key".to_string()));
    assert_eq!(selected2.len(), 2);
    assert!(selected2.contains(&"A".to_string()));
    assert!(selected2.contains(&"C".to_string()));
}

#[test]
fn test_button_widget() {
    let mut st = St::new();
    
    // Button not clicked initially
    let clicked1 = st.button("Click me", Some("btn_key".to_string()));
    assert!(!clicked1);
    
    // Simulate button click
    st.delta_gen().set_widget(
        "btn_key".to_string(),
        platypus_core::widget::WidgetValue::Bool(true),
    );
    
    let clicked2 = st.button("Click me", Some("btn_key".to_string()));
    assert!(clicked2);
}

#[test]
fn test_feedback_elements() {
    let mut st = St::new();
    
    let success_id = st.success("Operation successful");
    let error_id = st.error("An error occurred");
    let warning_id = st.warning("Warning message");
    let info_id = st.info("Information");
    
    assert!(st.delta_gen().get_element(success_id).is_some());
    assert!(st.delta_gen().get_element(error_id).is_some());
    assert!(st.delta_gen().get_element(warning_id).is_some());
    assert!(st.delta_gen().get_element(info_id).is_some());
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 4);
}

#[test]
fn test_progress_bar() {
    let mut st = St::new();
    
    let id = st.progress(0.75);
    assert!(st.delta_gen().get_element(id).is_some());
}

#[test]
fn test_layout_columns() {
    let mut st = St::new();
    
    let cols = st.columns(3);
    assert_eq!(cols.len(), 3);
    
    // Each column should be a container
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 3);
}

#[test]
fn test_container() {
    let mut st = St::new();
    
    let container = st.container();
    let mut container_st = container.st();
    
    container_st.write("Inside container");
    container_st.write("Another element");
    
    // Should have created elements (container + 2 writes)
    let deltas = container_st.take_deltas();
    assert_eq!(deltas.len(), 3);
}

#[test]
fn test_json_display() {
    let mut st = St::new();
    
    let json_value = serde_json::json!({
        "name": "John",
        "age": 30,
        "active": true
    });
    
    let id = st.json(json_value);
    assert!(st.delta_gen().get_element(id).is_some());
}

#[test]
fn test_image_display() {
    let mut st = St::new();
    
    let id = st.image("https://example.com/image.png", Some("Caption".to_string()), Some(200));
    assert!(st.delta_gen().get_element(id).is_some());
}

#[test]
fn test_divider_and_empty() {
    let mut st = St::new();
    
    let divider_id = st.divider();
    let empty_id = st.empty();
    
    assert!(st.delta_gen().get_element(divider_id).is_some());
    assert!(st.delta_gen().get_element(empty_id).is_some());
}

#[test]
fn test_widget_state_persistence() {
    let mut st = St::new();
    
    // Create multiple widgets with keys
    st.text_input("Name", "Alice", Some("name".to_string()));
    st.number_input("Age", 25.0, Some("age".to_string()));
    st.checkbox("Active", true, Some("active".to_string()));
    
    // Simulate state changes
    st.delta_gen().set_widget("name".to_string(), platypus_core::widget::WidgetValue::String("Bob".to_string()));
    st.delta_gen().set_widget("age".to_string(), platypus_core::widget::WidgetValue::Number(30.0));
    st.delta_gen().set_widget("active".to_string(), platypus_core::widget::WidgetValue::Bool(false));
    
    // Verify state persists
    let name = st.text_input("Name", "Alice", Some("name".to_string()));
    let age = st.number_input("Age", 25.0, Some("age".to_string()));
    let active = st.checkbox("Active", true, Some("active".to_string()));
    
    assert_eq!(name, "Bob");
    assert_eq!(age, 30.0);
    assert!(!active);
}

#[test]
fn test_streamlit_api_compatibility() {
    // This test verifies that Webag provides a Streamlit-compatible API
    let mut st = St::new();
    
    // Streamlit-like workflow
    st.title("My App");
    st.write("Welcome to my app");
    
    let name = st.text_input("What's your name?", "World", None);
    let age = st.number_input("How old are you?", 0.0, None);
    
    st.write(format!("Hello, {}! You are {} years old.", name, age));
    
    if st.button("Click me", None) {
        st.success("Button was clicked!");
    }
    
    // Verify all elements were created
    let deltas = st.take_deltas();
    assert!(deltas.len() > 0);
}

#[test]
fn test_multiple_reruns_simulation() {
    // Simulate multiple script reruns like Streamlit does
    
    // First run
    let mut st = St::new();
    st.title("Counter App");
    let count1 = st.number_input("Count", 0.0, Some("count".to_string()));
    assert_eq!(count1, 0.0);
    
    // Simulate user incrementing the counter
    st.delta_gen().set_widget("count".to_string(), platypus_core::widget::WidgetValue::Number(1.0));
    
    // Second run (rerun after state change)
    let mut st2 = St::new();
    // Copy state from previous run
    st2.delta_gen().set_widget("count".to_string(), platypus_core::widget::WidgetValue::Number(1.0));
    
    st2.title("Counter App");
    let count2 = st2.number_input("Count", 0.0, Some("count".to_string()));
    assert_eq!(count2, 1.0);
    
    // Simulate incrementing again
    st2.delta_gen().set_widget("count".to_string(), platypus_core::widget::WidgetValue::Number(2.0));
    
    // Third run
    let mut st3 = St::new();
    st3.delta_gen().set_widget("count".to_string(), platypus_core::widget::WidgetValue::Number(2.0));
    
    st3.title("Counter App");
    let count3 = st3.number_input("Count", 0.0, Some("count".to_string()));
    assert_eq!(count3, 2.0);
}

#[test]
fn test_delta_generation() {
    let mut st = St::new();
    
    // Add elements
    st.write("Element 1");
    st.write("Element 2");
    st.write("Element 3");
    
    // Get deltas
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 3);
    
    // After taking deltas, they should be cleared
    let deltas2 = st.take_deltas();
    assert_eq!(deltas2.len(), 0);
    
    // Add more elements
    st.write("Element 4");
    let deltas3 = st.take_deltas();
    assert_eq!(deltas3.len(), 1);
}

#[test]
fn test_text_area_widget() {
    let mut st = St::new();
    
    let text1 = st.text_area("Comments", "Enter text here", Some("comments_key".to_string()));
    assert_eq!(text1, "Enter text here");
    
    // Simulate user input
    st.delta_gen().set_widget(
        "comments_key".to_string(),
        platypus_core::widget::WidgetValue::String("User's comment".to_string()),
    );
    
    let text2 = st.text_area("Comments", "Enter text here", Some("comments_key".to_string()));
    assert_eq!(text2, "User's comment");
}

#[test]
fn test_complex_app_workflow() {
    // Simulate a more complex Streamlit-like app
    let mut st = St::new();
    
    // Header
    st.title("Data Entry Form");
    st.markdown("Please fill out the form below");
    
    // Form fields
    let name = st.text_input("Full Name", "", Some("name".to_string()));
    let email = st.text_input("Email", "", Some("email".to_string()));
    let age = st.number_input("Age", 0.0, Some("age".to_string()));
    let country = st.selectbox("Country", vec!["USA".to_string(), "UK".to_string(), "Canada".to_string()], 0, Some("country".to_string()));
    let subscribe = st.checkbox("Subscribe to newsletter", false, Some("subscribe".to_string()));
    
    // Simulate form submission
    if st.button("Submit", Some("submit".to_string())) {
        st.success("Form submitted successfully!");
    }
    
    // Verify all elements were created
    let deltas = st.take_deltas();
    assert!(deltas.len() >= 8); // title, markdown, 5 inputs, button
}

#[test]
fn test_tabs() {
    let mut st = St::new();
    
    let tabs = st.tabs(vec!["Tab 1", "Tab 2", "Tab 3"]);
    assert_eq!(tabs.len(), 3);
    
    // Add content to each tab
    let mut tab1_st = tabs[0].st();
    tab1_st.write("Content for tab 1");
    
    let mut tab2_st = tabs[1].st();
    tab2_st.write("Content for tab 2");
    
    let mut tab3_st = tabs[2].st();
    tab3_st.write("Content for tab 3");
    
    // Verify deltas were generated
    let deltas = st.take_deltas();
    assert!(deltas.len() >= 1); // At least the tabs container
}

#[test]
fn test_expander() {
    let mut st = St::new();
    
    let expander = st.expander("Click to expand");
    let mut exp_st = expander.st();
    
    exp_st.write("Hidden content");
    exp_st.write("More hidden content");
    
    // Verify deltas were generated
    let deltas = st.take_deltas();
    assert!(deltas.len() >= 1); // At least the expander
}

#[test]
fn test_metric() {
    let mut st = St::new();
    
    let id1 = st.metric("Revenue", "$1,000,000", Some("+10%".to_string()));
    let id2 = st.metric("Users", "5,000", Some("-5%".to_string()));
    let id3 = st.metric("Growth", "25%", None);
    
    assert!(st.delta_gen().get_element(id1).is_some());
    assert!(st.delta_gen().get_element(id2).is_some());
    assert!(st.delta_gen().get_element(id3).is_some());
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 3);
}

#[test]
fn test_sidebar() {
    let mut st = St::new();
    
    let sidebar = st.sidebar();
    let mut sidebar_st = sidebar.st();
    
    sidebar_st.title("Navigation");
    sidebar_st.write("Menu item 1");
    sidebar_st.write("Menu item 2");
    
    // Verify deltas were generated
    let deltas = st.take_deltas();
    assert!(deltas.len() >= 1); // At least the sidebar
}

#[test]
fn test_advanced_layout_workflow() {
    // Test a complex workflow with tabs, expanders, and sidebar
    let mut st = St::new();
    
    st.title("Advanced Layout App");
    
    // Sidebar
    let sidebar = st.sidebar();
    let mut sidebar_st = sidebar.st();
    sidebar_st.write("Sidebar content");
    
    // Main content with tabs
    let tabs = st.tabs(vec!["Overview", "Details", "Settings"]);
    
    // Tab 1: Overview
    let mut tab1_st = tabs[0].st();
    tab1_st.write("Overview content");
    tab1_st.metric("Metric 1", "100", Some("+5%".to_string()));
    
    // Tab 2: Details with expander
    let mut tab2_st = tabs[1].st();
    tab2_st.write("Details content");
    let expander = tab2_st.expander("More details");
    let mut exp_st = expander.st();
    exp_st.write("Expanded content");
    
    // Tab 3: Settings
    let mut tab3_st = tabs[2].st();
    tab3_st.write("Settings content");
    
    // Verify all elements were created
    let deltas = st.take_deltas();
    assert!(deltas.len() > 0);
}

#[test]
fn test_date_input() {
    let mut st = St::new();
    
    let date1 = st.date_input("Select date", "2025-01-01", Some("date_key".to_string()));
    assert_eq!(date1, "2025-01-01");
    
    // Simulate user input
    st.delta_gen().set_widget(
        "date_key".to_string(),
        platypus_core::widget::WidgetValue::String("2025-12-31".to_string()),
    );
    
    let date2 = st.date_input("Select date", "2025-01-01", Some("date_key".to_string()));
    assert_eq!(date2, "2025-12-31");
}

#[test]
fn test_time_input() {
    let mut st = St::new();
    
    let time1 = st.time_input("Select time", "12:00", Some("time_key".to_string()));
    assert_eq!(time1, "12:00");
    
    // Simulate user input
    st.delta_gen().set_widget(
        "time_key".to_string(),
        platypus_core::widget::WidgetValue::String("14:30".to_string()),
    );
    
    let time2 = st.time_input("Select time", "12:00", Some("time_key".to_string()));
    assert_eq!(time2, "14:30");
}

#[test]
fn test_color_picker() {
    let mut st = St::new();
    
    let color1 = st.color_picker("Pick color", "#FF0000", Some("color_key".to_string()));
    assert_eq!(color1, "#FF0000");
    
    // Simulate user input
    st.delta_gen().set_widget(
        "color_key".to_string(),
        platypus_core::widget::WidgetValue::String("#00FF00".to_string()),
    );
    
    let color2 = st.color_picker("Pick color", "#FF0000", Some("color_key".to_string()));
    assert_eq!(color2, "#00FF00");
}

#[test]
fn test_form_with_date_time_color() {
    let mut st = St::new();
    
    st.title("Event Form");
    
    let event_name = st.text_input("Event name", "", Some("name".to_string()));
    let event_date = st.date_input("Event date", "2025-01-01", Some("date".to_string()));
    let event_time = st.time_input("Event time", "10:00", Some("time".to_string()));
    let event_color = st.color_picker("Event color", "#0000FF", Some("color".to_string()));
    
    if st.button("Create Event", Some("create".to_string())) {
        st.success("Event created!");
    }
    
    // Verify all elements were created
    let deltas = st.take_deltas();
    assert!(deltas.len() >= 6); // title, 4 inputs, button
}

#[test]
fn test_file_uploader() {
    let mut st = St::new();
    
    // Initially no file uploaded
    let file1 = st.file_uploader("Upload file", Some("file_key".to_string()));
    assert!(file1.is_none());
    
    // Simulate file upload
    st.delta_gen().set_widget(
        "file_key".to_string(),
        platypus_core::widget::WidgetValue::String("document.pdf".to_string()),
    );
    
    let file2 = st.file_uploader("Upload file", Some("file_key".to_string()));
    assert_eq!(file2, Some("document.pdf".to_string()));
}

#[test]
fn test_file_upload_workflow() {
    let mut st = St::new();
    
    st.title("File Upload");
    st.write("Upload your file below");
    
    let uploaded_file = st.file_uploader("Choose file", Some("upload".to_string()));
    
    if let Some(filename) = uploaded_file {
        st.success(format!("File uploaded: {}", filename));
    } else {
        st.info("No file uploaded yet");
    }
    
    // Verify elements were created
    let deltas = st.take_deltas();
    assert!(deltas.len() >= 4); // title, write, uploader, info
}

#[test]
fn test_radio_button() {
    let mut st = St::new();
    
    let options = vec!["Option A", "Option B", "Option C"];
    let selected1 = st.radio("Choose", options.clone(), 0, Some("radio_key".to_string()));
    assert_eq!(selected1, "Option A");
    
    // Simulate user selection
    st.delta_gen().set_widget(
        "radio_key".to_string(),
        platypus_core::widget::WidgetValue::String("Option B".to_string()),
    );
    
    let selected2 = st.radio("Choose", options, 0, Some("radio_key".to_string()));
    assert_eq!(selected2, "Option B");
}

#[test]
fn test_form_validation() {
    let mut st = St::new();
    
    st.title("Registration Form");
    
    let email = st.text_input("Email", "", Some("email".to_string()));
    let password = st.text_input("Password", "", Some("password".to_string()));
    
    // Simple validation
    let mut valid = true;
    if !email.contains("@") {
        st.error("Invalid email format");
        valid = false;
    }
    
    if password.len() < 8 {
        st.error("Password must be at least 8 characters");
        valid = false;
    }
    
    if valid && st.button("Register", Some("register".to_string())) {
        st.success("Registration successful!");
    }
    
    // Verify elements were created
    let deltas = st.take_deltas();
    assert!(deltas.len() >= 4); // title, 2 inputs, button/error
}

#[test]
fn test_complete_form_workflow() {
    let mut st = St::new();
    
    st.title("Complete Survey Form");
    st.markdown("Please fill out all fields");
    
    // Personal info
    let name = st.text_input("Full Name", "", Some("name".to_string()));
    let email = st.text_input("Email", "", Some("email".to_string()));
    let age = st.number_input("Age", 0.0, Some("age".to_string()));
    
    // Preferences
    let gender = st.radio("Gender", vec!["Male", "Female", "Other"], 0, Some("gender".to_string()));
    let country = st.selectbox("Country", vec!["USA".to_string(), "UK".to_string(), "Canada".to_string()], 0, Some("country".to_string()));
    
    // Additional info
    let subscribe = st.checkbox("Subscribe to newsletter", false, Some("subscribe".to_string()));
    let comments = st.text_area("Comments", "", Some("comments".to_string()));
    
    // Date and time
    let event_date = st.date_input("Event date", "2025-01-01", Some("date".to_string()));
    let event_time = st.time_input("Event time", "10:00", Some("time".to_string()));
    
    // File upload
    let attachment = st.file_uploader("Upload attachment", Some("file".to_string()));
    
    // Submit
    if st.button("Submit Form", Some("submit".to_string())) {
        st.success("Form submitted successfully!");
    }
    
    // Verify all elements were created
    let deltas = st.take_deltas();
    assert!(deltas.len() >= 12); // title, markdown, and 10+ form fields
}

#[test]
fn test_table_display() {
    let mut st = St::new();
    
    st.title("Sales Data");
    
    let headers = vec!["Product", "Q1", "Q2", "Q3", "Q4"];
    let rows = vec![
        vec!["Product A", "100", "120", "150", "180"],
        vec!["Product B", "200", "210", "220", "240"],
        vec!["Product C", "150", "160", "170", "190"],
    ];
    
    st.table(headers, rows);
    
    let deltas = st.take_deltas();
    assert!(deltas.len() >= 2); // title, table
}

#[test]
fn test_dataframe_display() {
    let mut st = St::new();
    
    st.title("DataFrame Example");
    
    let json_data = r#"[
        {"name": "Alice", "age": 30, "city": "New York"},
        {"name": "Bob", "age": 25, "city": "Los Angeles"},
        {"name": "Charlie", "age": 35, "city": "Chicago"}
    ]"#;
    
    st.dataframe(json_data);
    
    let deltas = st.take_deltas();
    assert!(deltas.len() >= 2); // title, dataframe
}

#[test]
fn test_camera_input() {
    let mut st = St::new();
    
    // Initially no image captured
    let image1 = st.camera_input("Capture photo", Some("camera_key".to_string()));
    assert!(image1.is_none());
    
    // Simulate image capture
    st.delta_gen().set_widget(
        "camera_key".to_string(),
        platypus_core::widget::WidgetValue::String("data:image/jpeg;base64,/9j/4AAQSkZJRg...".to_string()),
    );
    
    let image2 = st.camera_input("Capture photo", Some("camera_key".to_string()));
    assert!(image2.is_some());
}

#[test]
fn test_data_dashboard() {
    let mut st = St::new();
    
    st.title("Analytics Dashboard");
    st.markdown("Real-time data visualization");
    
    // Metrics
    st.metric("Total Users", "1,234", Some("+12%".to_string()));
    st.metric("Revenue", "$45,678", Some("+8%".to_string()));
    st.metric("Conversion", "3.2%", Some("-0.5%".to_string()));
    
    // Table with data
    let headers = vec!["Date", "Users", "Revenue", "Conversion"];
    let rows = vec![
        vec!["2025-01-01", "1000", "40000", "3.5%"],
        vec!["2025-01-02", "1100", "42000", "3.3%"],
        vec!["2025-01-03", "1234", "45678", "3.2%"],
    ];
    st.table(headers, rows);
    
    // DataFrame
    let df_json = r#"[
        {"metric": "Page Views", "value": 5000},
        {"metric": "Clicks", "value": 1500},
        {"metric": "Conversions", "value": 150}
    ]"#;
    st.dataframe(df_json);
    
    let deltas = st.take_deltas();
    assert!(deltas.len() >= 6); // title, markdown, 3 metrics, table, dataframe
}

#[test]
fn test_media_capture_workflow() {
    let mut st = St::new();
    
    st.title("Media Capture");
    
    // File upload
    let document = st.file_uploader("Upload document", Some("doc".to_string()));
    
    // Camera capture
    let photo = st.camera_input("Take photo", Some("photo".to_string()));
    
    // Display results
    if document.is_some() {
        st.success("Document uploaded");
    }
    
    if photo.is_some() {
        st.success("Photo captured");
    }
    
    let deltas = st.take_deltas();
    assert!(deltas.len() >= 3); // title, uploader, camera
}
