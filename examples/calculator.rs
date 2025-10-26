use platypus_server::server::AppServer;
use platypus_server::error::Result;
use platypus_runtime::prelude::*;

/// App logic for calculator
fn app(st: &mut St) -> std::result::Result<(), String> {
    st.title("Simple Calculator");
    st.markdown("Perform basic arithmetic operations");

    // Input fields
    let num1 = st.number_input("First number", 0.0, Some("num1".to_string()));
    let num2 = st.number_input("Second number", 0.0, Some("num2".to_string()));

    st.divider();

    // Operation selection
    let operation = st.selectbox(
        "Select operation",
        vec!["Add".to_string(), "Subtract".to_string(), "Multiply".to_string(), "Divide".to_string()],
        0,
        None,
    );

    st.divider();

    // Calculate result
    let result = match operation.as_str() {
        "Add" => num1 + num2,
        "Subtract" => num1 - num2,
        "Multiply" => num1 * num2,
        "Divide" => {
            if num2 == 0.0 {
                st.error("Cannot divide by zero!");
                return Ok(());
            }
            num1 / num2
        }
        _ => 0.0,
    };

    // Display result
    st.metric("Result", format!("{:.2}", result), None);

    st.info(&format!(
        "{} {} {} = {:.2}",
        num1, operation, num2, result
    ));
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let server = AppServer::with_app(app);
    println!("🦆 Platypus Calculator Example");
    println!("🚀 Server starting on http://127.0.0.1:8501");
    server.run().await?;
    Ok(())
}
