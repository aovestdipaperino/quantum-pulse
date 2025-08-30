//! Example demonstrating async profiling capabilities with enum-based operations

use quantum_pulse::{profile, Category, Profiler};
use std::time::Duration;
use tokio::time::sleep;

/// Specific async service operations for profiling
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AsyncOperation {
    // Authentication operations
    AuthenticateUser,

    // Database operations
    FetchUserProfile,
    FetchUserPreferences,
    FetchUserHistory,
    ConcurrentDbOp1,
    ConcurrentDbOp2,
    ConcurrentDbOp3,

    // HTTP operations
    EnrichWithExternalData,
    ConcurrentHttpOp,

    // Message queue operations
    PublishResult,
    ConcurrentMsgOp,

    // Data processing operations
    ProcessUserData,
    ComplexProcessing,
    BatchProcessing,

    // Coordination operations
    ServiceCoordination,
}

/// Custom categories for organizing operations
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ServiceCategory {
    HttpRequest,
    DatabaseQuery,
    MessageQueue,
    DataProcessing,
    Coordination,
}

impl Category for AsyncOperation {
    fn description(&self) -> Option<&str> {
        match self {
            AsyncOperation::AuthenticateUser => Some("Authenticate user credentials"),
            AsyncOperation::FetchUserProfile => Some("Fetch user profile from database"),
            AsyncOperation::FetchUserPreferences => Some("Fetch user preferences from database"),
            AsyncOperation::FetchUserHistory => Some("Fetch user history from database"),
            AsyncOperation::ConcurrentDbOp1 => Some("Concurrent database operation 1"),
            AsyncOperation::ConcurrentDbOp2 => Some("Concurrent database operation 2"),
            AsyncOperation::ConcurrentDbOp3 => Some("Concurrent database operation 3"),
            AsyncOperation::EnrichWithExternalData => Some("Enrich data via external HTTP API"),
            AsyncOperation::ConcurrentHttpOp => Some("Concurrent HTTP request"),
            AsyncOperation::PublishResult => Some("Publish result to message queue"),
            AsyncOperation::ConcurrentMsgOp => Some("Concurrent message queue operation"),
            AsyncOperation::ProcessUserData => Some("Process user data"),
            AsyncOperation::ComplexProcessing => Some("Complex data processing task"),
            AsyncOperation::BatchProcessing => Some("Batch processing operation"),
            AsyncOperation::ServiceCoordination => Some("Service coordination task"),
        }
    }

    fn priority(&self) -> i32 {
        match self {
            // Critical path operations
            AsyncOperation::AuthenticateUser => 1,
            AsyncOperation::ProcessUserData => 1,

            // Core data operations
            AsyncOperation::FetchUserProfile | AsyncOperation::FetchUserPreferences => 2,
            AsyncOperation::FetchUserHistory => 2,

            // External dependencies
            AsyncOperation::EnrichWithExternalData => 3,
            AsyncOperation::PublishResult => 3,

            // Background processing
            AsyncOperation::ComplexProcessing | AsyncOperation::BatchProcessing => 4,

            // Concurrent operations
            AsyncOperation::ConcurrentDbOp1 | AsyncOperation::ConcurrentDbOp2 => 5,
            AsyncOperation::ConcurrentDbOp3 | AsyncOperation::ConcurrentHttpOp => 5,
            AsyncOperation::ConcurrentMsgOp => 5,

            // Coordination
            AsyncOperation::ServiceCoordination => 6,
        }
    }
}

impl AsyncOperation {
    fn category(&self) -> ServiceCategory {
        match self {
            AsyncOperation::AuthenticateUser | AsyncOperation::FetchUserProfile => {
                ServiceCategory::DatabaseQuery
            }
            AsyncOperation::FetchUserPreferences | AsyncOperation::FetchUserHistory => {
                ServiceCategory::DatabaseQuery
            }
            AsyncOperation::ConcurrentDbOp1 | AsyncOperation::ConcurrentDbOp2 => {
                ServiceCategory::DatabaseQuery
            }
            AsyncOperation::ConcurrentDbOp3 => ServiceCategory::DatabaseQuery,

            AsyncOperation::EnrichWithExternalData | AsyncOperation::ConcurrentHttpOp => {
                ServiceCategory::HttpRequest
            }

            AsyncOperation::PublishResult | AsyncOperation::ConcurrentMsgOp => {
                ServiceCategory::MessageQueue
            }

            AsyncOperation::ProcessUserData | AsyncOperation::ComplexProcessing => {
                ServiceCategory::DataProcessing
            }
            AsyncOperation::BatchProcessing => ServiceCategory::DataProcessing,

            AsyncOperation::ServiceCoordination => ServiceCategory::Coordination,
        }
    }
}

