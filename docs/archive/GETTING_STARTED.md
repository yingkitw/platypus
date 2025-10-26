# Getting Started with platypus

## Overview

platypus is a high-performance Rust-based web app generator that provides a Streamlit-compatible API. This guide will help you get up and running with platypus development.

## Prerequisites

- Rust 1.70+ (Edition 2021)
- Cargo
- macOS, Linux, or Windows with WSL2

## Installation

### Clone the Repository

```bash
git clone https://github.com/yingkitw/platypus.git
cd platypus
```

### Build the Project

```bash
cargo build
```

This will compile all crates in the workspace:
- `platypus-core`: Core types and traits
- `platypus-proto`: Protocol Buffer definitions
- `platypus-runtime`: App runtime and state management
- `platypus-server`: Web server with WebSocket support
- `platypus-cli`: Command-line interface

### Run Tests

```bash
cargo test
```

## Project Structure

```
platypus/
├── crates/
│   ├── platypus-core/          # Core types, traits, elements, widgets
│   ├── platypus-proto/         # Protocol Buffer definitions
│   ├── platypus-runtime/       # Runtime engine, St context, session management
│   ├── platypus-server/        # HTTP/WebSocket server, handlers
│   └── platypus-cli/           # CLI tool (platypus run, build, new)
├── proto/                   # Proto source files
├── Cargo.toml              # Workspace manifest
├── README.md               # Project overview
├── ARCHITECTURE.md         # Detailed architecture
├── TODO.md                 # Development roadmap
└── LICENSE                 # Apache 2.0 license
```

## Quick Start

### 1. Build the CLI

```bash
cargo build --release --bin platypus
```

The binary will be at `target/release/platypus`.

### 2. Create a Simple App

Create `my_app.rs`:

```rust
use platypus::prelude::*;

fn main() {
    let mut st = St::new();
    
    st.title("Hello platypus!");
    st.write("Welcome to the web app generator");
    
    let name = st.text_input("Enter your name", "World", None);
    st.write(format!("Hello, {}!", name));
    
    if st.button("Click me", Some("btn".to_string())) {
        st.success("Button clicked!");
    }
}
```

### 3. Run the App

```bash
./target/release/platypus run my_app.rs
```

Visit `http://localhost:8501` in your browser.

## Core Concepts

### St Context

The `St` type is the main API for building apps. It provides methods for:

- **Display**: `write()`, `markdown()`, `code()`, `heading()`, `title()`, `header()`, `subheader()`
- **Input**: `text_input()`, `text_area()`, `number_input()`, `slider()`, `checkbox()`, `selectbox()`, `multiselect()`
- **Feedback**: `success()`, `error()`, `warning()`, `info()`, `progress()`
- **Layout**: `columns()`, `container()`
- **Media**: `image()`, `audio()`, `video()`
- **Other**: `json()`, `divider()`, `empty()`

### Elements

Elements are UI components that can be displayed. They're defined in `platypus-core::element::ElementType` and include:

- Text elements: Text, Markdown, Code, Heading
- Input widgets: Button, TextInput, Slider, Checkbox, etc.
- Data display: DataFrame, JSON, Image, Audio, Video
- Layout: Container, Column, Row, Tab, Expander
- Feedback: Success, Error, Warning, Info, Progress

### Widgets

Widgets are interactive elements with state. They're managed through:

- `WidgetValue`: Type-safe values (String, Number, Bool, Arrays, JSON)
- `Widget` trait: Interface for stateful widgets
- `DeltaGenerator`: Manages element and widget state

### Sessions

Each user gets a session with:

- Unique `SessionId`
- Script hash for tracking reruns
- Widget state persistence
- Metadata storage
- Activity tracking

## Development Workflow

### 1. Modify Core Types

Edit files in `crates/platypus-core/src/`:

```bash
cargo test -p platypus-core
```

### 2. Update Proto Definitions

Edit `.proto` files in `crates/platypus-proto/proto/`:

```bash
cargo build -p platypus-proto
```

### 3. Implement Runtime Features

Edit files in `crates/platypus-runtime/src/`:

```bash
cargo test -p platypus-runtime
```

### 4. Add Server Endpoints

Edit files in `crates/platypus-server/src/`:

```bash
cargo test -p platypus-server
```

### 5. Test the CLI

```bash
cargo run --bin platypus -- run my_app.rs
```

## Common Tasks

### Add a New Element Type

1. Define proto message in `crates/platypus-proto/proto/element.proto`
2. Add variant to `ElementType` enum in `crates/platypus-core/src/element.rs`
3. Add builder method to `St` in `crates/platypus-runtime/src/context.rs`
4. Implement frontend component (React)

### Add a New Widget

1. Define proto message in `crates/platypus-proto/proto/element.proto`
2. Add variant to `ElementType` enum
3. Implement `Widget` trait for state management
4. Add method to `St` context
5. Implement frontend component

### Create a Custom Component

1. Implement `Element` trait
2. Register with component registry
3. Provide React component for rendering
4. Add to `St` API

## Testing

### Unit Tests

```bash
cargo test --lib
```

### Integration Tests

```bash
cargo test --test '*'
```

### Specific Test

```bash
cargo test -p platypus-core test_element_id
```

## Debugging

### Enable Logging

```bash
RUST_LOG=debug cargo run --bin platypus -- run my_app.rs
```

### Run with Verbose Output

```bash
cargo run --bin platypus -- -v run my_app.rs
```

## Performance

### Build Release Binary

```bash
cargo build --release
```

### Profile Performance

```bash
cargo flamegraph --bin platypus
```

## Troubleshooting

### Build Fails

1. Update Rust: `rustup update`
2. Clean build: `cargo clean && cargo build`
3. Check dependencies: `cargo tree`

### WebSocket Connection Issues

1. Check firewall settings
2. Verify port 8501 is available
3. Check browser console for errors
4. Enable debug logging

### Proto Compilation Issues

1. Ensure proto files are valid
2. Run `cargo build -p platypus-proto` explicitly
3. Check for syntax errors in `.proto` files

## Next Steps

1. **Read ARCHITECTURE.md** for detailed design
2. **Check TODO.md** for planned features
3. **Review examples** in the repository
4. **Contribute** improvements and features

## Resources

- [Streamlit Documentation](https://docs.streamlit.io)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Documentation](https://tokio.rs)
- [Axum Web Framework](https://github.com/tokio-rs/axum)
- [Protocol Buffers](https://developers.google.com/protocol-buffers)

## Contributing

See CONTRIBUTING.md for guidelines on:

- Code style
- Testing requirements
- Commit messages
- Pull request process

## License

platypus is licensed under Apache 2.0. See LICENSE file for details.
