# Webag Quick Reference

## Build Commands

```bash
# Build all crates
cargo build

# Build release binary
cargo build --release

# Build specific crate
cargo build -p webag-core

# Run tests
cargo test

# Run tests for specific crate
cargo test -p webag-runtime

# Format code
cargo fmt

# Lint code
cargo clippy
```

## CLI Commands

```bash
# Run an app
webag run app.rs

# Build for production
webag build app.rs --output dist

# Create new project
webag new my_project --template basic

# Show version
webag version
```

## St API Reference

### Display Elements

```rust
st.write("text")                           // Display text
st.markdown("# Heading")                   // Markdown
st.code("fn main() {}", Some("rust"))      // Code block
st.title("Title")                          // Level 1 heading
st.header("Header")                        // Level 2 heading
st.subheader("Subheader")                  // Level 3 heading
st.json(json!({"key": "value"}))           // JSON display
st.image("url", None, None)                // Image
st.divider()                               // Horizontal line
st.empty()                                 // Empty space
```

### Feedback

```rust
st.success("Success message")
st.error("Error message")
st.warning("Warning message")
st.info("Info message")
st.progress(0.75)                          // Progress bar (0.0-1.0)
```

### Input Widgets

```rust
// Text input
let name = st.text_input("Label", "default", None);

// Text area
let text = st.text_area("Label", "default", None);

// Number input
let num = st.number_input("Label", 0.0, None);

// Slider
let val = st.slider("Label", 0.0, 100.0, 50.0, None);

// Checkbox
let checked = st.checkbox("Label", false, None);

// Selectbox
let selected = st.selectbox("Label", vec!["A", "B", "C"], 0, None);

// Multiselect
let selected = st.multiselect("Label", vec!["A", "B", "C"], vec![], None);

// Button
if st.button("Click me", Some("btn_key")) {
    // Button was clicked
}
```

### Layout

```rust
// Columns
let cols = st.columns(3);
cols[0].st().write("Column 1");
cols[1].st().write("Column 2");
cols[2].st().write("Column 3");

// Container
let container = st.container();
container.st().write("Inside container");
```

## Element Types

```rust
ElementType::Text { value }
ElementType::Markdown { value }
ElementType::Code { value, language }
ElementType::Heading { value, level }
ElementType::Button { label, key }
ElementType::TextInput { label, value, key }
ElementType::Slider { label, value, min, max, key }
ElementType::Checkbox { label, value, key }
ElementType::Selectbox { label, options, value, key }
ElementType::Multiselect { label, options, values, key }
ElementType::Image { src, caption, width }
ElementType::Json { value }
ElementType::Progress { value }
ElementType::Success { message }
ElementType::Error { message }
ElementType::Warning { message }
ElementType::Info { message }
ElementType::Container { children }
ElementType::Column { children, width }
ElementType::Row { children }
ElementType::Divider
ElementType::Empty
```

## Widget Values

```rust
WidgetValue::String(s)           // Text value
WidgetValue::Number(n)           // Numeric value
WidgetValue::Bool(b)             // Boolean value
WidgetValue::StringArray(arr)    // Array of strings
WidgetValue::NumberArray(arr)    // Array of numbers
WidgetValue::Json(val)           // Generic JSON value
```

## Core Types

```rust
// Session
SessionId                         // Unique session identifier
Session                          // User session with state

// Elements
ElementId                        // Unique element identifier
Element                          // UI element trait
ElementType                      // Enum of all element types

// Widgets
Widget                           // Stateful widget trait
WidgetValue                      // Type-safe widget value

// State
DeltaGenerator                   // Manages elements and widgets
Delta                            // UI update (add/update/remove)
AppState                         // Global app state
```

## Proto Messages

### ForwardMsg (Server → Client)

```protobuf
NewSessionMsg                    // Initial session setup
DeltaMsg                         // UI updates
ScriptFinishedMsg                // Script execution status
SessionStatusMsg                 // Session status
ErrorMsg                         // Error information
```

### BackMsg (Client → Server)

```protobuf
WidgetStateChangeMsg             // Widget value changed
RerunScriptMsg                   // Request script rerun
UserInteractionMsg               // Custom interaction
```

## File Structure

```
webag/
├── crates/
│   ├── webag-core/
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── element.rs       # Element types
│   │       ├── widget.rs        # Widget types
│   │       ├── session.rs       # Session management
│   │       ├── state.rs         # State and deltas
│   │       └── error.rs         # Error types
│   ├── webag-proto/
│   │   ├── src/
│   │   │   └── lib.rs
│   │   ├── proto/
│   │   │   ├── element.proto
│   │   │   ├── forward_msg.proto
│   │   │   └── back_msg.proto
│   │   └── build.rs
│   ├── webag-runtime/
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── context.rs       # St API
│   │       ├── event.rs         # Event types
│   │       ├── session_store.rs # Session storage
│   │       └── error.rs
│   ├── webag-server/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── server.rs        # Main server
│   │   │   ├── handler.rs       # HTTP handlers
│   │   │   ├── ws.rs            # WebSocket handler
│   │   │   └── error.rs
│   │   └── frontend/
│   │       └── index.html
│   └── webag-cli/
│       └── src/
│           └── main.rs
├── proto/                       # Proto source files
├── Cargo.toml                   # Workspace manifest
├── README.md
├── ARCHITECTURE.md
├── GETTING_STARTED.md
├── TODO.md
└── LICENSE
```

## Common Patterns

### Simple Display App

```rust
use webag::prelude::*;

fn main() {
    let mut st = St::new();
    st.title("My App");
    st.write("Hello, World!");
}
```

### Interactive App

```rust
use webag::prelude::*;

fn main() {
    let mut st = St::new();
    st.title("Interactive App");
    
    let name = st.text_input("Name", "User", None);
    let age = st.number_input("Age", 0.0, None);
    
    st.write(format!("{} is {} years old", name, age));
}
```

### Multi-Column Layout

```rust
use webag::prelude::*;

fn main() {
    let mut st = St::new();
    let cols = st.columns(2);
    
    cols[0].st().write("Left column");
    cols[1].st().write("Right column");
}
```

## Debugging

```bash
# Enable debug logging
RUST_LOG=debug cargo run --bin webag -- run app.rs

# Verbose CLI output
cargo run --bin webag -- -v run app.rs

# Run tests with output
cargo test -- --nocapture

# Check compilation
cargo check
```

## Performance Tips

1. Use release builds for production: `cargo build --release`
2. Minimize delta generation - only update changed elements
3. Use appropriate data structures (Vec vs HashMap)
4. Cache expensive computations
5. Profile with flamegraph: `cargo flamegraph`

## Useful Resources

- **ARCHITECTURE.md**: Detailed system design
- **TODO.md**: Roadmap and planned features
- **GETTING_STARTED.md**: Comprehensive setup guide
- **Streamlit Docs**: API reference and examples
- **Rust Book**: Language fundamentals
