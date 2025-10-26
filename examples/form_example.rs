use platypus_server::server::AppServer;
use platypus_server::error::Result;
use platypus_runtime::prelude::*;

/// App logic for form example
fn app(st: &mut St) -> std::result::Result<(), String> {
    st.title("📝 User Registration Form");
    st.markdown("Fill out the form below to register");

    st.divider();

    // Personal Information Section
    st.subheader("Personal Information");
    
    let first_name = st.text_input("First Name", "", None);
    let last_name = st.text_input("Last Name", "", None);
    let email = st.text_input("Email", "", None);
    
    st.divider();

    // Preferences Section
    st.subheader("Preferences");
    
    let age = st.number_input("Age", 18.0, None);
    
    let country = st.selectbox(
        "Country",
        vec!["United States".to_string(), "Canada".to_string(), "United Kingdom".to_string(), "Australia".to_string(), "Other".to_string()],
        0,
        None,
    );

    let interests = st.multiselect(
        "Interests (select multiple)",
        vec!["Data Science".to_string(), "Web Development".to_string(), "DevOps".to_string(), "Machine Learning".to_string(), "Cloud Computing".to_string()],
        vec![],
        None,
    );

    st.divider();

    // Terms and Conditions
    st.subheader("Terms");
    
    let agree_terms = st.checkbox("I agree to the terms and conditions", false, None);
    let subscribe_newsletter = st.checkbox("Subscribe to newsletter", true, None);

    st.divider();

    // Submit button
    if st.button("Register", Some("register_btn".to_string())) {
        // Validation
        if first_name.is_empty() || last_name.is_empty() || email.is_empty() {
            st.error("Please fill in all required fields");
        } else if !agree_terms {
            st.error("You must agree to the terms and conditions");
        } else {
            st.success("✅ Registration successful!");
            st.markdown(&format!(
                "Welcome, {} {}! A confirmation email has been sent to {}",
                first_name, last_name, email
            ));
        }
    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let server = AppServer::with_app(app);
    println!("🦆 Platypus Form Example");
    println!("🚀 Server starting on http://127.0.0.1:8501");
    server.run().await?;
    Ok(())
}
