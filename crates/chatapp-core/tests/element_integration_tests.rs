//! Comprehensive integration tests for element system.

use chatapp_core::element::ElementId;
use chatapp_core::elements::*;
use chatapp_core::traits::*;
use serde_json::json;

#[test]
fn test_element_factory_creation() {
    let id = ElementId::new(1);
    
    let text = ElementFactory::text(id, "Hello");
    assert_eq!(text.content(), "Hello");
    
    let button = ElementFactory::button(id, "Click");
    assert_eq!(button.label(), "Click");
    
    let heading = ElementFactory::heading(id, "Title", 1).unwrap();
    assert_eq!(heading.level(), 1);
}

#[test]
fn test_element_builder_fluent_api() {
    let id = ElementId::new(1);
    
    let text = ElementBuilder::new(id).text("Hello");
    assert_eq!(text.content(), "Hello");
    
    let button = ElementBuilder::new(id).button("Click");
    assert_eq!(button.label(), "Click");
}

#[test]
fn test_slider_widget_full_workflow() {
    let mut slider = SliderElement::new(ElementId::new(1), "Volume", 0.0, 100.0).unwrap();
    
    // Initial state
    assert_eq!(slider.value(), 0.0);
    assert!(slider.validate().is_ok());
    
    // Set value
    slider.set_value(serde_json::json!(50.0)).unwrap();
    assert_eq!(slider.value(), 50.0);
    
    // Handle event
    let event = InteractionEvent {
        event_type: "change".to_string(),
        target_id: ElementId::new(1),
        data: Some(serde_json::json!(75.0)),
        timestamp: 0,
    };
    slider.handle_event(&event).unwrap();
    assert_eq!(slider.value(), 75.0);
    
    // Serialize
    let json = slider.to_json().unwrap();
    assert_eq!(json["type"], "slider");
    assert_eq!(json["value"], 75.0);
}

#[test]
fn test_checkbox_widget_full_workflow() {
    let mut checkbox = CheckboxElement::new(ElementId::new(1), "Accept");
    checkbox.set_value(json!(true)).unwrap();
    assert!(checkbox.is_checked());
    assert!(checkbox.validate().is_ok());
    
    // Set checked
    checkbox.set_checked(true);
    assert!(checkbox.is_checked());
    
    // Handle event
    let event = InteractionEvent {
        event_type: "change".to_string(),
        target_id: ElementId::new(1),
        data: Some(serde_json::json!(false)),
        timestamp: 0,
    };
    checkbox.handle_event(&event).unwrap();
    assert!(!checkbox.is_checked());
    
    // Get value
    let value = checkbox.get_value().unwrap();
    assert_eq!(value, serde_json::json!(false));
}

#[test]
fn test_selectbox_widget_full_workflow() {
    let options = vec![
        "Option 1".to_string(),
        "Option 2".to_string(),
        "Option 3".to_string(),
    ];
    
    let mut selectbox = SelectboxElement::new(ElementId::new(1), "Choose", options).unwrap();
    
    // Initial state
    assert_eq!(selectbox.selected_value(), "Option 1");
    assert_eq!(selectbox.selected_index(), 0);
    
    // Set index
    selectbox.set_selected_index(2).unwrap();
    assert_eq!(selectbox.selected_value(), "Option 3");
    
    // Set value by string
    selectbox.set_value(serde_json::json!("Option 2")).unwrap();
    assert_eq!(selectbox.selected_value(), "Option 2");
    
    // Validate
    assert!(selectbox.validate().is_ok());
}

#[test]
fn test_container_hierarchy() {
    let mut container = ContainerElement::new(ElementId::new(1));
    
    // Add children
    container.add_child(ElementId::new(2)).unwrap();
    container.add_child(ElementId::new(3)).unwrap();
    container.add_child(ElementId::new(4)).unwrap();
    
    assert_eq!(container.children().len(), 3);
    assert!(container.has_children());
    
    // Remove child
    container.remove_child(ElementId::new(3)).unwrap();
    assert_eq!(container.children().len(), 2);
    
    // Clear children
    container.clear_children();
    assert!(!container.has_children());
}

#[test]
fn test_responsive_container_breakpoints() {
    let mut container = ResponsiveContainerElement::new(ElementId::new(1));
    
    // Check breakpoints
    let breakpoints = container.breakpoints();
    assert_eq!(breakpoints.len(), 3);
    assert_eq!(breakpoints[0].name, "mobile");
    assert_eq!(breakpoints[1].name, "tablet");
    assert_eq!(breakpoints[2].name, "desktop");
    
    // Get layout for breakpoint
    let mobile_layout = container.layout_for_breakpoint("mobile").unwrap();
    assert_eq!(mobile_layout.display, "flex");
    assert_eq!(mobile_layout.flex_direction, Some("column".to_string()));
    
    let desktop_layout = container.layout_for_breakpoint("desktop").unwrap();
    assert_eq!(desktop_layout.display, "grid");
    
    // Set current breakpoint
    container.set_current_breakpoint("tablet".to_string());
    assert_eq!(container.current_breakpoint(), "tablet");
}

