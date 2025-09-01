# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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