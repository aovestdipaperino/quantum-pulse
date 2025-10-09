# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Quantum Pulse is a lightweight profiling library for Rust with a zero-cost abstraction design. The library has two modes:
- **Stub mode (default)**: All profiling code compiles away completely (zero overhead)
- **Full mode** (`--features full`): Complete profiling with HDR histograms and statistics

## Build and Test Commands

```bash
# Run tests (stub mode - zero cost)
cargo test

# Run tests with full profiling features
cargo test --all-features

# Run tests for specific feature combinations
cargo test --features full

# Lint the code
cargo clippy --all-features -- -D warnings

# Format code
cargo fmt --all

# Build documentation
cargo doc --all-features --no-deps

# Run examples (must use --features full)
cargo run --example basic --features full
cargo run --example macro_derive --features full
cargo run --example async_profiling --features full
cargo run --example trading_system --features full

# Run benchmarks
cargo bench
```

## Workspace Structure

This is a Cargo workspace with two crates:

1. **quantum-pulse** (main crate): The core profiling library
2. **quantum-pulse-macros**: Procedural macros, specifically the `ProfileOp` derive macro

### Important: Macro Versioning
The macros crate version MUST stay in sync with the main crate. When updating versions, update both:
- `quantum-pulse/Cargo.toml`
- `quantum-pulse-macros/Cargo.toml`
- The dependency reference in the main Cargo.toml

## Architecture

### Core Concepts

The library is built around three main traits:

1. **Category** (`src/category.rs`): Represents a grouping of related operations
2. **Operation** (`src/operation.rs`): Represents a single profiling point
3. **ProfileCollector** (`src/collector.rs`): Thread-safe centralized storage for metrics

### Dual Implementation Strategy

The library uses feature flags to provide two complete implementations:

```rust
#[cfg(feature = "full")]      // Full implementation in separate modules
#[cfg(not(feature = "full"))] // Stub implementation inline in lib.rs
```

**Key architectural principle**: Both implementations expose identical APIs. This allows users to write code once that works in both modes without conditional compilation.

### Module Organization

- `lib.rs`: Contains stub implementations and re-exports
- `category.rs`: Category trait and implementations (full mode only)
- `collector.rs`: Thread-safe metrics storage using `LazyLock<RwLock<HashMap>>`
- `operation.rs`: Operation trait and `SimpleOperation` helper
- `timer.rs`: RAII-based timers (`ProfileTimer`, `PausableTimer`)
- `reporter.rs`: Report generation and formatting
- `metrics.rs`: (exists but not in use currently)

### Macro Implementation

The `ProfileOp` derive macro (`quantum-pulse-macros/src/lib.rs`) generates `Operation` trait implementations from enums:

```rust
#[derive(ProfileOp)]
enum MyOp {
    #[category(name = "Database", description = "DB operations")]
    Query,
    #[category(name = "Database")]  // Reuses description
    Insert,
}
```

The macro intelligently manages categories by deduplicating category definitions within an enum.

## Feature Flags

- `full`: Enables complete profiling functionality (includes `hdrhistogram` dependency)
- `macros`: Currently unused, macros are always available
- `default`: No features, stub implementation only

## Testing Strategy

### Test Organization

- `tests/macro_test.rs`: Tests for the `ProfileOp` derive macro
- `tests/pause_unpause_integration.rs`: Tests for pause/unpause functionality
- Unit tests in source files

### Running Tests

Tests must be run with `--all-features` to test the full implementation:
```bash
cargo test --all-features
```

Running without features will test stub implementations.

## Publishing Process

**CRITICAL**: The macros crate MUST be published before the main crate.

1. Publish `quantum-pulse-macros` first
2. Wait for crates.io indexing (1-2 minutes)
3. Publish `quantum-pulse`

See `RELEASE_CHECKLIST.md` for detailed steps.

## Development Patterns

### When Adding New Features

1. Implement in the full mode modules first
2. Add corresponding stub implementations in `lib.rs`
3. Ensure APIs are identical between both modes
4. Test with both `cargo test` and `cargo test --all-features`

### When Modifying Categories or Operations

- Remember that category deduplication happens per-enum in the macro
- Test with multiple enums using the same category names
- Verify complex enum variants (tuple, struct) are handled

### Thread Safety

The collector uses:
- `LazyLock` for one-time initialization
- `RwLock` for concurrent access to shared state
- Thread-local storage for pause/unpause state

### HDR Histograms

In full mode, the library uses `hdrhistogram` with precision=3 for microsecond accuracy up to ~2.1 seconds with 0.1% relative error. This gives reasonable memory usage (~2KB per histogram).

## Common Gotchas

1. **Examples must use `--features full`**: The stub mode compiles away all profiling, so examples show nothing without the feature flag
2. **Macro and main crate versions must match**: They are tightly coupled
3. **Tests may have flaky behavior**: Some tests share global state in the collector
4. **Async support**: Use `profile_async!` macro, not `profile!`, for async code
5. **Timer recording**: Timers record on `Drop`, so be mindful of scope
