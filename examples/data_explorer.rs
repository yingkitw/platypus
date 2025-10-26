use platypus_server::server::AppServer;
use platypus_server::error::Result;
use platypus_runtime::prelude::*;

/// App logic for data explorer
fn app(st: &mut St) -> std::result::Result<(), String> {
    st.title("📊 Data Explorer");
    st.markdown("Explore and filter sample data with interactive controls");

    // Sidebar for filters
    let show_stats = st.checkbox("Show statistics", true, None);
    let show_raw = st.checkbox("Show raw data", false, None);

    st.divider();

    // Main content area with columns
    let _cols = st.columns(3);

    // Display metrics
    st.metric("Total Records", "1,000", None);
    st.metric("Filtered Records", "850", None);
    st.metric("Average Value", "45.2", None);

    st.divider();

    if show_stats {
        st.subheader("Statistics");
        st.markdown("- **Mean**: 45.2");
        st.markdown("- **Median**: 42.5");
        st.markdown("- **Std Dev**: 18.3");
    }

    if show_raw {
        st.subheader("Sample Data");
        st.code("id,name,value,category\n1,Item A,42,Category 1\n2,Item B,58,Category 2\n3,Item C,35,Category 1", Some("csv".to_string()));
    }

    st.divider();
    st.info("This is a demo. Connect to real data sources to explore actual datasets.");
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let server = AppServer::with_app(app);
    println!("🦆 Platypus Data Explorer Example");
    println!("🚀 Server starting on http://127.0.0.1:8501");
    server.run().await?;
    Ok(())
}