#[test]
fn test_themed_button_theme_switching() {
    let mut button = ThemedButtonElement::new(ElementId::new(1), "Click me");
    
    // Initial theme
    assert_eq!(button.current_theme(), "light");
    assert_eq!(button.available_themes().len(), 2);
    
    // Switch to dark theme
    button.set_theme("dark").unwrap();
    assert_eq!(button.current_theme(), "dark");
    
    // Check colors updated
    let colors = button.theme_colors();
    assert_eq!(colors.get("primary"), Some(&"#3399FF".to_string()));
    
    // Invalid theme
    assert!(button.set_theme("invalid").is_err());
}

#[test]
fn test_themed_text_theme_switching() {
    let mut text = ThemedTextElement::new(ElementId::new(1), "Hello");
    
    // Initial theme
    assert_eq!(text.current_theme(), "light");
    assert_eq!(text.theme_css_class(), "theme-light");
    
    // Switch theme
    text.set_theme("dark").unwrap();
    assert_eq!(text.current_theme(), "dark");
    assert_eq!(text.theme_css_class(), "theme-dark");
    
    // High contrast theme
    text.set_theme("high-contrast").unwrap();
    assert_eq!(text.current_theme(), "high-contrast");
}

#[test]
fn test_validation_chain() {
    // Valid elements
    let text = TextElement::new(ElementId::new(1), "Hello");
    assert!(text.validate().is_ok());
    
    let button = ButtonElement::new(ElementId::new(2), "Click");
    assert!(button.validate().is_ok());
    
    // Invalid elements
    let empty_text = TextElement::new(ElementId::new(3), "");
    assert!(empty_text.validate().is_err());
    
    let empty_button = ButtonElement::new(ElementId::new(4), "");
    assert!(empty_button.validate().is_err());
}

#[test]
fn test_serialization_chain() {
    let text = TextElement::new(ElementId::new(1), "Hello");
    let json = text.to_json().unwrap();
    assert_eq!(json["type"], "text");
    assert_eq!(json["content"], "Hello");
    
    let button = ButtonElement::new(ElementId::new(2), "Click");
    let json = button.to_json().unwrap();
    assert_eq!(json["type"], "button");
    assert_eq!(json["label"], "Click");
    
    let slider = SliderElement::new(ElementId::new(3), "Volume", 0.0, 100.0).unwrap();
    let json = slider.to_json().unwrap();
    assert_eq!(json["type"], "slider");
    assert_eq!(json["min"], 0.0);
    assert_eq!(json["max"], 100.0);
}

#[test]
fn test_interactive_event_handling() {
    let mut button = ButtonElement::new(ElementId::new(1), "Click");
    
    // Click event
    let event = InteractionEvent {
        event_type: "click".to_string(),
        target_id: ElementId::new(1),
        data: None,
        timestamp: 0,
    };
    
    button.handle_event(&event).unwrap();
    assert_eq!(button.click_count(), 1);
    
    button.handle_event(&event).unwrap();
    assert_eq!(button.click_count(), 2);
}

#[test]
fn test_text_input_with_constraints() {
    let mut input = TextInputElement::new(ElementId::new(1), "Username");
    input.set_max_length(20);
    input.set_required(true);
    
    // Empty value should fail validation
    assert!(input.validate().is_err());
    
    // Set valid value
    input.set_value(serde_json::json!("john_doe")).unwrap();
    assert!(input.validate().is_ok());
    
    // Exceed max length
    let long_value = "a".repeat(25);
    assert!(input.set_value(serde_json::json!(long_value)).is_err());
}

#[test]
fn test_feedback_elements() {
    let success = FeedbackElement::new(ElementId::new(1), FeedbackType::Success, "Operation successful");
    assert_eq!(success.feedback_type(), FeedbackType::Success);
    assert_eq!(success.message(), "Operation successful");
    
    let error = FeedbackElement::new(ElementId::new(2), FeedbackType::Error, "An error occurred");
    assert_eq!(error.feedback_type(), FeedbackType::Error);
    
    let warning = FeedbackElement::new(ElementId::new(3), FeedbackType::Warning, "Warning message");
    assert_eq!(warning.feedback_type(), FeedbackType::Warning);
    
    let info = FeedbackElement::new(ElementId::new(4), FeedbackType::Info, "Info message");
    assert_eq!(info.feedback_type(), FeedbackType::Info);
}

