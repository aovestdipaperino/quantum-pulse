# Quantum Pulse - Crate Summary

## Overview

Quantum Pulse is a high-performance, customizable profiling library designed for microsecond-precision timing measurements. It provides detailed timing metrics with percentile statistics, custom categories, and multiple output formats.

## Key Features

### Core Capabilities
- **Zero-Cost Abstraction**: Completely removable at compile time via feature flags
- **High-Resolution Timing**: Microsecond precision with HDR histogram-based percentile calculations
- **Custom Categories**: Define your own operation categories for better organization
- **Multiple Output Formats**: Console, JSON, and CSV export options
- **Async Support**: Full support for profiling async/await code
- **Pausable Timers**: Exclude specific periods from measurements (e.g., I/O wait times)

### Statistics Provided
- Count of operations
- Total time spent
- Mean, min, and max times
- Percentiles: P50 (median), P95, P99, P99.9
- Standard deviation
- Per-category aggregation

## Architecture

### Module Structure
```
quantum-pulse/
├── src/
│   ├── lib.rs          # Main API and macros
│   ├── category.rs     # Category trait and implementations
│   ├── collector.rs    # Central metrics collection with thread-safe histograms
│   ├── timer.rs        # RAII-based timing primitives
│   ├── metrics.rs      # Metric definition and registry system
│   └── reporter.rs     # Report generation and formatting
├── examples/
│   ├── basic.rs        # Simple usage examples
│   ├── custom_categories.rs  # Category customization
│   └── async_profiling.rs    # Async operation profiling
```

### Key Components

1. **Profiler**: Main interface for timing operations
2. **ProfileTimer**: RAII timer that records on drop
3. **PausableTimer**: Timer that can be paused/resumed
4. **ProfileCollector**: Thread-safe global metrics storage
5. **ProfileReport**: Report generation with various formats
6. **Category System**: Extensible categorization framework

## Usage Examples

### Basic Profiling
```rust
use quantum_pulse::{Profiler, profile, DefaultCategory};

// Simple function profiling
let result = Profiler::<DefaultCategory>::time("operation", || {
    expensive_computation()
});

// Using macros
let data = profile!("fetch_data" => {
    fetch_from_database()
});
```

### Custom Categories
```rust
use quantum_pulse::{Category, Profiler};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum MyCategory {
    Database,
    Network,
    Compute,
}

impl Category for MyCategory {
    fn name(&self) -> &str {
        match self {
            MyCategory::Database => "Database Operations",
            MyCategory::Network => "Network I/O",
            MyCategory::Compute => "Computation",
        }
    }
}

// Use with custom category
Profiler::<MyCategory>::time_with_category(
    "query",
    MyCategory::Database,
    || run_query()
);
```

### Report Generation
```rust
use quantum_pulse::{ReportBuilder, TimeFormat, Percentile};

let report = ReportBuilder::<MyCategory>::new()
    .include_percentiles(true)
    .sort_by_percentile(Percentile::P99)
    .group_by_category(true)
    .time_format(TimeFormat::Milliseconds)
    .build();

// Export formats
println!("{}", report);                    // Console
let json = report.to_json().unwrap();      // JSON (requires "json" feature)
let csv = report.to_csv();                 // CSV
```

## Design Philosophy

### Key Changes Made

1. **Generic and Domain-Agnostic**
   - No assumptions about application domain
   - Fully customizable operation categories
   - Flexible metric categories

2. **Added Extensibility**
   - Generic `Category` trait for custom categorization
   - Configurable metric definitions
   - Pluggable report formats

3. **API Improvements**
   - Builder patterns for configuration
   - Type-safe category system
   - Better async support

### Integration Example

To use quantum-pulse in your application, add the dependency:

```toml
[dependencies]
quantum-pulse = { version = "0.1", features = ["full"] }
```

Then define application-specific categories:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppCategory {
    Database,
    Cache,
    Network,
    Computation,
    FileIO,
}

impl Category for AppCategory {
    fn description(&self) -> Option<&str> {
        match self {
            AppCategory::Database => Some("Database Operations"),
            AppCategory::Cache => Some("Cache Access"),
            AppCategory::Network => Some("Network I/O"),
            AppCategory::Computation => Some("CPU Intensive Tasks"),
            AppCategory::FileIO => Some("File System Operations"),
        }
    }
}

type MyProfiler = Profiler<AppCategory>;
```

## Performance Characteristics

- **Minimal Overhead**: ~100-200ns per timing operation
- **Memory Efficient**: HDR histograms use logarithmic memory scaling
- **Lock-Free Reads**: Most common operations avoid locks
- **Compile-Time Removal**: Zero overhead when disabled via features

## Feature Flags

- `enabled` (default): Enable profiling functionality
- `json`: Add JSON serialization support
- `chrono`: Enhanced timestamp formatting

## Testing

The crate includes comprehensive tests:
- Unit tests for all components
- Integration tests via examples
- Thread-safety tests for concurrent access

Run tests with:
```bash
cargo test --lib -- --test-threads=1  # Single-threaded for deterministic results
cargo test --examples                 # Test all examples
```

## Future Enhancements

Potential improvements for future versions:

1. **Distributed Tracing**: OpenTelemetry integration
2. **Real-time Monitoring**: Live metrics dashboard
3. **Memory Profiling**: Track allocations alongside timing
4. **Flame Graphs**: Visual performance analysis
5. **Benchmark Integration**: Criterion.rs compatibility
6. **Configuration Files**: YAML/TOML-based setup

## License

Licensed under either MIT or Apache 2.0 at your option.

## Credits

Designed for high-performance applications requiring microsecond-precision profiling.