type ServiceProfiler = Profiler<AsyncOperation>;

impl Category for ServiceCategory {
    fn description(&self) -> Option<&str> {
        match self {
            ServiceCategory::HttpRequest => Some("External HTTP API calls"),
            ServiceCategory::DatabaseQuery => Some("Database operations and queries"),
            ServiceCategory::MessageQueue => Some("Message queue operations"),
            ServiceCategory::DataProcessing => Some("Data transformation and processing"),
            ServiceCategory::Coordination => Some("Service coordination and orchestration"),
        }
    }

    fn priority(&self) -> i32 {
        match self {
            ServiceCategory::HttpRequest => 1,
            ServiceCategory::DatabaseQuery => 2,
            ServiceCategory::MessageQueue => 3,
            ServiceCategory::DataProcessing => 4,
            ServiceCategory::Coordination => 5,
        }
    }
}

impl std::fmt::Display for ServiceCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            ServiceCategory::HttpRequest => "HTTP Requests",
            ServiceCategory::DatabaseQuery => "Database Queries",
            ServiceCategory::MessageQueue => "Message Queue",
            ServiceCategory::DataProcessing => "Data Processing",
            ServiceCategory::Coordination => "Coordination",
        };
        write!(f, "{}", name)
    }
}

#[tokio::main]
async fn main() {
    println!("=== Async Profiling Example ===\n");

    // Example 1: Simple async profiling
    println!("1. Simple async operation profiling:");
    let result = Profiler::<ServiceCategory>::time_async("fetch_data", || async {
        simulate_async_work(50).await;
        "Data fetched successfully"
    })
    .await;
    println!("   Result: {}", result);

    // Example 2: Using async profile! macro
    println!("\n2. Using async profile! macro:");
    let data = profile!(AsyncOperation::ComplexProcessing => async {
        simulate_async_work(30).await;
        vec![1, 2, 3, 4, 5]
    });
    println!("   Processed data: {:?}", data);

    // Example 3: Categorized async operations
    println!("\n3. Categorized async operations:");
    let api_name = format!("{:?}", AsyncOperation::EnrichWithExternalData);
    let api_result = ServiceProfiler::time_async_with_category(
        &api_name,
        AsyncOperation::EnrichWithExternalData,
        || async {
            simulate_async_work(100).await;
            "API response"
        },
    )
    .await;
    println!("   API result: {}", api_result);

    // Example 4: Concurrent async operations
    println!("\n4. Running concurrent async operations:");
    run_concurrent_operations().await;

    // Example 5: Complex async workflow
    println!("\n5. Complex async workflow:");
    process_user_request(123).await;

    // Example 6: Streaming data processing
    println!("\n6. Streaming data processing:");
    process_stream().await;

    // Example 7: Pausable async timer
    println!("\n7. Using pausable timer in async context:");
    async_with_pausable_timer().await;

    // Generate report
    println!("\n{}", "=".repeat(70));
    println!("ASYNC PROFILING REPORT");
    println!("{}\n", "=".repeat(70));

    let report = ServiceProfiler::report();
    println!("{:#?}", report);

    // Show statistics for specific operations
    show_async_stats();
}

async fn run_concurrent_operations() {
    println!("   Starting 3 concurrent operations...");

    let op1_name = format!("{:?}", AsyncOperation::ConcurrentDbOp1);
    let op2_name = format!("{:?}", AsyncOperation::ConcurrentHttpOp);
    let op3_name = format!("{:?}", AsyncOperation::ConcurrentMsgOp);

    let (result1, result2, result3) = tokio::join!(
        ServiceProfiler::time_async_with_category(
            &op1_name,
            AsyncOperation::ConcurrentDbOp1,
            || async {
                simulate_async_work(50).await;
                "Query 1 complete"
            }
        ),
        ServiceProfiler::time_async_with_category(
            &op2_name,
            AsyncOperation::ConcurrentHttpOp,
            || async {
                simulate_async_work(75).await;
                "HTTP request complete"
            }
        ),
        ServiceProfiler::time_async_with_category(
            &op3_name,
            AsyncOperation::ConcurrentMsgOp,
            || async {
                simulate_async_work(60).await;
                "Message published"
            }
        )
    );

    println!("   Results: {}, {}, {}", result1, result2, result3);
}

