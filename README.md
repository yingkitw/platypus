# Chatapp - Web App Generator

A high-performance Rust-based web app generator inspired by Streamlit, designed for building interactive data applications with ease and speed.

## Overview

**Chatapp** is a Rust implementation of a Streamlit-compatible framework that enables developers to create interactive web applications with minimal code. It maintains API compatibility with Streamlit while providing superior performance and developer experience.

### Key Features

- **Streamlit-Compatible API**: Write apps using familiar `st.write()`, `st.button()`, etc.
- **High Performance**: Built in Rust for speed and efficiency
- **Easy Development**: Simple, Pythonic-like API for Rust
- **Real-time Interactivity**: WebSocket-based communication for instant feedback
- **Modern UI**: Carbon Design System styling with React frontend
- **Modular Architecture**: Trait-based design for extensibility

## Quick Start

### Installation

```bash
cargo build --release
```

### Create Your First App

Create `app.rs`:

```rust
use webag::prelude::*;

#[webag::app]
fn main(st: &mut St) {
    st.title("Hello Webag!");
    
    let name = st.text_input("Enter your name", "World");
    st.write(format!("Hello, {}!", name));
    
    if st.button("Click me") {
        st.success("Button clicked!");
    }
}
```

Run your app:

```bash
webag run app.rs
```

Visit `http://localhost:8501` in your browser.

## Architecture

### Crates

- **webag-core**: Core types, traits, and element definitions
- **webag-proto**: Protocol Buffer definitions for client-server communication
- **webag-runtime**: App runtime, state management, and execution engine
- **webag-server**: Web server with WebSocket support
- **webag-cli**: Command-line interface

### Communication

Webag uses Protocol Buffers over WebSocket for efficient client-server communication, maintaining compatibility with Streamlit's message format where applicable.

## Supported Elements

### Display Elements ✅
- Text, markdown, code
- Headings (h1, h2, h3)
- JSON display
- Images
- Dividers, empty space
- Tables
- DataFrames

### Input Widgets ✅
- Text input, text area
- Number input
- Slider
- Checkbox
- Radio buttons
- Selectbox, multiselect
- Button
- Date/time pickers
- Color picker
- File uploader
- Camera input

### Feedback Elements ✅
- Success, error, warning, info messages
- Progress bars
- Metrics with deltas

### Layout ✅
- Columns
- Containers
- Tabs
- Expanders
- Sidebars

### Coming in Phase 3+
- Charts (Plotly, Vega-Lite, Bokeh)
- Custom components
- Multi-page apps
- Caching decorators

## Development

### Build

```bash
cargo build
```

### Run Tests

```bash
cargo test
```

### Format & Lint

```bash
cargo fmt
cargo clippy
```

## Project Structure

```
webag/
├── crates/
│   ├── webag-core/          # Core types and traits
│   ├── webag-proto/         # Protocol Buffer definitions
│   ├── webag-runtime/       # Runtime engine
│   ├── webag-server/        # Web server
│   └── webag-cli/           # CLI tool
├── frontend/                # React/TypeScript UI
├── proto/                   # Proto source files
├── tests/                   # Integration tests
├── Cargo.toml              # Workspace manifest
└── README.md               # This file
```

## License

Webag is licensed under the Apache License 2.0. See [LICENSE](LICENSE) for details.

## Compatibility

Webag maintains **strong API compatibility** with Streamlit 1.x. This is verified by a comprehensive test suite with **39 passing tests** covering:

- ✅ 9 display methods (write, markdown, code, headings, metrics, etc.)
- ✅ 12 input widgets (text, numbers, sliders, selections, etc.)
- ✅ 5 feedback elements (success, error, warning, info, progress)
- ✅ 5 layout components (columns, containers, tabs, expanders, sidebar)
- ✅ Complex workflows (forms, dashboards, settings pages)
- ✅ State management and conditional rendering
- ✅ Performance benchmarks (1000 elements in <500ms)

See [Streamlit Compatibility Tests](docs/STREAMLIT_COMPATIBILITY_TESTS.md) for detailed test results.

## Contributing

Contributions are welcome! Please follow the project's code style and add tests for new features.

## Roadmap

- [ ] Core API implementation
- [ ] Full widget support
- [ ] Advanced data visualization
- [ ] Custom components
- [ ] Multi-page apps
- [ ] Caching and performance optimization
- [ ] Deployment tools
