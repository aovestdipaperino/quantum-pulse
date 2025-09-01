# Quantum Pulse

<div align="center">
  <img src="https://raw.githubusercontent.com/aovestdipaperino/quantum-pulse/main/logo.png" alt="Quantum Pulse Logo" width="600">
</div>

<br>

[![Crates.io](https://img.shields.io/crates/v/quantum-pulse.svg)](https://crates.io/crates/quantum-pulse)
[![Documentation](https://docs.rs/quantum-pulse/badge.svg)](https://docs.rs/quantum-pulse)
[![License](https://img.shields.io/crates/l/quantum-pulse.svg)](https://github.com/aovestdipaperino/quantum-pulse)

A lightweight, customizable profiling library for Rust applications with support for custom categories and percentile statistics.

## Features

- 🚀 **True Zero-Cost Abstraction** - Stub implementation compiles to nothing when disabled
- 🎯 **Derive Macro Support** - Automatic implementation with `#[derive(ProfileOp)]`
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
quantum-pulse = { version = "0.1.5", default-features = false }

# For development builds (with profiling and macros)
quantum-pulse = { version = "0.1.5", features = ["full"] }
```

Or use feature flags in your application:

```toml
[dependencies]
quantum-pulse = { version = "0.1.5", default-features = false }

[features]
profiling = ["quantum-pulse/full"]
```

## Quick Start

### 🎯 Recommended: Using the Derive Macro

The easiest and most maintainable way to use quantum-pulse is with the `ProfileOp` derive macro:

```rust
use quantum_pulse::{ProfileOp, profile, ProfileCollector};

// Simply derive ProfileOp and add category attributes
#[derive(Debug, ProfileOp)]
enum AppOperation {
    #[category(name = "Database", description = "Database operations")]
    QueryUser,
    
    #[category(name = "Database")]  // Reuses the description
    UpdateUser,
    
    #[category(name = "Network", description = "External API calls")]
    HttpRequest,
    
    #[category(name = "Cache", description = "Cache operations")]
    ReadCache,
    
    ComputeHash,  // No category attribute - uses variant name as category
}

fn main() {
    // Profile operations with zero boilerplate
    let user = profile!(AppOperation::QueryUser, {
        fetch_user_from_database()
    });

    let result = profile!(AppOperation::HttpRequest, {
        call_external_api()
    });

    // Generate and display report
    let report = ProfileCollector::get_summary();
    println!("Total operations: {}", report.total_operations);
}
```

### Category Management

The `ProfileOp` macro intelligently manages categories:

```rust
#[derive(Debug, ProfileOp)]
enum DatabaseOps {
    #[category(name = "Query", description = "Read operations")]
    SelectUsers,
    
    #[category(name = "Query")]  // Automatically reuses "Read operations" description
    SelectPosts,
    
    #[category(name = "Mutation", description = "Write operations")]
    InsertUser,
    
    #[category(name = "Mutation")]  // Automatically reuses "Write operations" description
    UpdateUser,
    
    DeleteUser,  // Uses "DeleteUser" as both name and description
}
```

### Alternative: Manual Implementation

For advanced use cases or when you prefer explicit control:

```rust
use quantum_pulse::{Operation, Category, profile};

#[derive(Debug)]
enum AppOperation {
    DatabaseQuery,
    NetworkRequest,
}

// Implement Operation trait manually
impl Operation for AppOperation {
    fn get_category(&self) -> &dyn Category {
        match self {
            AppOperation::DatabaseQuery => &DatabaseCategory,
            AppOperation::NetworkRequest => &NetworkCategory,
        }
    }
}

// Define custom categories
struct DatabaseCategory;
impl Category for DatabaseCategory {
    fn get_name(&self) -> &str { "Database" }
    fn get_description(&self) -> &str { "Database operations" }
}

struct NetworkCategory;
impl Category for NetworkCategory {
    fn get_name(&self) -> &str { "Network" }
    fn get_description(&self) -> &str { "Network operations" }
}
```

## Advanced Features

### Async Support

```rust
use quantum_pulse::{ProfileOp, profile_async};

#[derive(Debug, ProfileOp)]
enum AsyncOperation {
    #[category(name = "IO", description = "I/O operations")]
    FileRead,
    
    #[category(name = "Network", description = "Network operations")]
    HttpRequest,
    
    #[category(name = "Database")]
    DatabaseQuery,
}

async fn fetch_data() -> Result<Data, Error> {
    // Profile async operations seamlessly
    let data = profile_async!(AsyncOperation::HttpRequest, async {
        client.get("https://api.example.com/data").await
    }).await;
    
    profile_async!(AsyncOperation::DatabaseQuery, async {
        process_data(data).await
    }).await
}
```

### Complex Enum Variants

The `ProfileOp` macro supports all enum variant types:

```rust
#[derive(Debug, ProfileOp)]
enum ComplexOperation {
    // Unit variant
    #[category(name = "Simple")]
    Basic,
    
    // Tuple variant with data
    #[category(name = "Database", description = "Database operations")]
    Query(String),
    
    // Struct variant with named fields
    #[category(name = "Cache", description = "Cache operations")]
    CacheOp { key: String, ttl: u64 },
}

fn example() {
    let op1 = ComplexOperation::Basic;
    let op2 = ComplexOperation::Query("SELECT * FROM users".to_string());
    let op3 = ComplexOperation::CacheOp { 
        key: "user:123".to_string(), 
        ttl: 3600 
    };
    
    // All variants work seamlessly with profiling
    profile!(op1, { /* work */ });
    profile!(op2, { /* work */ });
    profile!(op3, { /* work */ });
}
```

### Report Generation

```rust
use quantum_pulse::{ProfileCollector, ReportBuilder, TimeFormat};

// Quick summary
let summary = ProfileCollector::get_summary();
println!("Total operations: {}", summary.total_operations);
println!("Total time: {} µs", summary.total_time_micros);

// Detailed report with configuration
let report = ReportBuilder::new()
    .include_percentiles(true)
    .group_by_category(true)
    .time_format(TimeFormat::Milliseconds)
    .build();

println!("{}", report.to_string());

// Export to CSV
let stats = ProfileCollector::get_all_stats();
let mut csv = String::from("Operation,Count,Mean(µs)\n");
for (name, stat) in stats {
    csv.push_str(&format!("{},{},{:.2}\n", 
        name, stat.count, stat.mean().as_micros()));
}
std::fs::write("profile.csv", csv).unwrap();
```

### Pausable Timers

For operations where you need to exclude certain periods:

```rust
use quantum_pulse::{PausableTimer, ProfileOp};

#[derive(Debug, ProfileOp)]
enum Operation {
    #[category(name = "Processing")]
    DataProcessing,
}

fn process_with_io() {
    let mut timer = PausableTimer::new(&Operation::DataProcessing);
    
    // Processing phase 1 (measured)
    process_part_1();
    
    timer.pause();
    // I/O operation (not measured)
    let data = read_from_disk();
    timer.resume();
    
    // Processing phase 2 (measured)
    process_part_2(data);
    
    // Timer automatically records on drop
}
```

## Zero-Cost Abstractions

Quantum Pulse implements true zero-cost abstractions through compile-time feature selection:

### How It Works

```rust
#[derive(Debug, ProfileOp)]
enum AppOp {
    #[category(name = "Critical")]
    ImportantWork,
}

// Your code always looks the same
let result = profile!(AppOp::ImportantWork, {
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

## Migration Guide

### From String-based Profiling

If you're currently using string-based operation names, migrate to type-safe enums:

```rust
// Before: String-based (error-prone, no compile-time checks)
profile!("database_query", {
    query_database()
});

// After: Type-safe with ProfileOp (recommended)
#[derive(Debug, ProfileOp)]
enum DbOp {
    #[category(name = "Database")]
    Query,
}

profile!(DbOp::Query, {
    query_database()
});
```

### From Manual Implementation

If you have existing manual `Operation` implementations, you can gradually migrate:

```rust
// Before: Manual implementation
#[derive(Debug)]
enum OldOp {
    Task1,
    Task2,
}

impl Operation for OldOp {
    fn get_category(&self) -> &dyn Category {
        // Manual category logic
    }
}

// After: Simply add ProfileOp derive
#[derive(Debug, ProfileOp)]
enum NewOp {
    #[category(name = "Tasks", description = "Application tasks")]
    Task1,
    
    #[category(name = "Tasks")]
    Task2,
}
```

## Examples

Check out the `examples/` directory for comprehensive examples:

- `macro_derive.rs` - **Recommended**: Using the ProfileOp derive macro
- `basic.rs` - Simple profiling example
- `custom_categories.rs` - Manual category implementation
- `async_profiling.rs` - Profiling async code
- `trading_system.rs` - Real-world trading system example

Run examples with:

```bash
# Recommended: See the derive macro in action
cargo run --example macro_derive --features full

# Other examples
cargo run --example basic --features full
cargo run --example async_profiling --features full
cargo run --example trading_system --features full
```

## Feature Flags

- `full`: Enable full profiling functionality with HDR histograms and derive macros
- `macros`: Enable only the derive macros (included in `full`)
- Default (no features): Stub implementation with zero overhead

## Best Practices

1. **Use ProfileOp Derive**: Start with the derive macro for cleaner, more maintainable code
2. **Organize by Category**: Group related operations under the same category name
3. **Descriptive Names**: Use clear, descriptive names for both categories and operations
4. **Profile Boundaries**: Profile at meaningful boundaries (API calls, database queries, etc.)
5. **Avoid Over-Profiling**: Don't profile every function - focus on potential bottlenecks

## Performance Considerations

The library is designed with performance in mind:

- **True Zero-Cost**: Stub implementations are completely removed by the compiler
- **Efficient Percentiles**: Using HDR histograms for O(1) percentile calculations
- **Lock-Free Operations**: Using atomic operations and thread-local storage
- **Smart Inlining**: Critical paths marked with `#[inline(always)]` in stub mode
- **No Runtime Checks**: Feature selection happens at compile time

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

## Acknowledgments

This library was designed for high-performance applications requiring microsecond-precision profiling with minimal overhead and maximum ergonomics.