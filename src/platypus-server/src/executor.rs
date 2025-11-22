//! Script execution and delta generation for handling user interactions.

use platypus_core::state::{Delta, DeltaGenerator};
use platypus_core::widget::WidgetValue;
use platypus_runtime::{St, SessionStore};
use platypus_core::session::SessionId;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;

/// App function type for custom app logic
pub type AppFn = fn(&mut St) -> Result<(), String>;

/// Widget state storage
type WidgetState = Arc<Mutex<HashMap<String, String>>>;

/// Handles script execution and generates UI deltas
pub struct ScriptExecutor {
    session_store: Arc<SessionStore>,
    app_fn: Option<AppFn>,
    widget_state: WidgetState,
}

impl ScriptExecutor {
    /// Create a new script executor
    pub fn new(session_store: Arc<SessionStore>) -> Self {
        ScriptExecutor { 
            session_store, 
            app_fn: None,
            widget_state: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Create a new script executor with custom app function
    pub fn with_app(session_store: Arc<SessionStore>, app_fn: AppFn) -> Self {
        ScriptExecutor { 
            session_store, 
            app_fn: Some(app_fn),
            widget_state: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Execute a script and return deltas
    pub fn execute_script(&self, _session_id: SessionId) -> Result<Vec<Delta>, String> {
        // Get or create session
        let delta_gen = DeltaGenerator::new();
        
        // Restore widget state from previous interactions
        if let Ok(state) = self.widget_state.lock() {
            for (key, value) in state.iter() {
                // Try to parse as number first, then as string
                if let Ok(num) = value.parse::<f64>() {
                    delta_gen.set_widget(key.clone(), platypus_core::widget::WidgetValue::Number(num));
                } else {
                    delta_gen.set_widget(key.clone(), platypus_core::widget::WidgetValue::String(value.clone()));
                }
            }
        }
        
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
        widget_key: &str,
        value: &str,
    ) -> Result<Vec<Delta>, String> {
        // Store widget state
        if let Ok(mut state) = self.widget_state.lock() {
            state.insert(widget_key.to_string(), value.to_string());
            tracing::debug!("Stored widget state: {} = {}", widget_key, value);
        }

        // Rerun script with updated state
        self.execute_script(session_id)
    }

    /// Run the application logic
    fn run_app(&self, st: &mut St) -> Result<(), String> {
        if let Some(app_fn) = self.app_fn {
            app_fn(st)
        } else {
            // Default demo app
            st.title("Platypus Demo");
            st.write("Welcome to Platypus!");
            
            let name = st.text_input("Enter your name", "World", Some("name_input".to_string()));
            st.write(format!("Hello, {}!", name));

            if st.button("Click me!", Some("demo_button".to_string())) {
                st.success("Button clicked!");
            }

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use platypus_core::session::SessionId;

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
