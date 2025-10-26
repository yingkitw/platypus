# Webag Architecture

## Overview

Webag is a Rust-based web app generator that maintains API compatibility with Streamlit while providing superior performance. The architecture is modular, trait-based, and designed for extensibility.

## Core Design Principles

1. **Trait-Based Design**: Use traits for capabilities and extensibility
2. **Separation of Concerns**: Clear boundaries between runtime, server, and API layers
3. **Performance First**: Async/await throughout, minimal allocations
4. **API Compatibility**: Maintain Streamlit API surface where possible
5. **Test-Friendly**: Traits enable easy mocking and testing

## Architecture Layers

### 1. Core Layer (`webag-core`)

Defines fundamental types and traits:

- **Element**: Base trait for all UI elements
- **Widget**: Input widgets with state
- **Container**: Layout containers (columns, tabs, etc.)
- **Delta**: Incremental updates to the UI
- **Session**: User session state
- **AppState**: Global application state

Key types:
- `ElementType`: Enum of all supported elements
- `WidgetValue`: Type-safe widget state
- `DeltaGenerator`: Builds UI incrementally

### 2. Proto Layer (`webag-proto`)

Protocol Buffer definitions for client-server communication:

- **ForwardMsg**: Server → Browser (elements, state updates)
- **BackMsg**: Browser → Server (user interactions)
- **Element**: 60+ UI component types
- **Block**: Layout containers
- **Delta**: Incremental UI updates

Proto files are compiled to Rust code using `prost`.

### 3. Runtime Layer (`webag-runtime`)

Manages app execution and state:

- **ScriptRunner**: Executes user scripts/functions
- **StateManager**: Manages session and widget state
- **DeltaGenerator**: Generates UI deltas
- **SessionStore**: Stores active sessions
- **EventHandler**: Processes user interactions

Key responsibilities:
- Script execution in isolated contexts
- State persistence across reruns
- Delta generation for efficient updates
- Event routing and handling

### 4. Server Layer (`webag-server`)

HTTP and WebSocket server:

- **AppServer**: Main server implementation
- **WebSocketHandler**: Manages WebSocket connections
- **MessageRouter**: Routes messages to handlers
- **SessionManager**: Manages client sessions
- **FileServer**: Serves static assets

Endpoints:
- `GET /`: Main app page
- `WS /ws`: WebSocket for real-time communication
- `GET /assets/*`: Static assets (CSS, JS, etc.)
- `POST /api/*`: API endpoints

### 5. CLI Layer (`webag-cli`)

Command-line interface:

- `webag run <script>`: Run an app
- `webag build <script>`: Build for production
- `webag new <name>`: Create new project
- `webag dev`: Development server with hot reload

## Data Flow

### App Initialization

```
User starts app
    ↓
CLI parses arguments
    ↓
Server starts (Axum)
    ↓
Browser connects via WebSocket
    ↓
Server sends NewSession message
    ↓
Script execution begins
    ↓
Elements added to DeltaGenerator
    ↓
Deltas sent to browser via ForwardMsg
    ↓
Browser renders UI
```

### User Interaction

```
User interacts with widget
    ↓
Browser sends BackMsg (user input)
    ↓
Server receives via WebSocket
    ↓
StateManager updates widget state
    ↓
Script reruns
    ↓
New deltas generated
    ↓
ForwardMsg sent to browser
    ↓
Browser updates UI
```

## State Management

### Session State

Each user session has:
- `session_id`: Unique identifier
- `script_hash`: Hash of current script
- `widget_states`: Map of widget ID → value
- `reruns`: Count of script reruns
- `metadata`: Session metadata

### Widget State

Widgets maintain:
- `widget_id`: Unique identifier
- `value`: Current value (type-safe via `WidgetValue`)
- `changed`: Whether value changed this run
- `metadata`: Widget-specific data

### Delta Generation

Deltas represent incremental UI changes:
- `add_element`: Add new element
- `update_element`: Update existing element
- `remove_element`: Remove element
- `clear_container`: Clear container contents

## Message Protocol

### ForwardMsg (Server → Browser)

```
ForwardMsg {
    hash: String,                    // For caching
    metadata: ForwardMsgMetadata,
    type: OneOf {
        new_session: NewSession,     // Initial session
        delta: Delta,                // UI updates
        script_finished: Status,     // Execution status
        session_status: Status,      // Session status
        ...
    }
}
```

### BackMsg (Browser → Server)

```
BackMsg {
    session_id: String,
    type: OneOf {
        widget_state_change: WidgetStateChange,  // User input
        rerun_script: RerunScript,               // Rerun request
        ...
    }
}
```

## Extensibility

### Adding New Elements

1. Define proto message in `webag-proto/proto/`
2. Add to `Element` oneof in `Element.proto`
3. Implement `Element` trait in `webag-core`
4. Add rendering logic in frontend
5. Add builder method to `St` API

### Adding New Widgets

1. Define proto message
2. Implement `Widget` trait
3. Add state handling in `StateManager`
4. Implement frontend component
5. Add API method to `St`

### Custom Components

Users can create custom components by:
1. Implementing `Element` trait
2. Providing frontend React component
3. Registering with component registry

## Performance Considerations

1. **Proto Caching**: ForwardMsg caching reduces bandwidth
2. **Delta Compression**: Only send changed elements
3. **Async I/O**: All I/O is async via Tokio
4. **Connection Pooling**: Reuse database connections
5. **Memory Efficiency**: Use `Arc` for shared state

## Testing Strategy

### Unit Tests
- Core types and traits
- State management
- Delta generation
- Message serialization

### Integration Tests
- Full app execution
- State persistence
- Message routing
- Session management

### E2E Tests
- Browser interaction
- Widget state changes
- Multi-page navigation
- File uploads

## Implementation Status

### Phase 1: Foundation ✅ COMPLETE
- [x] Core types and traits implemented
- [x] Proto definitions created
- [x] Runtime engine with St API
- [x] Web server with WebSocket
- [x] CLI tool
- [x] 24 unit tests (all passing)
- [x] Comprehensive documentation

### Phase 2: Core API Enhancement ✅ COMPLETE (95%)
- [x] Expand element types (20+ → 30+)
- [x] Add advanced widgets (date/time, file upload, color picker, radio, camera)
- [x] Implement sidebar support
- [x] Add container nesting
- [x] Implement tabs and expanders
- [x] Add form validation
- [x] Implement widget key-based state persistence
- [x] Add comprehensive integration tests (41 tests)
- [x] Test Streamlit API compatibility
- [x] Add table display
- [x] Add dataframe support
- [ ] Add chart support (Plotly, Vega-Lite)

### Phase 3-7: Planned
See TODO.md for detailed roadmap

## Future Enhancements

1. **Caching Layer**: @st.cache_data, @st.cache_resource decorators
2. **Multi-Page Apps**: Support for multi-page applications
3. **Custom Components**: Third-party component framework
4. **Data Visualization**: Plotly, Vega-Lite, Bokeh integration
5. **DataFrame Support**: Arrow tables and pandas compatibility
6. **File Handling**: Upload/download support
7. **Authentication**: User management and permissions
8. **Clustering**: Multi-instance deployment
9. **Analytics**: Built-in usage tracking
10. **Hot Reload**: Development server with live reload
