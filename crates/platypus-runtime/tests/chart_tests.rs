//! Chart functionality tests
//! Tests for all chart types: line, bar, area, scatter, pie, plotly, vega-lite

use platypus_runtime::prelude::*;

// ============================================================================
// SIMPLE CHART TESTS
// ============================================================================

#[test]
fn test_line_chart() {
    let mut st = St::new();
    
    let data = r#"[{"x": 1, "y": 10}, {"x": 2, "y": 20}, {"x": 3, "y": 15}]"#;
    let id = st.line_chart(data, Some("Sales Trend".to_string()));
    
    assert!(st.delta_gen().get_element(id).is_some());
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 1);
}

#[test]
fn test_bar_chart() {
    let mut st = St::new();
    
    let data = r#"[{"category": "A", "value": 100}, {"category": "B", "value": 200}]"#;
    let id = st.bar_chart(data, Some("Sales by Category".to_string()));
    
    assert!(st.delta_gen().get_element(id).is_some());
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 1);
}

#[test]
fn test_area_chart() {
    let mut st = St::new();
    
    let data = r#"[{"x": 1, "y": 10}, {"x": 2, "y": 25}, {"x": 3, "y": 20}]"#;
    let id = st.area_chart(data, Some("Revenue".to_string()));
    
    assert!(st.delta_gen().get_element(id).is_some());
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 1);
}

#[test]
fn test_scatter_chart() {
    let mut st = St::new();
    
    let data = r#"[{"x": 1, "y": 10}, {"x": 2, "y": 20}, {"x": 3, "y": 15}]"#;
    let id = st.scatter_chart(data, Some("Correlation".to_string()));
    
    assert!(st.delta_gen().get_element(id).is_some());
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 1);
}

#[test]
fn test_pie_chart() {
    let mut st = St::new();
    
    let data = r#"[{"label": "A", "value": 30}, {"label": "B", "value": 70}]"#;
    let id = st.pie_chart(data, Some("Market Share".to_string()));
    
    assert!(st.delta_gen().get_element(id).is_some());
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 1);
}

// ============================================================================
// ADVANCED CHART TESTS
// ============================================================================

#[test]
fn test_plotly_chart() {
    let mut st = St::new();
    
    let spec = r#"{
        "data": [{"x": [1, 2, 3], "y": [10, 20, 15], "type": "scatter"}],
        "layout": {"title": "Plotly Chart"}
    }"#;
    
    let id = st.plotly_chart(spec);
    assert!(st.delta_gen().get_element(id).is_some());
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 1);
}

#[test]
fn test_vega_lite_chart() {
    let mut st = St::new();
    
    let spec = r#"{
        "$schema": "https://vega.github.io/schema/vega-lite/v5.json",
        "data": {"values": [{"x": 1, "y": 10}, {"x": 2, "y": 20}]},
        "mark": "line",
        "encoding": {"x": {"field": "x"}, "y": {"field": "y"}}
    }"#;
    
    let id = st.vega_lite_chart(spec);
    assert!(st.delta_gen().get_element(id).is_some());
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 1);
}

// ============================================================================
// CHART WORKFLOW TESTS
// ============================================================================

#[test]
fn test_dashboard_with_charts() {
    let mut st = St::new();
    
    st.title("Sales Dashboard");
    st.markdown("Real-time sales metrics");
    
    // Metrics
    st.metric("Total Sales", "$10,000", Some("+5%".to_string()));
    st.metric("Orders", "250", Some("+10%".to_string()));
    
    st.divider();
    
    // Charts
    st.markdown("### Sales Trends");
    let line_data = r#"[{"month": "Jan", "sales": 1000}, {"month": "Feb", "sales": 1500}]"#;
    st.line_chart(line_data, Some("Monthly Sales".to_string()));
    
    st.markdown("### Sales by Category");
    let bar_data = r#"[{"category": "Electronics", "sales": 5000}, {"category": "Clothing", "sales": 3000}]"#;
    st.bar_chart(bar_data, Some("Sales by Category".to_string()));
    
    st.markdown("### Market Share");
    let pie_data = r#"[{"product": "A", "share": 40}, {"product": "B", "share": 60}]"#;
    st.pie_chart(pie_data, Some("Market Share".to_string()));
    
    let deltas = st.take_deltas();
    assert!(deltas.len() >= 8);
}

#[test]
fn test_analytics_dashboard() {
    let mut st = St::new();
    
    st.title("Analytics Dashboard");
    
    // Top metrics
    st.metric("Page Views", "10,234", Some("+8%".to_string()));
    st.metric("Unique Users", "2,456", Some("+12%".to_string()));
    st.metric("Bounce Rate", "32%", Some("-2%".to_string()));
    
    st.divider();
    
    // Charts
    st.markdown("### Traffic Over Time");
    let traffic_data = r#"[
        {"date": "2025-01-01", "views": 1000},
        {"date": "2025-01-02", "views": 1200},
        {"date": "2025-01-03", "views": 1100}
    ]"#;
    st.area_chart(traffic_data, Some("Page Views".to_string()));
    
    st.markdown("### User Engagement");
    let engagement_data = r#"[
        {"metric": "Clicks", "value": 5000},
        {"metric": "Shares", "value": 1200},
        {"metric": "Comments", "value": 800}
    ]"#;
    st.bar_chart(engagement_data, Some("Engagement Metrics".to_string()));
    
    let deltas = st.take_deltas();
    assert!(deltas.len() >= 7);
}

