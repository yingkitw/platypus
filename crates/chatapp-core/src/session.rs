//! Session management for user sessions.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Unique session identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(Uuid);

impl SessionId {
    /// Create a new random session ID.
    pub fn new() -> Self {
        SessionId(Uuid::new_v4())
    }

    /// Create from UUID.
    pub fn from_uuid(uuid: Uuid) -> Self {
        SessionId(uuid)
    }

    /// Get the UUID.
    pub fn uuid(self) -> Uuid {
        self.0
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// User session state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Unique session identifier.
    pub id: SessionId,

    /// Hash of the current script.
    pub script_hash: String,

    /// Number of script reruns.
    pub reruns: u32,

    /// Session metadata.
    pub metadata: HashMap<String, String>,

    /// Creation timestamp (Unix seconds).
    pub created_at: u64,

    /// Last activity timestamp (Unix seconds).
    pub last_activity: u64,
}

impl Session {
    /// Create a new session.
    pub fn new(script_hash: String) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Session {
            id: SessionId::new(),
            script_hash,
            reruns: 0,
            metadata: HashMap::new(),
            created_at: now,
            last_activity: now,
        }
    }

    /// Increment rerun counter.
    pub fn increment_reruns(&mut self) {
        self.reruns += 1;
        self.update_activity();
    }

    /// Update last activity timestamp.
    pub fn update_activity(&mut self) {
        self.last_activity = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
    }

    /// Set metadata value.
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Get metadata value.
    pub fn get_metadata(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(|s| s.as_str())
    }

    /// Check if session is stale (no activity for more than timeout seconds).
    pub fn is_stale(&self, timeout_secs: u64) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        now - self.last_activity > timeout_secs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_id() {
        let id1 = SessionId::new();
        let id2 = SessionId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_session_creation() {
        let session = Session::new("script_hash".to_string());
        assert_eq!(session.script_hash, "script_hash");
        assert_eq!(session.reruns, 0);
    }

    #[test]
    fn test_session_reruns() {
        let mut session = Session::new("script_hash".to_string());
        session.increment_reruns();
        assert_eq!(session.reruns, 1);
    }

    #[test]
    fn test_session_metadata() {
        let mut session = Session::new("script_hash".to_string());
        session.set_metadata("key".to_string(), "value".to_string());
        assert_eq!(session.get_metadata("key"), Some("value"));
    }
}
