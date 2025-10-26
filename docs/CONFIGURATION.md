# Configuration Guide

**Status**: ✅ No Hardcoding - All values centralized in configuration

---

## Overview

The Platypus codebase has been refactored to eliminate hardcoded values. All configuration constants are centralized in `crates/platypus-server/src/config.rs`, making it easy to adjust settings in one place.

---

## Configuration Constants

### Server Configuration

| Constant | Value | Purpose |
|----------|-------|---------|
| `DEFAULT_APP_NAME` | `"Platypus App"` | Application name |
| `DEFAULT_HOST` | `"127.0.0.1"` | Server host address |
| `DEFAULT_PORT` | `8501` | Server port number |
| `DEFAULT_MAX_BODY_SIZE` | `100 * 1024 * 1024` | Max request body (100 MB) |
| `DEFAULT_SESSION_TIMEOUT` | `3600` | Session timeout (1 hour) |

### CLI Configuration

| Constant | Value | Purpose |
|----------|-------|---------|
| `DEFAULT_OUTPUT_DIR` | `"dist"` | Build output directory |
| `DEFAULT_TEMPLATE` | `"basic"` | Default project template |

### Endpoint Paths

| Constant | Value | Purpose |
|----------|-------|---------|
| `HEALTH_CHECK_PATH` | `"/health"` | Health check endpoint |
| `APP_INFO_PATH` | `"/api/info"` | App info endpoint |
| `INDEX_PATH` | `"/"` | Index page path |
| `WEBSOCKET_PATH` | `"/ws"` | WebSocket endpoint |

### Logging Configuration

| Constant | Value | Purpose |
|----------|-------|---------|
| `VERBOSE_LOG_LEVEL` | `"debug"` | Verbose logging level |
| `NORMAL_LOG_LEVEL` | `"info"` | Normal logging level |

---

## Helper Functions

### `session_timeout_duration() -> Duration`
Converts session timeout seconds to a `Duration` object.

```rust
let timeout = config::session_timeout_duration();
// Returns: Duration::from_secs(3600)
```

### `max_body_size_usize() -> usize`
Converts max body size to `usize` for use in middleware.

```rust
let size = config::max_body_size_usize();
// Returns: 104857600 (100 MB)
```

---

## Usage Examples

### Server Configuration

```rust
use platypus_server::config;

// Using default configuration
let config = ServerConfig::default();

// Custom configuration
let config = ServerConfig {
    app_name: config::DEFAULT_APP_NAME.to_string(),
    host: "0.0.0.0".to_string(),  // Override host
    port: config::DEFAULT_PORT,
    max_body_size: config::DEFAULT_MAX_BODY_SIZE,
    session_timeout: config::DEFAULT_SESSION_TIMEOUT,
};
```

### Router Configuration

```rust
use platypus_server::config;

Router::new()
    .route(config::HEALTH_CHECK_PATH, get(handler::health))
    .route(config::APP_INFO_PATH, get(handler::app_info))
    .route(config::INDEX_PATH, get(handler::index))
    .route(config::WEBSOCKET_PATH, get(ws_handler))
    .layer(DefaultBodyLimit::max(config::max_body_size_usize()))
```

### CLI Usage

```rust
use platypus_server::config;

// Default port from configuration
let port = config::DEFAULT_PORT;

// Default output directory
let output = config::DEFAULT_OUTPUT_DIR;

// Default template
let template = config::DEFAULT_TEMPLATE;
```

---

## Modifying Configuration

To change any configuration value:

1. Open `crates/platypus-server/src/config.rs`
2. Update the constant value
3. All code using that constant will automatically use the new value

### Example: Change Default Port

**Before**:
```rust
pub const DEFAULT_PORT: u16 = 8501;
```

**After**:
```rust
pub const DEFAULT_PORT: u16 = 3000;
```

All code using `config::DEFAULT_PORT` will now use port 3000.

---

## Environment Variables

For runtime configuration via environment variables, extend the configuration module:

```rust
use std::env;

pub fn get_port() -> u16 {
    env::var("PLATYPUS_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(DEFAULT_PORT)
}

pub fn get_host() -> String {
    env::var("PLATYPUS_HOST")
        .unwrap_or_else(|_| DEFAULT_HOST.to_string())
}
```

---

## Testing Configuration

All configuration constants are tested in `config.rs`:

```rust
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
```

---

## Best Practices

### ✅ DO

- Use configuration constants instead of hardcoded values
- Add new constants to `config.rs` for any magic values
- Document constants with comments
- Test configuration values
- Use helper functions for type conversions

### ❌ DON'T

- Hardcode strings, numbers, or paths
- Duplicate configuration values
- Use magic numbers without explanation
- Forget to update tests when changing configuration

---

## Configuration Locations

### Server Configuration
**File**: `crates/platypus-server/src/config.rs`
- Server settings (host, port, timeouts)
- Endpoint paths
- Logging levels

### CLI Configuration
**File**: `crates/platypus-cli/src/main.rs`
- Uses constants from `platypus_server::config`
- CLI-specific arguments

### Runtime Configuration
**File**: `crates/platypus-runtime/src/`
- Runtime-specific settings (if needed)

---

## Adding New Configuration

### Step 1: Define Constant
```rust
// In config.rs
pub const NEW_SETTING: u32 = 1000;
```

### Step 2: Add Helper Function (if needed)
```rust
pub fn new_setting_as_duration() -> Duration {
    Duration::from_millis(NEW_SETTING as u64)
}
```

### Step 3: Add Tests
```rust
#[test]
fn test_new_setting() {
    assert_eq!(NEW_SETTING, 1000);
}
```

### Step 4: Use in Code
```rust
use crate::config;

let value = config::NEW_SETTING;
```

---

## Summary

✅ **No Hardcoding**: All configuration values are centralized
✅ **Easy to Modify**: Change values in one place
✅ **Well Tested**: Configuration constants are tested
✅ **Type Safe**: Rust's type system ensures correctness
✅ **Documented**: All constants are documented
✅ **Extensible**: Easy to add new configuration values

---

**Last Updated**: October 26, 2025
**Status**: ✅ Complete - All hardcoded values eliminated
