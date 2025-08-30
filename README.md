# Quantum Pulse

[![Crates.io](https://img.shields.io/crates/v/quantum-pulse.svg)](https://crates.io/crates/quantum-pulse)
[![Documentation](https://docs.rs/quantum-pulse/badge.svg)](https://docs.rs/quantum-pulse)
[![License](https://img.shields.io/crates/l/quantum-pulse.svg)](https://github.com/yourusername/quantum-pulse)

A lightweight, customizable profiling library for Rust applications with support for custom categories and percentile statistics.

## Features

- 🚀 **True Zero-Cost Abstraction** - Stub implementation compiles to nothing when disabled
- 📊 **Percentile Statistics** - Automatic calculation of p50, p95, p99, and p99.9 percentiles using HDR histograms
- 🏷️ **Type-Safe Categories** - Define your own operation categories with compile-time guarantees
- 📈 **Multiple Output Formats** - Console and CSV export options
- ⏸️ **Pausable Timers** - Exclude specific periods from measurements
- 🔧 **Clean API** - Same interface whether profiling is enabled or disabled
- 🌐 **Async Support** - Full support for async/await patterns
- 🎯 **No Conditionals Required** - Use the same code for both production and development

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
# For production builds (zero overhead)
quantum-pulse = { version = "0.1.0", default-features = false }

# For development builds (with profiling)
quantum-pulse = { version = "0.1.0", features = ["full"] }
```

Or use feature flags in your application:

```toml
[dependencies]
quantum-pulse = { version = "0.1.0", default-features = false }

[features]
profiling = ["quantum-pulse/full"]
```

## Quick Start

### Basic Usage

```rust
use quantum_pulse::{Profiler, Category, profile};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppOperation {
    DatabaseQuery,
    DataProcessing,
    NetworkRequest,
}

impl Category for AppOperation {
    fn description(&self) -> Option<&str> {
        match self {
            AppOperation::DatabaseQuery => Some("Database operations"),
            AppOperation::DataProcessing => Some("Data processing tasks"),
            AppOperation::NetworkRequest => Some("Network I/O operations"),
        }
    }
}

type AppProfiler = Profiler<AppOperation>;

fn main() {
    // Type-safe timing with enum-based macro
    let result = profile!(AppOperation::DatabaseQuery => {
        expensive_database_operation()
    });

    // Using the Profiler directly with categories
    let data = AppProfiler::time_with_category(
        &format!("{:?}", AppOperation::DataProcessing),
        AppOperation::DataProcessing,
        || process_large_dataset()
    );

    // Generate a report
    let report = AppProfiler::report();
    println!("{:#?}", report);
}
```

### Type-Safe Operations with Categories

Define specific operations with automatic categorization:

```rust
use quantum_pulse::{Profiler, Category, profile};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum WebServerOperation {
    AuthenticateUser,
    FetchUserData,
    UpdateCache,
    ProcessRequest,
    SerializeResponse,
}

impl Category for WebServerOperation {
    fn description(&self) -> Option<&str> {
        match self {
            WebServerOperation::AuthenticateUser => Some("User authentication"),
            WebServerOperation::FetchUserData => Some("Database queries"),
            WebServerOperation::UpdateCache => Some("Cache operations"),
            WebServerOperation::ProcessRequest => Some("Business logic"),
            WebServerOperation::SerializeResponse => Some("Response formatting"),
        }
    }

    fn priority(&self) -> i32 {
        match self {
            WebServerOperation::AuthenticateUser => 1,  // Critical path
            WebServerOperation::FetchUserData => 2,     // Important
            WebServerOperation::ProcessRequest => 2,
            WebServerOperation::UpdateCache => 3,       // Supporting
            WebServerOperation::SerializeResponse => 3,
        }
    }
}

type WebProfiler = Profiler<WebServerOperation>;

async fn handle_request(user_id: u64) -> Response {
    // Profile with enum-based operations
    let user = profile!(WebServerOperation::AuthenticateUser => {
        authenticate_user(user_id)
    });

    let data = profile!(WebServerOperation::FetchUserData => {
        database.get_user_data(user_id)
    });

    let response = profile!(WebServerOperation::SerializeResponse => {
        serialize_response(data)
    });

    // Generate categorized report
    let report = WebProfiler::report();
    println!("{:#?}", report);
    
    response
}
```

### Async Support

```rust
use quantum_pulse::{Profiler, Category, profile};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AsyncOperation {
    HttpRequest,
    DatabaseQuery,
    ProcessData,
}

impl Category for AsyncOperation {
    fn description(&self) -> Option<&str> {
        match self {
            AsyncOperation::HttpRequest => Some("HTTP API calls"),
            AsyncOperation::DatabaseQuery => Some("Database operations"),
            AsyncOperation::ProcessData => Some("Data processing"),
        }
    }
}

async fn fetch_data() -> Result<Data, Error> {
    // Profile async operations with enums
    profile!(AsyncOperation::HttpRequest => async {
        client.get("https://api.example.com/data").await
    })
}

async fn main() {
    let data = profile!(AsyncOperation::ProcessData => async {
        fetch_data().await
    });
}
```

### Manual Recording

For precise control over timing measurements:

```rust
use quantum_pulse::{Profiler, Category};
use std::time::Instant;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ProcessingOperation {
    BatchProcessing,
    FileIO,
}

impl Category for ProcessingOperation {
    fn description(&self) -> Option<&str> {
        match self {
            ProcessingOperation::BatchProcessing => Some("Batch data processing"),
            ProcessingOperation::FileIO => Some("File I/O operations"),
        }
    }
}

fn process_with_io() {
    let start = Instant::now();
    
    // Do some processing
    process_part_1();
    
    // Exclude I/O wait time from measurement
    let processing_time = start.elapsed();
    let data = read_from_disk(); // Not measured
    
    let start2 = Instant::now();
    process_part_2(data);
    let processing_time2 = start2.elapsed();
    
    // Manually record total processing time
    Profiler::<ProcessingOperation>::record_with_category(
        &format!("{:?}", ProcessingOperation::BatchProcessing),
        ProcessingOperation::BatchProcessing,
        (processing_time + processing_time2).as_micros() as u64
    );
}
```

### Type-Safe Operation Tracking

Define your operations as Debug-derived enums for compile-time safety:

```rust
use quantum_pulse::{Profiler, Category, profile};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum UserOperation {
    FetchUser,
    SaveUser,
    QueryCache,
}

impl Category for UserOperation {
    fn description(&self) -> Option<&str> {
        match self {
            UserOperation::FetchUser => Some("Fetch user from database"),
            UserOperation::SaveUser => Some("Save user to database"),
            UserOperation::QueryCache => Some("Query cache for user data"),
        }
    }

    fn priority(&self) -> i32 {
        match self {
            UserOperation::FetchUser => 1,
            UserOperation::SaveUser => 2,
            UserOperation::QueryCache => 1,
        }
    }
}

// Use with type-safe enum operations
fn fetch_user_data(user_id: u64) -> User {
    profile!(UserOperation::FetchUser => {
        fetch_user_from_db(user_id)
    })
}
```

## Advanced Features

### Report Configuration

Customize report generation with various options:

```rust
use quantum_pulse::{ReportBuilder, TimeFormat, Category};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppOperation {
    DatabaseQuery,
    ApiCall,
    CacheOperation,
}

impl Category for AppOperation {
    fn description(&self) -> Option<&str> {
        match self {
            AppOperation::DatabaseQuery => Some("Database operations"),
            AppOperation::ApiCall => Some("External API calls"),
            AppOperation::CacheOperation => Some("Cache operations"),
        }
    }
}

let report = ReportBuilder::<AppOperation>::new()
    .include_percentiles(true)
    .group_by_category(true)
    .time_format(TimeFormat::Milliseconds)
    .build();

println!("{:#?}", report);
```

### Export Formats

```rust
use quantum_pulse::Profiler;

type AppProfiler = Profiler<AppOperation>;

// Console output (default)
let report = AppProfiler::report();
println!("{:#?}", report);

// Simple CSV-like export
let stats = AppProfiler::get_all_stats();
let mut csv_content = String::from("Operation,Count,Mean(μs),Min(μs),Max(μs)\n");
for (name, stat) in stats {
    csv_content.push_str(&format!(
        "{},{},{:.1},{},{}\n",
        name, stat.count, stat.mean_micros, stat.min_micros, stat.max_micros
    ));
}
std::fs::write("profile_report.csv", csv_content).unwrap();
```

### Operation Categories

Define operation categories with metadata for better organization:

```rust
use quantum_pulse::{Category, profile};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum CriticalOperation {
    ApiRequest,
    DatabaseTransaction,
    UserAuthentication,
}

impl Category for CriticalOperation {
    fn description(&self) -> Option<&str> {
        match self {
            CriticalOperation::ApiRequest => Some("External API call duration"),
            CriticalOperation::DatabaseTransaction => Some("Database transaction processing"),
            CriticalOperation::UserAuthentication => Some("User login and token validation"),
        }
    }

    fn priority(&self) -> i32 {
        match self {
            CriticalOperation::UserAuthentication => 1, // Highest priority
            CriticalOperation::DatabaseTransaction => 2,
            CriticalOperation::ApiRequest => 3,
        }
    }
}

// Use type-safe operations for consistent profiling
let result = profile!(CriticalOperation::ApiRequest => {
    make_api_call()
});
```

## Zero-Cost Abstractions

Quantum Pulse implements true zero-cost abstractions through its innovative stub feature system:

### How It Works

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppOperation {
    DatabaseQuery,
}

impl Category for AppOperation {}

// Your code always looks the same
let result = profile!(AppOperation::DatabaseQuery => {
    expensive_operation()
});

// With default features (stub mode):
// - profile! macro expands to just the code block
// - No timing, no allocations, no overhead
// - Compiler optimizes it to: let result = expensive_operation();

// With "full" feature enabled:
// - Full profiling with timing and statistics
// - HDR histograms for accurate percentiles
// - Comprehensive reporting
```

### Performance Characteristics

| Configuration | Overhead | Use Case |
|--------------|----------|----------|
| Stub (default) | **Zero** - methods are empty and inlined away | Production |
| Full | ~200-300ns per operation | Development, debugging |

### Implementation Details

The library provides two implementations:
- **Stub**: Empty trait implementations that compile to nothing
- **Full**: Complete profiling implementation with HDR histograms

Both expose the exact same API, ensuring your code never needs conditional compilation.

## Performance Considerations

The library is designed with performance in mind:

- **True Zero-Cost**: Stub implementations are completely removed by the compiler
- **Efficient Percentiles**: Using HDR histograms for O(1) percentile calculations
- **Lock-Free Operations**: Using atomic operations and thread-local storage
- **Smart Inlining**: Critical paths marked with `#[inline(always)]` in stub mode
- **No Runtime Checks**: Feature selection happens at compile time

## Feature Flags

- `full`: Enable full profiling functionality with HDR histograms
- Default (no features): Stub implementation with zero overhead

When no features are enabled, all profiling operations compile to no-ops that are completely eliminated by the optimizer.

## Examples

Check out the `examples/` directory for more comprehensive examples:

- `basic.rs` - Simple enum-based profiling example
- `custom_categories.rs` - Using custom operation categories
- `async_profiling.rs` - Profiling async code with enums
- `trading_system.rs` - High-frequency trading system example

Run examples with:

```bash
cargo run --example basic
cargo run --example custom_categories
cargo run --example async_profiling
cargo run --example trading_system
```

## Benchmarks

Run benchmarks with:

```bash
cargo bench
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

## Migration from Conditional Compilation

If you're currently using `#[cfg(feature = "...")]` for profiling, Quantum Pulse eliminates that need:

### Before (with conditionals and strings)
```rust
#[cfg(feature = "profiling")]
let timer = Timer::start("operation");

let result = do_work();

#[cfg(feature = "profiling")]
timer.stop();
```

### After (with Quantum Pulse and type-safe enums)
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppOperation {
    WorkOperation,
}

impl Category for AppOperation {}

let result = profile!(AppOperation::WorkOperation => {
    do_work()
});
```

The same clean code works in both production (zero-cost) and development (full profiling).

## Acknowledgments

This library was designed for high-performance applications requiring microsecond-precision profiling, where traditional sampling profilers lack the necessary granularity and CPU performance counters provide excessive detail.