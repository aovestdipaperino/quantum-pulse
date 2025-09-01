//! Basic example demonstrating enum-based profiling with Operation trait

use quantum_pulse::{profile, profile_async, scoped_timer, Category, Operation, ProfileCollector};
use std::thread;
use std::time::Duration;
use tokio::time::sleep;

/// Basic application operations for profiling
#[derive(Debug)]
enum AppOperation {
    Fibonacci,
    SumNumbers,
    Iteration,
    ManualOperation,
    ConditionalOperation,
    ExpensiveOperation,
    ScopedOperation,
}

/// BasicOperation enum for direct string operations
#[derive(Debug)]
enum BasicOperation {
    Test,
    AsyncTest,
    FileRead,
    AsyncOperation,
}

impl Operation for BasicOperation {
    fn to_str(&self) -> String {
        match self {
            BasicOperation::Test => "test_operation".to_string(),
            BasicOperation::AsyncTest => "async_test".to_string(),
            BasicOperation::FileRead => "file_read".to_string(),
            BasicOperation::AsyncOperation => "async_operation".to_string(),
        }
    }
}

/// Custom category for compute-intensive operations
#[derive(Debug)]
struct ComputeCategory;

impl Category for ComputeCategory {
    fn get_name(&self) -> &str {
        "Compute"
    }

    fn get_description(&self) -> &str {
        "CPU-intensive computational operations"
    }

    fn color_hint(&self) -> Option<&str> {
        Some("#e74c3c")
    }

    fn priority(&self) -> i32 {
        1
    }
}

/// Custom category for I/O operations
#[derive(Debug)]
struct IOCategory;

impl Category for IOCategory {
    fn get_name(&self) -> &str {
        "IO"
    }

    fn get_description(&self) -> &str {
        "Input/Output operations"
    }

    fn color_hint(&self) -> Option<&str> {
        Some("#3498db")
    }

    fn priority(&self) -> i32 {
        2
    }
}

impl Operation for AppOperation {
    fn get_category(&self) -> &dyn Category {
        match self {
            AppOperation::Fibonacci
            | AppOperation::SumNumbers
            | AppOperation::ExpensiveOperation => &ComputeCategory,
            AppOperation::ScopedOperation => &IOCategory,
            _ => &quantum_pulse::NoCategory,
        }
    }

    fn to_str(&self) -> String {
        match self {
            AppOperation::Fibonacci => "fibonacci_calc".to_string(),
            AppOperation::SumNumbers => "sum_numbers".to_string(),
            AppOperation::Iteration => "iteration".to_string(),
            AppOperation::ManualOperation => "manual_op".to_string(),
            AppOperation::ConditionalOperation => "conditional_op".to_string(),
            AppOperation::ExpensiveOperation => "expensive_op".to_string(),
            AppOperation::ScopedOperation => "scoped_op".to_string(),
        }
    }
}

#[tokio::main]
async fn main() {
    println!("=== Basic Enum-Based Profiling Example ===\n");

    // Example 1: Simple function profiling with enums
    println!("1. Simple function profiling with enums:");
    let result = profile!(AppOperation::Fibonacci, { fibonacci(30) });
    println!("   Fibonacci(30) = {}", result);

    // Example 2: Using BasicOperation for quick profiling
    println!("\n2. Using BasicOperation for quick profiling:");
    let data = profile!(BasicOperation::FileRead, {
        thread::sleep(Duration::from_millis(10));
        "file_content"
    });
    println!("   File data: {}", data);

    // Example 3: Multiple operations with enums
    println!("\n3. Profiling multiple operations with enums:");
    for i in 0..3 {
        profile!(AppOperation::Iteration, {
            thread::sleep(Duration::from_millis(5));
            println!("   Completed iteration {}", i);
        });
    }

    // Example 4: Async operations
    println!("\n4. Async operations:");
    let async_result = profile_async!(BasicOperation::AsyncOperation, async {
        sleep(Duration::from_millis(20)).await;
        "async_result"
    })
    .await;
    println!("   Async result: {}", async_result);

    // Example 5: Conditional profiling
    println!("\n5. Conditional profiling:");
    let enable_profiling = true;
    if enable_profiling {
        let result = profile!(AppOperation::ConditionalOperation, {
            expensive_operation()
        });
        println!("   Operation result: {}", result);
    }

    // Example 6: Scoped timing
    println!("\n6. Scoped timing:");
    {
        scoped_timer!(AppOperation::ScopedOperation);
        println!("   Doing work in a scoped block...");
        thread::sleep(Duration::from_millis(10));
        println!("   Scoped block complete");
    }

    // Example 7: Sum numbers with compute category
    println!("\n7. Sum numbers with compute category:");
    let sum = profile!(AppOperation::SumNumbers, {
        let mut total = 0u64;
        for i in 1..=100_000 {
            total += i;
        }
        total
    });
    println!("   Sum of 1 to 100,000 = {}", sum);

    // Example 8: Using other enum variants to avoid warnings
    if false {
        // This code never runs but prevents unused variant warnings
        profile!(AppOperation::ManualOperation, { 42 });
        profile!(AppOperation::ExpensiveOperation, { 42 });

        // For test variants in BasicOperation
        let _test_result = profile_async!(BasicOperation::Test, async { 42 }).await;
        let _async_test_result = profile_async!(BasicOperation::AsyncTest, async { 42 }).await;
    }

    // Generate and display report
    println!("\n{}", "=".repeat(70));
    println!("PROFILING REPORT");
    println!("{}\n", "=".repeat(70));

    ProfileCollector::report_stats();

    // Show summary
    let summary = ProfileCollector::get_summary();
    println!("\nSummary:");
    println!("- Total operations: {}", summary.total_operations);
    println!("- Unique operations: {}", summary.unique_operations);
    println!("- Total time: {}μs", summary.total_time_micros);

    // Show detailed stats
    println!("\n{}", "=".repeat(70));
    println!("DETAILED OPERATION STATISTICS");
    println!("{}\n", "=".repeat(70));

    let all_stats = ProfileCollector::get_all_stats();
    for (operation_name, stats) in all_stats {
        println!("📊 {}:", operation_name);
        println!("   Calls: {}", stats.count);
        println!("   Mean: {:?}", stats.mean());
        println!("   Total: {:?}", stats.total);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_operation_categories() {
        let fib_op = AppOperation::Fibonacci;
        assert_eq!(fib_op.get_category().get_name(), "Compute");
        assert_eq!(fib_op.to_str(), "fibonacci_calc");

        let scoped_op = AppOperation::ScopedOperation;
        assert_eq!(scoped_op.get_category().get_name(), "IO");
        assert_eq!(scoped_op.to_str(), "scoped_op");

        let iter_op = AppOperation::Iteration;
        assert_eq!(iter_op.get_category().get_name(), "NoCategory");
        assert_eq!(iter_op.to_str(), "iteration");
    }

    #[test]
    fn test_profiling_with_operations() {
        ProfileCollector::clear_all();

        let op = AppOperation::Fibonacci;
        profile!(op, { fibonacci(5) });

        assert!(ProfileCollector::has_data());
        let stats = ProfileCollector::get_stats("Compute::fibonacci_calc");
        assert!(stats.is_some());
        assert_eq!(stats.unwrap().count, 1);
    }

    #[tokio::test]
    async fn test_async_profiling() {
        ProfileCollector::clear_all();

        // Pass the enum variant directly to the macro
        let result = profile_async!(BasicOperation::AsyncTest, async {
            sleep(Duration::from_millis(1)).await;
            42
        })
        .await;

        assert_eq!(result, 42);
        assert!(ProfileCollector::has_data());
    }
}