#[test]
fn test_financial_dashboard() {
    let mut st = St::new();
    
    st.title("Financial Dashboard");
    
    // Key metrics
    st.metric("Portfolio Value", "$100,000", Some("+2.5%".to_string()));
    st.metric("YTD Return", "12.3%", Some("+1.2%".to_string()));
    
    st.divider();
    
    // Charts
    st.markdown("### Asset Allocation");
    let allocation_data = r#"[
        {"asset": "Stocks", "percentage": 60},
        {"asset": "Bonds", "percentage": 30},
        {"asset": "Cash", "percentage": 10}
    ]"#;
    st.pie_chart(allocation_data, Some("Portfolio Allocation".to_string()));
    
    st.markdown("### Performance");
    let performance_data = r#"[
        {"month": "Jan", "return": 2.1},
        {"month": "Feb", "return": 1.5},
        {"month": "Mar", "return": 3.2}
    ]"#;
    st.line_chart(performance_data, Some("Monthly Returns".to_string()));
    
    let deltas = st.take_deltas();
    assert!(deltas.len() >= 6);
}

// ============================================================================
// MULTIPLE CHARTS TEST
// ============================================================================

#[test]
fn test_multiple_charts_in_app() {
    let mut st = St::new();
    
    st.title("Multi-Chart Application");
    
    // Line chart
    st.markdown("### Line Chart");
    st.line_chart(r#"[{"x": 1, "y": 10}]"#, None);
    
    // Bar chart
    st.markdown("### Bar Chart");
    st.bar_chart(r#"[{"x": "A", "y": 20}]"#, None);
    
    // Area chart
    st.markdown("### Area Chart");
    st.area_chart(r#"[{"x": 1, "y": 15}]"#, None);
    
    // Scatter chart
    st.markdown("### Scatter Chart");
    st.scatter_chart(r#"[{"x": 1, "y": 10}]"#, None);
    
    // Pie chart
    st.markdown("### Pie Chart");
    st.pie_chart(r#"[{"label": "A", "value": 50}]"#, None);
    
    let deltas = st.take_deltas();
    assert!(deltas.len() >= 10);
}

// ============================================================================
// CHART WITH FILTERS TEST
// ============================================================================

#[test]
fn test_chart_with_filters() {
    let mut st = St::new();
    
    st.title("Filtered Chart View");
    
    // Filters
    st.markdown("### Filters");
    let date_range = st.selectbox(
        "Date Range",
        vec!["Last 7 days".to_string(), "Last 30 days".to_string(), "Last 90 days".to_string()],
        0,
        Some("date_range_key".to_string()),
    );
    
    let category = st.selectbox(
        "Category",
        vec!["All".to_string(), "Electronics".to_string(), "Clothing".to_string()],
        0,
        Some("category_key".to_string()),
    );
    
    st.divider();
    
    // Chart based on filters
    st.markdown("### Sales Chart");
    let chart_data = format!(
        r#"[{{"period": "{}", "category": "{}", "sales": 5000}}]"#,
        date_range, category
    );
    st.line_chart(chart_data, Some("Filtered Sales".to_string()));
    
    let deltas = st.take_deltas();
    assert!(deltas.len() >= 5);
}

// ============================================================================
// PERFORMANCE TEST
// ============================================================================

#[test]
fn test_chart_performance() {
    let mut st = St::new();
    
    let start = std::time::Instant::now();
    
    // Create multiple charts
    for i in 0..50 {
        st.line_chart(
            format!(r#"[{{"x": 1, "y": {}}}]"#, i),
            Some(format!("Chart {}", i)),
        );
    }
    
    let duration = start.elapsed();
    
    // Should be fast
    assert!(duration.as_millis() < 100, "Chart creation took {:?}", duration);
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 50);
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn test_chart_without_title() {
    let mut st = St::new();
    
    st.line_chart(r#"[{"x": 1, "y": 10}]"#, None);
    st.bar_chart(r#"[{"x": "A", "y": 20}]"#, None);
    st.pie_chart(r#"[{"label": "A", "value": 50}]"#, None);
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 3);
}

#[test]
fn test_empty_chart_data() {
    let mut st = St::new();
    
    st.line_chart("[]", Some("Empty Chart".to_string()));
    st.bar_chart("[]", Some("Empty Chart".to_string()));
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 2);
}

#[test]
fn test_complex_chart_data() {
    let mut st = St::new();
    
    let complex_data = r#"[
        {"x": 1, "y": 10, "size": 100, "color": "red"},
        {"x": 2, "y": 20, "size": 200, "color": "blue"},
        {"x": 3, "y": 15, "size": 150, "color": "green"}
    ]"#;
    
    st.scatter_chart(complex_data, Some("Complex Data".to_string()));
    
    let deltas = st.take_deltas();
    assert_eq!(deltas.len(), 1);
}
