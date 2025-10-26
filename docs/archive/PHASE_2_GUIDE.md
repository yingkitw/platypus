# Phase 2: Core API Enhancement Guide

## Overview

Phase 2 focuses on expanding the core API with advanced widgets, improved state management, and comprehensive testing to ensure Streamlit API compatibility.

## Completed in Phase 1

✅ **Core Types & Traits**
- Element system with 20+ types
- Widget system with type-safe values
- Session management
- Delta generation

✅ **Runtime API (St)**
- 25+ display and input methods
- State management
- Session handling

✅ **Testing**
- 24 unit tests (core, runtime, server)
- 22 integration tests (API compatibility)
- 100% test pass rate

## Phase 2 Objectives

### 1. Expand Element Types (20+ → 40+)

**New Display Elements:**
```rust
// Tabs
st.tabs(["Tab 1", "Tab 2", "Tab 3"])

// Expanders
st.expander("Click to expand")

// Sidebar
st.sidebar.write("Sidebar content")

// Columns with custom widths
st.columns([1, 2, 1])

// Metric display
st.metric("Revenue", "$1,000,000", "+10%")

// Table display
st.table(data)

// DataFrame
st.dataframe(df)
```

**New Input Widgets:**
```rust
// Date picker
st.date_input("Select date")

// Time picker
st.time_input("Select time")

// Color picker
st.color_picker("Pick a color")

// File uploader
st.file_uploader("Upload file")

// Camera input
st.camera_input("Take a photo")

// Radio button
st.radio("Choose one", ["A", "B", "C"])
```

### 2. Implement Sidebar Support

**Implementation Plan:**
1. Add `Sidebar` type to core
2. Create `st.sidebar` context
3. Implement sidebar rendering
4. Add sidebar state management

**Example:**
```rust
st.sidebar.title("Navigation")
st.sidebar.write("Sidebar content")
let choice = st.sidebar.selectbox("Choose", ["Home", "About", "Contact"])
```

### 3. Add Container Nesting

**Implementation Plan:**
1. Support nested containers
2. Implement column nesting
3. Add row support
4. Implement proper layout hierarchy

**Example:**
```rust
let cols = st.columns(2);
let col1_container = cols[0].st().container();
col1_container.st().write("Nested content");
```

### 4. Implement Tabs and Expanders

**Tabs:**
```rust
let tabs = st.tabs(["Tab 1", "Tab 2", "Tab 3"]);
tabs[0].st().write("Content for tab 1");
tabs[1].st().write("Content for tab 2");
```

**Expanders:**
```rust
let expander = st.expander("Click to expand");
expander.st().write("Hidden content");
```

### 5. Add Form Validation

**Implementation Plan:**
1. Create validation framework
2. Add field validators
3. Implement error messages
4. Add form submission handling

**Example:**
```rust
let email = st.text_input("Email", "");
if !email.contains("@") {
    st.error("Invalid email");
}
```

### 6. Widget Key-Based State Persistence

**Implementation Plan:**
1. Enhance key system
2. Implement state serialization
3. Add session persistence
4. Implement state recovery

**Example:**
```rust
// State persists across reruns with same key
let value = st.text_input("Name", "default", Some("name_key"));
```

### 7. Comprehensive Integration Tests

**Test Categories:**
1. Widget interaction tests
2. State persistence tests
3. Layout tests
4. Streamlit compatibility tests
5. Complex workflow tests

**Current Status:** 22 integration tests passing

### 8. Streamlit API Compatibility Testing

**Test Plan:**
1. Compare API signatures with Streamlit
2. Test behavior equivalence
3. Verify state management
4. Test widget interactions
5. Validate layout rendering

**Current Tests:**
- `test_streamlit_api_compatibility()` - Basic workflow
- `test_multiple_reruns_simulation()` - Rerun behavior
- `test_complex_app_workflow()` - Form submission
- `test_widget_state_persistence()` - State management

## Implementation Roadmap

### Week 1: Expand Element Types
- [ ] Add tabs element
- [ ] Add expander element
- [ ] Add metric element
- [ ] Add table element
- [ ] Add date/time pickers
- [ ] Add color picker
- [ ] Write tests for each

### Week 2: Sidebar & Layout
- [ ] Implement sidebar support
- [ ] Add column nesting
- [ ] Add row support
- [ ] Implement layout hierarchy
- [ ] Write layout tests

### Week 3: Advanced Widgets
- [ ] Add file uploader
- [ ] Add camera input
- [ ] Add radio button
- [ ] Add form validation
- [ ] Write widget tests

### Week 4: Testing & Polish
- [ ] Complete integration tests
- [ ] Add E2E test framework
- [ ] Performance testing
- [ ] Documentation updates
- [ ] Bug fixes

## Code Structure

### New Files to Create

```
crates/platypus-core/src/
├── sidebar.rs          # Sidebar implementation
├── tabs.rs             # Tabs implementation
├── expander.rs         # Expander implementation
├── form.rs             # Form validation

crates/platypus-runtime/src/
├── sidebar_context.rs  # Sidebar St context
├── form_context.rs     # Form context
├── validation.rs       # Validation framework

tests/
├── sidebar_tests.rs
├── tabs_tests.rs
├── form_tests.rs
├── layout_tests.rs
```

### Modified Files

```
crates/platypus-core/src/
├── element.rs          # Add new element types
├── lib.rs              # Export new types

crates/platypus-runtime/src/
├── context.rs          # Add new St methods
├── lib.rs              # Export new modules
```

## Testing Strategy

### Unit Tests
- Test each new element type
- Test state management
- Test validation logic

### Integration Tests
- Test widget interactions
- Test state persistence
- Test complex workflows
- Test Streamlit API compatibility

### E2E Tests
- Test browser interactions
- Test WebSocket communication
- Test UI rendering
- Test state synchronization

## Performance Considerations

1. **State Management**: Efficient delta generation
2. **Memory Usage**: Minimize widget state copies
3. **Network**: Compress messages for large states
4. **Rendering**: Optimize element updates

## Backward Compatibility

All Phase 2 changes maintain backward compatibility with Phase 1 API.

## Success Criteria

- [ ] 40+ element types implemented
- [ ] Sidebar support working
- [ ] Container nesting functional
- [ ] Tabs and expanders implemented
- [ ] Form validation working
- [ ] 50+ integration tests passing
- [ ] 100% test pass rate
- [ ] Streamlit API compatibility verified
- [ ] Performance benchmarks established
- [ ] Documentation complete

## Next Steps

1. Start with expanding element types
2. Implement sidebar support
3. Add advanced widgets
4. Write comprehensive tests
5. Verify Streamlit compatibility
6. Optimize performance
7. Update documentation

## References

- [ARCHITECTURE.md](ARCHITECTURE.md) - System design
- [QUICK_REFERENCE.md](QUICK_REFERENCE.md) - API reference
- [TODO.md](TODO.md) - Development roadmap
- [Streamlit API Docs](https://docs.streamlit.io)
