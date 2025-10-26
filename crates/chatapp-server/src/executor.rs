//! Script execution and delta generation for handling user interactions.

use chatapp_core::state::{Delta, DeltaGenerator};
use chatapp_core::widget::WidgetValue;
use chatapp_runtime::{St, SessionStore};
use chatapp_core::session::SessionId;
use std::sync::Arc;

/// Handles script execution and generates UI deltas
pub struct ScriptExecutor {
    session_store: Arc<SessionStore>,
}

impl ScriptExecutor {
    /// Create a new script executor
    pub fn new(session_store: Arc<SessionStore>) -> Self {
        ScriptExecutor { session_store }
    }

    /// Execute a script and return deltas
    pub fn execute_script(&self, _session_id: SessionId) -> Result<Vec<Delta>, String> {
        // Get or create session
        let delta_gen = DeltaGenerator::new();
        let mut st = St::with_delta_gen(delta_gen.clone());

        // Execute the app logic (placeholder - would be user's script)
        self.run_app(&mut st)?;

        // Get deltas
        let deltas = st.delta_gen().take_deltas();
        Ok(deltas)
    }

    /// Handle widget state change and rerun script
    pub fn handle_widget_change(
        &self,
        session_id: SessionId,
        _widget_key: &str,
        value: &str,
    ) -> Result<Vec<Delta>, String> {
        // Parse widget value (JSON)
        let _parsed_value: WidgetValue = serde_json::from_str(value)
            .map_err(|e| format!("Failed to parse widget value: {}", e))?;

        // Store widget state in session (would update session here)
        let _session = self.session_store
            .get_session(session_id)
            .map_err(|e| e.to_string())?;

        // Rerun script with updated state
        self.execute_script(session_id)
    }

    /// Run the application logic (placeholder)
    fn run_app(&self, st: &mut St) -> Result<(), String> {
        // This is where user's app code would run
        // For now, just create a simple demo app
        st.title("Chatapp Demo");
        st.write("Welcome to Chatapp!");
        
        let name = st.text_input("Enter your name", "World", Some("name_input".to_string()));
        st.write(format!("Hello, {}!", name));

        if st.button("Click me!", Some("demo_button".to_string())) {
            st.success("Button clicked!");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chatapp_core::session::SessionId;

    #[test]
    fn test_executor_creation() {
        let session_store = Arc::new(SessionStore::new());
        let executor = ScriptExecutor::new(session_store);
        assert_eq!(std::mem::size_of_val(&executor) > 0, true);
    }

    #[test]
    fn test_execute_script() {
        let session_store = Arc::new(SessionStore::new());
        let executor = ScriptExecutor::new(session_store.clone());
        let session_id = session_store.create_session("test".to_string());

        let result = executor.execute_script(session_id);
        assert!(result.is_ok());
        
        let deltas = result.unwrap();
        assert!(!deltas.is_empty(), "Script should generate deltas");
    }

    #[test]
    fn test_handle_widget_change() {
        let session_store = Arc::new(SessionStore::new());
        let executor = ScriptExecutor::new(session_store.clone());
        let session_id = session_store.create_session("test".to_string());

        let result = executor.handle_widget_change(
            session_id,
            "test_widget",
            r#""test_value""#,
        );
        assert!(result.is_ok());
    }
}
