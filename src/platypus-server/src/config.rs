//! Configuration constants and defaults for the Platypus server.
//!
//! This module centralizes all configuration values, preventing hardcoding
//! and making it easy to adjust settings in one place.

use std::time::Duration;

/// Default application name
pub const DEFAULT_APP_NAME: &str = "Platypus App";

/// Default host to bind to
pub const DEFAULT_HOST: &str = "127.0.0.1";

/// Default port to listen on
pub const DEFAULT_PORT: u16 = 8501;

/// Default maximum body size (100 MB)
pub const DEFAULT_MAX_BODY_SIZE: u64 = 100 * 1024 * 1024;

/// Default session timeout (1 hour in seconds)
pub const DEFAULT_SESSION_TIMEOUT: u64 = 3600;

/// Default output directory for builds
pub const DEFAULT_OUTPUT_DIR: &str = "dist";

/// Default template for new projects
pub const DEFAULT_TEMPLATE: &str = "basic";

/// Health check endpoint path
pub const HEALTH_CHECK_PATH: &str = "/health";

/// App info endpoint path
pub const APP_INFO_PATH: &str = "/api/info";

/// Index page path
pub const INDEX_PATH: &str = "/";

/// WebSocket endpoint path
pub const WEBSOCKET_PATH: &str = "/ws";

/// Log level for verbose mode
pub const VERBOSE_LOG_LEVEL: &str = "debug";

/// Log level for normal mode
pub const NORMAL_LOG_LEVEL: &str = "info";

/// Session timeout as Duration
pub fn session_timeout_duration() -> Duration {
    Duration::from_secs(DEFAULT_SESSION_TIMEOUT)
}

/// Maximum body size as usize
pub fn max_body_size_usize() -> usize {
    DEFAULT_MAX_BODY_SIZE as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_constants() {
        assert_eq!(DEFAULT_PORT, 8501);
        assert_eq!(DEFAULT_HOST, "127.0.0.1");
        assert_eq!(DEFAULT_APP_NAME, "Platypus App");
    }

    #[test]
    fn test_session_timeout_duration() {
        let duration = session_timeout_duration();
        assert_eq!(duration.as_secs(), DEFAULT_SESSION_TIMEOUT);
    }

    #[test]
    fn test_max_body_size_usize() {
        let size = max_body_size_usize();
        assert_eq!(size, DEFAULT_MAX_BODY_SIZE as usize);
    }
}