async fn process_user_request(user_id: u64) {
    println!("   Processing request for user {}...", user_id);

    // Step 1: Authenticate user
    let _auth_token = profile!(AsyncOperation::AuthenticateUser => async {
        simulate_async_work(20).await;
        format!("token_{}", user_id)
    });

    // Step 2: Fetch user data (parallel queries)
    let profile_name = format!("{:?}", AsyncOperation::FetchUserProfile);
    let prefs_name = format!("{:?}", AsyncOperation::FetchUserPreferences);
    let history_name = format!("{:?}", AsyncOperation::FetchUserHistory);

    let (profile_data, preferences, history) = tokio::join!(
        ServiceProfiler::time_async_with_category(
            &profile_name,
            AsyncOperation::FetchUserProfile,
            || async {
                simulate_async_work(30).await;
                "User profile"
            }
        ),
        ServiceProfiler::time_async_with_category(
            &prefs_name,
            AsyncOperation::FetchUserPreferences,
            || async {
                simulate_async_work(25).await;
                "User preferences"
            }
        ),
        ServiceProfiler::time_async_with_category(
            &history_name,
            AsyncOperation::FetchUserHistory,
            || async {
                simulate_async_work(35).await;
                "User history"
            }
        )
    );

    // Step 3: Process data
    let processed = profile!(AsyncOperation::ProcessUserData => async {
        simulate_async_work(40).await;
        format!("Processed: {} + {} + {}", profile_data, preferences, history)
    });

    // Step 4: Call external service
    let enriched = profile!(AsyncOperation::EnrichWithExternalData => async {
        simulate_async_work(80).await;
        format!("{} [enriched]", processed)
    });

    // Step 5: Publish to message queue
    profile!(AsyncOperation::PublishResult => async {
        simulate_async_work(15).await;
        println!("   Published result for user {}: {}", user_id, enriched);
    });

    println!("   Request processing complete for user {}", user_id);
}

async fn process_stream() {
    use tokio_stream::{self as stream, StreamExt};

    println!("   Processing stream of 5 items...");

    let mut stream = stream::iter(1..=5);

    while let Some(item) = stream.next().await {
        profile!(AsyncOperation::BatchProcessing => async {
            simulate_async_work(10 * item).await;
            println!("     Processed stream item: {}", item);
        });
    }

    println!("   Stream processing complete");
}

async fn async_with_pausable_timer() {
    // Note: PausableTimer doesn't work with enum-based approach in current implementation
    // Using direct timing instead
    let start = std::time::Instant::now();

    println!("   Starting complex operation with pausable timer...");

    // Do some work
    simulate_async_work(20).await;
    println!("     Phase 1 complete");

    // Do some work
    simulate_async_work(30).await;
    println!("     Phase 2 complete");

    let elapsed = start.elapsed();
    let coord_name = format!("{:?}", AsyncOperation::ServiceCoordination);
    ServiceProfiler::record_with_category(
        &coord_name,
        AsyncOperation::ServiceCoordination,
        elapsed.as_micros() as u64,
    );
    println!("   Complex operation complete in {:?}", elapsed);
}

fn show_async_stats() {
    println!("\n{}", "=".repeat(70));
    println!("DETAILED ASYNC OPERATION STATISTICS");
    println!("{}", "=".repeat(70));

    // Show all operation statistics
    let all_stats = ServiceProfiler::get_all_stats();
    for (operation_name, stats) in all_stats {
        println!("\n📊 {}:", operation_name);
        println!("   Calls: {}", stats.count);
        println!("   Mean: {:.1} μs", stats.mean_micros);
        println!("   Min:  {} μs", stats.min_micros);
        println!("   Max:  {} μs", stats.max_micros);
    }

    // Show summary by operation type
    println!("\n{}", "=".repeat(70));
    println!("SUMMARY BY OPERATION TYPE");
    println!("{}", "=".repeat(70));
    println!("Note: Operations are self-categorizing based on their enum definitions");

    // Generate report with enum-based operations
    let report = ServiceProfiler::report();
    println!("{:#?}", report);
}

async fn simulate_async_work(millis: u64) {
    sleep(Duration::from_millis(millis)).await;
}
