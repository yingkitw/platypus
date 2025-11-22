//! Session storage and management.

use crate::error::Result;
use dashmap::DashMap;
use std::sync::Arc;
use platypus_core::session::{Session, SessionId};

/// Manages active sessions.
pub struct SessionStore {
    sessions: Arc<DashMap<String, Session>>,
}

impl SessionStore {
    /// Create a new session store.
    pub fn new() -> Self {
        SessionStore {
            sessions: Arc::new(DashMap::new()),
        }
    }

    /// Create a new session.
    pub fn create_session(&self, script_hash: String) -> SessionId {
        let session = Session::new(script_hash);
        let session_id = session.id;
        self.sessions.insert(session_id.to_string(), session);
        session_id
    }

    /// Get a session.
    pub fn get_session(&self, session_id: SessionId) -> Result<Session> {
        self.sessions
            .get(&session_id.to_string())
            .map(|entry| entry.clone())
            .ok_or_else(|| crate::error::Error::session(format!("Session not found: {}", session_id)))
    }

    /// Update a session.
    pub fn update_session(&self, session: Session) -> Result<()> {
        self.sessions.insert(session.id.to_string(), session);
        Ok(())
    }

    /// Remove a session.
    pub fn remove_session(&self, session_id: SessionId) -> Result<()> {
        self.sessions.remove(&session_id.to_string());
        Ok(())
    }

    /// Get all sessions.
    pub fn all_sessions(&self) -> Vec<Session> {
        self.sessions.iter().map(|entry| entry.value().clone()).collect()
    }

    /// Clean up stale sessions (no activity for more than timeout seconds).
    pub fn cleanup_stale_sessions(&self, timeout_secs: u64) {
        self.sessions.retain(|_, session| !session.is_stale(timeout_secs));
    }

    /// Get session count.
    pub fn session_count(&self) -> usize {
        self.sessions.len()
    }
}

impl Default for SessionStore {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for SessionStore {
    fn clone(&self) -> Self {
        SessionStore {
            sessions: Arc::clone(&self.sessions),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_session() {
        let store = SessionStore::new();
        let session_id = store.create_session("script_hash".to_string());
        assert!(store.get_session(session_id).is_ok());
    }

    #[test]
    fn test_get_session() {
        let store = SessionStore::new();
        let session_id = store.create_session("script_hash".to_string());
        let session = store.get_session(session_id).unwrap();
        assert_eq!(session.script_hash, "script_hash");
    }

    #[test]
    fn test_remove_session() {
        let store = SessionStore::new();
        let session_id = store.create_session("script_hash".to_string());
        store.remove_session(session_id).unwrap();
        assert!(store.get_session(session_id).is_err());
    }
}
