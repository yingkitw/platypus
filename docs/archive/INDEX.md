# platypus Documentation Index

## 📚 Documentation Guide

Start here to navigate the platypus project documentation.

### Quick Links

- **[README.md](README.md)** - Project overview and features
- **[GETTING_STARTED.md](GETTING_STARTED.md)** - Setup and installation guide
- **[QUICK_REFERENCE.md](QUICK_REFERENCE.md)** - API reference and common patterns
- **[ARCHITECTURE.md](ARCHITECTURE.md)** - Detailed system design
- **[MIGRATION_SUMMARY.md](MIGRATION_SUMMARY.md)** - Streamlit migration details
- **[TODO.md](TODO.md)** - Development roadmap
- **[LICENSE](LICENSE)** - Apache 2.0 license

## 🚀 Getting Started

### For New Users

1. Read [README.md](README.md) for project overview
2. Follow [GETTING_STARTED.md](GETTING_STARTED.md) for setup
3. Check [QUICK_REFERENCE.md](QUICK_REFERENCE.md) for API basics
4. Run your first app: `cargo run --bin platypus -- run app.rs`

### For Developers

1. Understand [ARCHITECTURE.md](ARCHITECTURE.md) for system design
2. Review [MIGRATION_SUMMARY.md](MIGRATION_SUMMARY.md) for implementation details
3. Check [TODO.md](TODO.md) for current tasks
4. Run tests: `cargo test`
5. Start contributing!

### For Architects

1. Study [ARCHITECTURE.md](ARCHITECTURE.md) for design patterns
2. Review [MIGRATION_SUMMARY.md](MIGRATION_SUMMARY.md) for tech stack
3. Understand trait-based design in crates
4. Plan extensions using [TODO.md](TODO.md)

## 📖 Documentation by Topic

### Project Overview
- [README.md](README.md) - What is platypus?
- [MIGRATION_SUMMARY.md](MIGRATION_SUMMARY.md) - Streamlit migration details
- [LICENSE](LICENSE) - Apache 2.0 license

### Getting Started
- [GETTING_STARTED.md](GETTING_STARTED.md) - Installation and setup
- [QUICK_REFERENCE.md](QUICK_REFERENCE.md) - Quick API reference
- [TODO.md](TODO.md) - Development roadmap

### Technical Details
- [ARCHITECTURE.md](ARCHITECTURE.md) - System architecture
- [MIGRATION_SUMMARY.md](MIGRATION_SUMMARY.md) - Implementation summary
- Crate documentation (via `cargo doc --open`)

### API Reference
- [QUICK_REFERENCE.md](QUICK_REFERENCE.md) - St API methods
- [ARCHITECTURE.md](ARCHITECTURE.md) - Message protocols
- Inline code documentation (via `cargo doc`)

## 🏗️ Project Structure

```
platypus/
├── crates/
│   ├── platypus-core/          # Core types and traits
│   ├── platypus-proto/         # Protocol Buffer definitions
│   ├── platypus-runtime/       # Runtime engine and St API
│   ├── platypus-server/        # Web server and handlers
│   └── platypus-cli/           # Command-line interface
├── proto/                   # Proto source files
├── Cargo.toml              # Workspace manifest
├── README.md               # Project overview
├── ARCHITECTURE.md         # System design
├── GETTING_STARTED.md      # Setup guide
├── QUICK_REFERENCE.md      # API reference
├── MIGRATION_SUMMARY.md    # Migration details
├── TODO.md                 # Roadmap
├── INDEX.md                # This file
└── LICENSE                 # Apache 2.0 license
```

## 🔧 Common Tasks

### Build and Test
```bash
cargo build              # Build all crates
cargo test              # Run all tests
cargo fmt               # Format code
cargo clippy            # Lint code
```

### Development
```bash
cargo run --bin platypus -- run app.rs    # Run app
RUST_LOG=debug cargo build             # Debug build
cargo doc --open                       # View docs
```

### Release
```bash
cargo build --release   # Build optimized binary
cargo test --release    # Test release build
```

## 📚 Crate Documentation

### platypus-core
**Purpose**: Core types and traits  
**Key Types**: `Element`, `Widget`, `Session`, `DeltaGenerator`  
**Documentation**: See [ARCHITECTURE.md](ARCHITECTURE.md) - Core Layer

### platypus-proto
**Purpose**: Protocol Buffer definitions  
**Key Messages**: `ForwardMsg`, `BackMsg`, `Element`, `Delta`  
**Documentation**: See [ARCHITECTURE.md](ARCHITECTURE.md) - Proto Layer

### platypus-runtime
**Purpose**: App runtime and state management  
**Key Types**: `St`, `SessionStore`, `Event`  
**Documentation**: See [QUICK_REFERENCE.md](QUICK_REFERENCE.md) - St API Reference

### platypus-server
**Purpose**: Web server with WebSocket support  
**Key Types**: `AppServer`, `ServerConfig`  
**Documentation**: See [ARCHITECTURE.md](ARCHITECTURE.md) - Server Layer

### platypus-cli
**Purpose**: Command-line interface  
**Commands**: `run`, `build`, `new`, `version`  
**Documentation**: See [GETTING_STARTED.md](GETTING_STARTED.md) - CLI Commands

## 🎯 Development Phases

### ✅ Phase 1: Foundation (Complete)
- Workspace setup
- Core types and traits
- Proto definitions
- Runtime engine
- Web server
- CLI tool

### 📋 Phase 2-7: Planned
See [TODO.md](TODO.md) for detailed roadmap

## 🔗 External Resources

- [Streamlit Documentation](https://docs.streamlit.io)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Documentation](https://tokio.rs)
- [Axum Web Framework](https://github.com/tokio-rs/axum)
- [Protocol Buffers](https://developers.google.com/protocol-buffers)

## ❓ FAQ

### Where do I start?
Read [README.md](README.md) first, then [GETTING_STARTED.md](GETTING_STARTED.md).

### How do I build an app?
Follow the Quick Start in [GETTING_STARTED.md](GETTING_STARTED.md).

### What's the API?
Check [QUICK_REFERENCE.md](QUICK_REFERENCE.md) for St API methods.

### How does it work?
Read [ARCHITECTURE.md](ARCHITECTURE.md) for system design.

### What's the roadmap?
See [TODO.md](TODO.md) for planned features.

### How do I contribute?
See [GETTING_STARTED.md](GETTING_STARTED.md) - Development Workflow.

## 📞 Support

- **Issues**: GitHub Issues
- **Discussions**: GitHub Discussions
- **Documentation**: See files in this directory

## 📝 License

platypus is licensed under Apache 2.0. See [LICENSE](LICENSE) for details.

---

**Last Updated**: 2025  
**Project Status**: Phase 1 Complete ✅  
**Maintainer**: yingkitw