#[test]
fn test_metric_element() {
    let mut metric = MetricElement::new(ElementId::new(1), "Revenue", "$1,234.56");
    assert_eq!(metric.label(), "Revenue");
    assert_eq!(metric.value(), "$1,234.56");
    assert!(metric.delta().is_none());
    
    metric.set_delta("+5%");
    assert_eq!(metric.delta(), Some("+5%"));
    
    let json = metric.to_json().unwrap();
    assert_eq!(json["type"], "metric");
    assert_eq!(json["label"], "Revenue");
}

#[test]
fn test_image_element_with_dimensions() {
    let mut image = ImageElement::new(ElementId::new(1), "image.png", "A sample image");
    image.set_width(200);
    image.set_height(150);
    
    assert_eq!(image.src(), "image.png");
    assert_eq!(image.alt(), "A sample image");
    
    let json = image.to_json().unwrap();
    assert_eq!(json["src"], "image.png");
    assert_eq!(json["width"], 200);
    assert_eq!(json["height"], 150);
}

#[test]
fn test_heading_levels() {
    for level in 1..=6 {
        let heading = HeadingElement::new(ElementId::new(level as u64), "Title", level).unwrap();
        assert_eq!(heading.level(), level);
        assert!(heading.validate().is_ok());
    }
    
    // Invalid levels
    assert!(HeadingElement::new(ElementId::new(7), "Title", 0).is_err());
    assert!(HeadingElement::new(ElementId::new(8), "Title", 7).is_err());
}

#[test]
fn test_complex_form_workflow() {
    // Create a form with multiple elements
    let mut form_container = ContainerElement::new(ElementId::new(1));
    
    // Add form elements
    form_container.add_child(ElementId::new(2)).unwrap(); // Name input
    form_container.add_child(ElementId::new(3)).unwrap(); // Email input
    form_container.add_child(ElementId::new(4)).unwrap(); // Accept checkbox
    form_container.add_child(ElementId::new(5)).unwrap(); // Submit button
    
    assert_eq!(form_container.children().len(), 4);
    
    // Create individual elements
    let mut name_input = TextInputElement::new(ElementId::new(2), "Name");
    name_input.set_required(true);
    
    let mut email_input = TextInputElement::new(ElementId::new(3), "Email");
    email_input.set_required(true);
    
    let mut accept_checkbox = CheckboxElement::new(ElementId::new(4), "I accept the terms");
    
    let submit_button = ButtonElement::new(ElementId::new(5), "Submit");
    
    // Validate form
    assert!(name_input.validate().is_err()); // Empty
    assert!(email_input.validate().is_err()); // Empty
    assert!(accept_checkbox.validate().is_ok());
    assert!(submit_button.validate().is_ok());
    
    // Fill form
    name_input.set_value(serde_json::json!("John Doe")).unwrap();
    email_input.set_value(serde_json::json!("john@example.com")).unwrap();
    accept_checkbox.set_checked(true);
    
    // Validate again
    assert!(name_input.validate().is_ok());
    assert!(email_input.validate().is_ok());
    assert!(accept_checkbox.validate().is_ok());
}

#[test]
fn test_element_metadata() {
    let mut text = TextElement::new(ElementId::new(1), "Hello");
    
    // Add CSS classes
    text.add_class("primary");
    text.add_class("large");
    
    // Add inline styles
    text.add_style("color", "red");
    text.add_style("font-size", "16px");
    
    // Set ARIA attributes
    text.set_aria_label("Greeting text");
    text.set_aria_role("heading");
    
    let metadata = text.metadata();
    assert_eq!(metadata.css_classes.len(), 2);
    assert_eq!(metadata.inline_styles.len(), 2);
    assert_eq!(metadata.aria_label, Some("Greeting text".to_string()));
    assert_eq!(metadata.aria_role, Some("heading".to_string()));
}

#[test]
fn test_batch_element_creation() {
    let elements: Vec<_> = (0..100)
        .map(|i| TextElement::new(ElementId::new(i), format!("Element {}", i)))
        .collect();
    
    assert_eq!(elements.len(), 100);
    
    // Validate all
    let all_valid = elements.iter().all(|e| e.validate().is_ok());
    assert!(all_valid);
    
    // Serialize all
    let jsons: Vec<_> = elements.iter()
        .map(|e| e.to_json())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    
    assert_eq!(jsons.len(), 100);
}
