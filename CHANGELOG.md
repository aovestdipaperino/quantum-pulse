# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.12] - 2025-10-09

### Added
- **Stack-Based Pause/Unpause** - Fine-grained profiling control for timers on the call stack
  - `pause_stack!()` macro to pause only timers currently active on the call stack
  - `unpause_stack!()` macro to resume timers that were paused by `pause_stack!()`
  - Thread-local timer tracking with unique IDs for each timer instance
  - Thread-local pause set for tracking which timers are paused
  - Automatic cleanup of timer IDs from stack and pause set on drop
  - Full stub implementations for zero-cost abstractions in default mode

### Documentation
- Added comprehensive example `examples/stack_pause.rs` demonstrating:
  - Excluding I/O wait time from processing metrics
  - Pausing nested timers
  - Multiple pause/unpause cycles
  - Conditional profiling based on runtime conditions
- Added 6 integration tests covering all stack-based pause/unpause scenarios

### How It Works
- Each timer gets a unique ID and registers on a thread-local stack when created
- `pause_stack!()` marks all timer IDs currently on the stack as paused
- Paused timers don't record metrics when they drop
- `unpause_stack!()` removes currently active timers from the paused set
- Timers can be paused without being resumed (won't record)
- New timers created after `pause_stack!()` are not affected

### Use Cases
- Exclude I/O wait time from algorithm profiling
- Measure only CPU-bound work in mixed operations
- Exclude network latency from processing metrics
- Conditional profiling based on runtime conditions
- Fine-grained control without affecting concurrent operations

## [0.1.11] - 2025-10-08

### Fixed
- **Test Isolation** - Fixed race conditions in test suite
  - Added `.cargo/config.toml` with `RUST_TEST_THREADS = "1"` to force serial test execution
  - Tests now run reliably without flaky failures due to shared global state
- **ProfileCollector State Management** - Fixed incomplete cleanup in `reset_all()`
  - `ProfileCollector::reset_all()` now properly clears both `GLOBAL_STATS` and `GLOBAL_CATEGORIES`
  - Ensures complete state reset between test runs
- **ProfileOp Derive Macro** - Fixed category name generation for variants without attributes
  - Variants without `#[category]` attribute now correctly use their variant name as the category name
  - Previously, these variants incorrectly received empty string category names

## [0.1.8] - 2024-12-22

### Changed
- **Documentation Improvements** - Full compliance with Rust API guidelines
  - Removed global `#![allow(unused_must_use)]` lint suppression
  - Added `#[doc(inline)]` attributes to all internal module re-exports for better API documentation
  - Shortened all module-level documentation first sentences to under 15 words (M-FIRST-DOC-SENTENCE)
  - Made all documentation examples fully compilable by adding stub functions
  - Improved histogram precision documentation with detailed accuracy/memory tradeoffs

### Fixed
- Documentation examples now compile without `no_run` annotations
- Module summaries now follow the 15-word guideline for better readability

## [0.1.7] - 2024-09-09

### Added
- **Pause/Unpause Profiling** - New macros for dynamic profiling control
  - `pause!()` macro to globally pause all active profiling timers
  - `unpause!()` macro to globally resume all paused profiling timers
  - Global pause state management with thread-safe implementation
  - `ProfileCollector::pause()`, `ProfileCollector::unpause()`, and `ProfileCollector::is_paused()` methods
  - `ProfileCollector::reset_pause_state()` for test isolation
  - Complete stub implementations for zero-cost abstractions in default mode

### Added
- **Enhanced PausableTimer** - Full implementation for fine-grained timing control
  - `PausableTimer::new()` and `PausableTimer::new_paused()` constructors
  - `pause()` and `resume()` methods for manual timer control
  - `total_elapsed()`, `total_elapsed_micros()`, `total_elapsed_millis()` for elapsed time queries
  - `is_running()` status check
  - `stop()` and `stop_and_record()` for controlled timer termination
  - `reset()` and `reset_paused()` for timer reinitialization
  - Complete stub implementation for compatibility with default features

### Added
- **Network Time Exclusion Examples** - Comprehensive examples showing different approaches
  - Method 1: Using `pause!()`/`unpause!()` macros for simple exclusions
  - Method 2: Using `PausableTimer` for fine-grained control
  - Method 3: Separate profiling by category (recommended approach)
  - Method 4: Nested profiling with comprehensive categorization
  - Real-world payment processing example demonstrating all methods

### Documentation
- Updated README.md with pause/unpause functionality section
- Added advanced patterns section to USER-GUIDE.md with pause/unpause use cases
- Comprehensive examples showing network time exclusion techniques
- Added integration tests for pause/unpause functionality

### Fixed
- All compiler warnings eliminated across examples and library code
- Fixed unused async futures in examples by properly awaiting them
- Added appropriate `#[allow(dead_code)]` attributes for demonstration code
- Complete compatibility between stub mode and full feature implementations

## [0.1.6] - 2024-09-05

### Improved
- Replaced panic behavior when no category found with a better NotFound category applied
- Improved the alignment of some reports
- Use empty string to represent NoCategory so that output looks great even if the ProfileOp enum doesn't set categories

## [0.1.5] - 2024-09-01

### Added
- **ProfileOp Derive Macro** - New recommended way to use quantum-pulse with automatic trait implementation
  - Automatic `Operation` trait implementation via `#[derive(ProfileOp)]`
  - Built-in category management with `#[category(name = "...", description = "...")]` attributes
  - Support for all enum variant types (unit, tuple, and struct variants)
  - Intelligent description reuse - first description for a category name is used across all variants
  - Zero boilerplate code required
- New `quantum-pulse-macros` crate providing procedural macro support
- `macros` feature flag for using just the derive macros
- Comprehensive macro tests and examples
- `macro_derive.rs` example demonstrating the new derive macro usage

### Changed
- Updated documentation to emphasize `ProfileOp` as the recommended approach
- `full` feature now includes macro support by default
- Improved USER-GUIDE.md with ProfileOp-first examples
- Enhanced README.md with migration guide from manual implementation

### Improved
- Better compile-time safety with type-checked operation definitions
- Reduced boilerplate code for new users
- More maintainable profiling code with automatic categorization

## [0.1.4] - Previous Release

### Added
- HDR histogram support for accurate percentile calculations
- Async profiling support with `profile_async!` macro
- Pausable timers for excluding specific time periods
- Report builder with customizable output formats

### Changed
- Improved zero-cost abstractions in stub mode
- Better performance characteristics documentation

## [0.1.3] - Previous Release

### Added
- Custom category support with the `Category` trait
- Thread-safe operation recording
- CSV export functionality

### Fixed
- Memory leak in long-running applications
- Incorrect percentile calculations for small sample sizes

## [0.1.2] - Previous Release

### Added
- Basic profiling functionality with `profile!` macro
- Operation trait for type-safe profiling
- Stub and full implementations

### Changed
- Initial public release
