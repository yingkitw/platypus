# Codebase Cleanup Plan

## Objective
Tidy up the codebase following DRY, KISS, capability-facing design, and test-friendly patterns.

## Phase 1: Documentation Consolidation

### Root Level Cleanup
**Files to Archive** (43 files):
- All Phase/Session/Status reports → `docs/archive/`
- All Migration/Completion reports → `docs/archive/`
- All Progress/Guide files → `docs/archive/`

**Keep in Root**:
- `README.md` - Main project documentation
- `TODO.md` - Active tasks
- `ARCHITECTURE.md` - System design
- `LICENSE` - License file
- `Cargo.toml` - Project manifest
- `.gitignore` - Git configuration

### Documentation Structure
```
docs/
├── index.html              # Landing page
├── TESTING_OVERVIEW.md     # Test documentation
├── STREAMLIT_COMPATIBILITY_TESTS.md
├── MIGRATION_CHECKLIST.md
├── TEST_SUITE_INDEX.md
├── archive/                # Historical documentation
│   ├── phase-reports/
│   ├── migration-reports/
│   └── session-summaries/
└── examples/               # Example applications
```

## Phase 2: Code Organization

### Current Issues
1. **Duplication**: Similar test patterns repeated across test files
2. **Module Organization**: Some modules could be better organized
3. **Trait Usage**: Opportunity to use traits for capability-facing design
4. **Test Helpers**: Common test utilities not extracted

### Improvements

#### A. Extract Common Test Utilities
Create `crates/platypus-runtime/tests/common/mod.rs`:
- Common test fixtures
- Helper functions
- Mock builders

#### B. Capability-Facing Traits
- `Cacheable` - For cacheable operations
- `Navigable` - For navigation support
- `Secretable` - For secrets management
- `Componentizable` - For custom components

#### C. Reduce Code Duplication
- Extract common test patterns
- Create test builders
- Use macros for repetitive tests

## Phase 3: Test Organization

### Current Test Files
- `element_integration_tests.rs` (24 tests)
- `integration_tests.rs` (41 tests)
- `streamlit_compatibility_tests.rs` (39 tests)
- `chart_tests.rs` (16 tests)
- `phase3_features_tests.rs` (27 tests)
- `phase5_features_tests.rs` (31 tests)
- `final_features_tests.rs` (32 tests)

### Reorganization
```
tests/
├── common/
│   ├── mod.rs              # Shared utilities
│   ├── fixtures.rs         # Test fixtures
│   └── builders.rs         # Test builders
├── core/
│   ├── display_tests.rs    # Display element tests
│   ├── input_tests.rs      # Input widget tests
│   ├── feedback_tests.rs   # Feedback element tests
│   └── layout_tests.rs     # Layout component tests
├── advanced/
│   ├── charts_tests.rs     # Chart tests
│   ├── caching_tests.rs    # Caching tests
│   ├── navigation_tests.rs # Multi-page tests
│   ├── components_tests.rs # Component tests
│   └── secrets_tests.rs    # Secrets tests
├── compatibility/
│   └── streamlit_tests.rs  # Streamlit compatibility
└── integration/
    └── workflows_tests.rs  # Complex workflows
```

## Phase 4: Module Improvements

### Capability-Facing Design

#### Cache Module
```rust
pub trait Cacheable {
    fn cache_key(&self) -> String;
    fn cache_ttl(&self) -> Option<Duration>;
}

pub trait CacheProvider {
    fn get<T: Cacheable>(&self, key: &str) -> Option<T>;
    fn set<T: Cacheable>(&self, key: String, value: T);
}
```

#### Navigation Module
```rust
pub trait Navigable {
    fn pages(&self) -> Vec<Page>;
    fn current_page(&self) -> Option<Page>;
    fn navigate(&mut self, page: &str) -> Result<()>;
}
```

#### Components Module
```rust
pub trait Component {
    fn metadata(&self) -> &ComponentMetadata;
    fn validate(&self) -> Result<()>;
    fn render(&self, props: &ComponentProps) -> Result<String>;
}
```

## Phase 5: Cleanup Actions

### Step 1: Archive Documentation
- Create `docs/archive/` directory
- Move 43 old documentation files
- Update `.gitignore` if needed

### Step 2: Reorganize Tests
- Create test module structure
- Extract common utilities
- Consolidate similar tests

### Step 3: Refactor Modules
- Add capability traits
- Reduce duplication
- Improve organization

### Step 4: Update Documentation
- Keep only essential docs in root
- Update `README.md` with new structure
- Create `docs/index.html` landing page

### Step 5: Verify
- Run full test suite
- Check code coverage
- Verify build succeeds

## Expected Outcomes

### Before
- 43 extra documentation files in root
- Scattered test files
- Duplicated test code
- 306+ tests

### After
- Clean root directory (6 files)
- Organized test structure
- Reusable test utilities
- 306+ tests (same coverage, better organization)
- Capability-facing design
- Improved maintainability

## Timeline
- Phase 1: 5 minutes (archive docs)
- Phase 2: 10 minutes (code organization)
- Phase 3: 15 minutes (test reorganization)
- Phase 4: 10 minutes (module improvements)
- Phase 5: 5 minutes (verification)

**Total: ~45 minutes**

## Success Criteria
✅ Root directory has only essential files
✅ Documentation is organized
✅ Tests are well-organized
✅ Code follows DRY principle
✅ Capability-facing design implemented
✅ All 306+ tests still pass
✅ Build succeeds
