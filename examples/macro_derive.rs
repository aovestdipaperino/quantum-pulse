//! Example demonstrating the use of the Operation derive macro
//!
//! This example shows how to use the #[derive(Operation)] macro to automatically
//! implement the Operation trait for enums with custom categories.

use quantum_pulse::{profile, Operation, ProfileCollector, ProfileOp};

// Define an enum with the Operation derive macro
#[derive(Debug, ProfileOp)]
enum MyProfileOperation {
    // Category with both name and description
    #[category(name = "IO", description = "File system write operations")]
    WriteFile,

    // Same category name but no description (will use the first description found)
    #[category(name = "IO")]
    ReadFile,

    // Category with only name (description defaults to the name)
    #[category(name = "Network")]
    HttpRequest,

    // Another variant with the same Network category
    #[category(name = "Network")]
    WebSocketMessage,

    // Different description for same category (will be ignored, first one wins)
    #[category(name = "IO", description = "This description will be ignored")]
    DeleteFile,

    // No category attribute - uses variant name as category
    ComputeHash,

    // Enum variant with data
    #[category(name = "Database", description = "Database operations")]
    QueryDatabase(String),

    // Enum variant with named fields
    #[category(name = "Cache", description = "Cache operations")]
    CacheOperation {
        key: String,
        ttl: u64,
    },
}

fn main() {
    // Clear any existing profiling data
    ProfileCollector::clear_all();

    // Example 1: IO operations (multiple variants, same category)
    let write_op = MyProfileOperation::WriteFile;
    println!(
        "WriteFile category: {} - {}",
        write_op.get_category().get_name(),
        write_op.get_category().get_description()
    );

    let _result = profile!(write_op, {
        // Simulate file write
        std::thread::sleep(std::time::Duration::from_millis(10));
        "file written"
    });

    let read_op = MyProfileOperation::ReadFile;
    println!(
        "ReadFile category: {} - {}",
        read_op.get_category().get_name(),
        read_op.get_category().get_description()
    );

    let _result = profile!(read_op, {
        // Simulate file read
        std::thread::sleep(std::time::Duration::from_millis(5));
        "file content"
    });

    // Example 2: Network operations
    let http_op = MyProfileOperation::HttpRequest;
    println!(
        "HttpRequest category: {} - {}",
        http_op.get_category().get_name(),
        http_op.get_category().get_description()
    );

    let _result = profile!(http_op, {
        // Simulate HTTP request
        std::thread::sleep(std::time::Duration::from_millis(20));
        "response"
    });

    // Example 3: No category attribute (uses variant name)
    let compute_op = MyProfileOperation::ComputeHash;
    println!(
        "ComputeHash category: {} - {}",
        compute_op.get_category().get_name(),
        compute_op.get_category().get_description()
    );

    let _result = profile!(compute_op, {
        // Simulate computation
        std::thread::sleep(std::time::Duration::from_millis(15));
        "hash_value"
    });

    // Example 4: Enum variant with data
    let db_op = MyProfileOperation::QueryDatabase("users".to_string());
    println!(
        "QueryDatabase category: {} - {}",
        db_op.get_category().get_name(),
        db_op.get_category().get_description()
    );

    let _result = profile!(db_op, {
        // Simulate database query
        std::thread::sleep(std::time::Duration::from_millis(30));
        vec!["user1", "user2"]
    });

    // Example 5: Enum variant with named fields
    let cache_op = MyProfileOperation::CacheOperation {
        key: "user:123".to_string(),
        ttl: 3600,
    };
    println!(
        "CacheOperation category: {} - {}",
        cache_op.get_category().get_name(),
        cache_op.get_category().get_description()
    );

    let _result = profile!(cache_op, {
        // Simulate cache operation
        std::thread::sleep(std::time::Duration::from_millis(2));
        "cached"
    });

    // Example 6: DeleteFile - same IO category
    let delete_op = MyProfileOperation::DeleteFile;
    println!(
        "DeleteFile category: {} - {}",
        delete_op.get_category().get_name(),
        delete_op.get_category().get_description()
    );
    // Notice that the description is "File system write operations" (from WriteFile)
    // not "This description will be ignored"

    // Generate and print the profiling report
    println!("\n=== Profiling Report ===");
    let summary = ProfileCollector::get_summary();
    println!("Total operations: {}", summary.total_operations);
    println!("Unique operations: {}", summary.unique_operations);
    println!("Total time: {} µs", summary.total_time_micros);

    // Get detailed stats for each operation
    println!("\n=== Detailed Stats ===");
    let all_stats = ProfileCollector::get_all_stats();
    for (operation, stats) in all_stats.iter() {
        println!(
            "{}: {} calls, avg {:?}",
            operation,
            stats.count,
            stats.mean()
        );
    }
}
