use platypus_server::server::AppServer;
use platypus_server::error::Result;
use platypus_runtime::prelude::*;

/// App logic for hello world
fn app(st: &mut St) -> std::result::Result<(), String> {
    st.title("Hello Platypus! 👋");
    st.markdown("Welcome to your first Platypus app. This example demonstrates basic widgets and interactivity.");

    // Text input
    let name = st.text_input("What's your name?", "World", None);
    
    st.write(format!("Hello, {}!", name));

    // Button interaction
    if st.button("Click me!", Some("hello_btn".to_string())) {
        st.success("Button clicked! 🎉");
    }

    st.divider();
    st.info("Platypus is a Rust-native data app framework with Streamlit-compatible API");
    
    Ok(())
}

/// A simple "Hello World" example demonstrating basic Platypus features.
/// 
/// This example shows:
/// - Creating a title
/// - Text input widget
/// - Conditional rendering with buttons
/// - Display messages
/// - Running with a web server
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create and run the server with app logic
    let server = AppServer::with_app(app);
    
    println!("🦆 Platypus Hello World Example");
    println!("🚀 Server starting on http://127.0.0.1:8501");
    println!("📖 Open your browser to view the app");
    
    server.run().await?;
    
    Ok(())
}
