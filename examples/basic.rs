//! Basic example demonstrating enum-based profiling usage

use quantum_pulse::{profile, profile_if, Category, Profiler, ReportBuilder, TimeFormat};
use std::thread;
use std::time::Duration;

/// Basic application operations for profiling
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppOperation {
    Fibonacci,
    SumNumbers,
    Iteration,
    ManualOperation,
    ConditionalOperation,
    ExpensiveOperation,
    ScopedOperation,
}

impl Category for AppOperation {
    fn description(&self) -> Option<&str> {
        match self {
            AppOperation::Fibonacci => Some("Recursive Fibonacci calculation"),
            AppOperation::SumNumbers => Some("Sum of sequential numbers"),
            AppOperation::Iteration => Some("Loop iteration operations"),
            AppOperation::ManualOperation => Some("Manually recorded operation"),
            AppOperation::ConditionalOperation => Some("Conditionally profiled operation"),
            AppOperation::ExpensiveOperation => Some("CPU intensive operation"),
            AppOperation::ScopedOperation => Some("RAII scoped timer operation"),
        }
    }

    fn priority(&self) -> i32 {
        match self {
            AppOperation::Fibonacci | AppOperation::ExpensiveOperation => 1, // CPU intensive
            AppOperation::SumNumbers | AppOperation::Iteration => 2,         // Regular operations
            _ => 3,                                                          // Less critical
        }
    }
}

type AppProfiler = Profiler<AppOperation>;

fn main() {
    println!("=== Basic Enum-Based Profiling Example ===\n");

    // Example 1: Simple function profiling with enums
    println!("1. Simple function profiling with enums:");
    let result = AppProfiler::time_with_category(
        &format!("{:?}", AppOperation::Fibonacci),
        AppOperation::Fibonacci,
        || fibonacci(35),
    );
    println!("   Fibonacci(35) = {}", result);

    // Example 2: Using the profile! macro with enums
    println!("\n2. Using the profile! macro with enums:");
    let sum = profile!(AppOperation::SumNumbers => {
        let mut total = 0u64;
        for i in 1..=1_000_000 {
            total += i;
        }
        total
    });
    println!("   Sum of 1 to 1,000,000 = {}", sum);

    // Example 3: Multiple operations with enum profiling
    println!("\n3. Profiling multiple operations with enums:");
    for i in 0..5 {
        profile!(AppOperation::Iteration => {
            thread::sleep(Duration::from_millis(i * 10));
            println!("   Completed iteration {}", i);
        });
    }

    // Example 4: Manual recording with enum category
    println!("\n4. Manual metric recording with enums:");
    for i in 1..=10 {
        // Simulate varying response times
        let simulated_time = (i * i * 100) as u64;
        AppProfiler::record_with_category(
            &format!("{:?}", AppOperation::ManualOperation),
            AppOperation::ManualOperation,
            simulated_time,
        );
    }

    // Example 5: Conditional profiling with enums
    println!("\n5. Conditional profiling with enums:");
    let enable_profiling = true;
    let result = profile_if!(enable_profiling, AppOperation::ConditionalOperation => {
        expensive_operation()
    });
    println!("   Operation result: {}", result);

    // Example 6: Scoped timing with enums
    println!("\n6. Scoped timing with enums:");
    {
        AppProfiler::time_with_category(
            &format!("{:?}", AppOperation::ScopedOperation),
            AppOperation::ScopedOperation,
            || {
                println!("   Doing work in a scoped block...");
                thread::sleep(Duration::from_millis(50));
                println!("   Scoped block complete");
            },
        );
    }

    // Generate and display report
    println!("\n{}", "=".repeat(70));
    println!("PROFILING REPORT");
    println!("{}\n", "=".repeat(70));

    // Default report with enum-based categories
    let report = AppProfiler::report();
    println!("{:#?}", report);

    // Custom formatted report with categories
    println!("\n{}", "=".repeat(70));
    println!("CUSTOM FORMATTED REPORT (Grouped by Category)");
    println!("{}\n", "=".repeat(70));

    let custom_report = ReportBuilder::<AppOperation>::new()
        .group_by_category(true)
        .include_percentiles(true)
        .time_format(TimeFormat::Microseconds)
        .build();

    println!("{:#?}", custom_report);

    // Quick summary of all operations
    println!("\n{}", "=".repeat(70));
    println!("OPERATION STATISTICS");
    println!("{}\n", "=".repeat(70));

    let all_stats = AppProfiler::get_all_stats();
    for (operation_name, stats) in all_stats {
        println!("📊 {}:", operation_name);
        println!("   Calls: {}", stats.count);
        println!("   Mean: {:.1} μs", stats.mean_micros);
        println!("   Min: {} μs", stats.min_micros);
        println!("   Max: {} μs", stats.max_micros);
        println!();
    }
}

fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn expensive_operation() -> i32 {
    thread::sleep(Duration::from_millis(25));
    42
}
