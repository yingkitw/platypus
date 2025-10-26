# Platypus Examples

This directory contains example applications demonstrating various features and patterns in Platypus. Each example runs a web server on `http://127.0.0.1:8501` that you can visit in your browser.

## Examples

### 1. Hello World (`hello_world.rs`)
A simple introductory example showing:
- Creating titles and markdown
- Text input widgets
- Button interactions
- Conditional rendering
- Display messages

**Run:**
```bash
cargo run -p platypus-examples --bin hello_world
```

Then open `http://127.0.0.1:8501` in your browser.

### 2. Calculator (`calculator.rs`)
A functional calculator demonstrating:
- Number input widgets
- Selectbox for operation selection
- Conditional logic
- Metric display
- Error handling (division by zero)

**Run:**
```bash
cargo run -p platypus-examples --bin calculator
```

Then open `http://127.0.0.1:8501` in your browser.

### 3. Data Explorer (`data_explorer.rs`)
An interactive data exploration interface showing:
- Sidebar layout and filters
- Columns layout
- Checkbox widgets
- Multiple display elements
- Conditional rendering based on filters

**Run:**
```bash
cargo run -p platypus-examples --bin data_explorer
```

Then open `http://127.0.0.1:8501` in your browser.

### 4. Form Example (`form_example.rs`)
A comprehensive form demonstrating:
- Multiple input widget types (text, number, selectbox, multiselect, checkbox)
- Form layout and organization
- Form validation
- Success/error feedback
- User registration workflow

**Run:**
```bash
cargo run -p platypus-examples --bin form_example
```

Then open `http://127.0.0.1:8501` in your browser.

## Creating Your Own Example

To create a new example:

1. Create a new `.rs` file in this directory
2. Implement a `main()` function using the Platypus API
3. Run with: `cargo run --example <filename_without_rs>`

## Tips

- Use `st.title()` for page headers
- Use `st.sidebar_*()` for sidebar controls
- Use `st.columns()` for multi-column layouts
- Use `st.divider()` to separate sections
- Use `st.markdown()` for formatted text
- Always provide a key for stateful widgets to avoid conflicts

## Next Steps

- Explore the [Platypus documentation](../README.md)
- Check the [API reference](../docs/)
- Review the [test suite](../crates/platypus-runtime/tests/) for more patterns
