//! Example demonstrating async profiling capabilities with enum-based operations using Operation trait

use quantum_pulse::{profile_async, Category, Operation, ProfileCollector};
use std::time::Duration;
use tokio::time::sleep;
use tokio_stream::{self as stream, StreamExt};

/// Async service operations that implement Operation trait
#[derive(Debug)]
enum AsyncOperation {
    // Authentication operations
    AuthenticateUser,
    ValidateSession,

    // Database operations
    FetchUserProfile,
    FetchUserPreferences,
    FetchUserHistory,
    UpdateUserData,
    BatchQuery,

    // HTTP operations
    EnrichWithExternalData,
    CallPaymentApi,
    FetchWeatherData,

    // Message queue operations
    PublishEvent,
    ConsumeMessage,
    ProcessNotification,

    // Data processing operations
    ProcessUserData,
    ComplexAnalysis,
    StreamProcessing,
    BatchProcessing,

    // Coordination operations
    ServiceCoordination,
    WorkflowOrchestration,
}

/// Database category for async database operations
#[derive(Debug)]
struct AsyncDatabaseCategory;

impl Category for AsyncDatabaseCategory {
    fn get_name(&self) -> &str {
        "AsyncDatabase"
    }

    fn get_description(&self) -> &str {
        "Asynchronous database queries and transactions"
    }

    fn color_hint(&self) -> Option<&str> {
        Some("#FF6B6B")
    }

    fn priority(&self) -> i32 {
        1
    }
}

/// HTTP category for external API calls
#[derive(Debug)]
struct AsyncHttpCategory;

impl Category for AsyncHttpCategory {
    fn get_name(&self) -> &str {
        "AsyncHTTP"
    }

    fn get_description(&self) -> &str {
        "Asynchronous HTTP requests and external API calls"
    }

    fn color_hint(&self) -> Option<&str> {
        Some("#4ECDC4")
    }

    fn priority(&self) -> i32 {
        2
    }
}

/// Message queue category for async messaging
#[derive(Debug)]
struct AsyncMessageCategory;

impl Category for AsyncMessageCategory {
    fn get_name(&self) -> &str {
        "AsyncMessage"
    }

    fn get_description(&self) -> &str {
        "Asynchronous message queue operations"
    }

    fn color_hint(&self) -> Option<&str> {
        Some("#45B7D1")
    }

    fn priority(&self) -> i32 {
        3
    }
}

/// Processing category for data processing operations
#[derive(Debug)]
struct AsyncProcessingCategory;

impl Category for AsyncProcessingCategory {
    fn get_name(&self) -> &str {
        "AsyncProcessing"
    }

    fn get_description(&self) -> &str {
        "Asynchronous data processing and analysis"
    }

    fn color_hint(&self) -> Option<&str> {
        Some("#96CEB4")
    }

    fn priority(&self) -> i32 {
        4
    }
}

/// Authentication category for auth operations
#[derive(Debug)]
struct AsyncAuthCategory;

impl Category for AsyncAuthCategory {
    fn get_name(&self) -> &str {
        "AsyncAuth"
    }

    fn get_description(&self) -> &str {
        "Asynchronous authentication and authorization"
    }

    fn color_hint(&self) -> Option<&str> {
        Some("#DDA0DD")
    }

    fn priority(&self) -> i32 {
        5
    }
}

/// Coordination category for orchestration
#[derive(Debug)]
struct AsyncCoordinationCategory;

impl Category for AsyncCoordinationCategory {
    fn get_name(&self) -> &str {
        "AsyncCoordination"
    }

    fn get_description(&self) -> &str {
        "Asynchronous service coordination and orchestration"
    }

    fn color_hint(&self) -> Option<&str> {
        Some("#F4A460")
    }

    fn priority(&self) -> i32 {
        6
    }
}

impl Operation for AsyncOperation {
    fn get_category(&self) -> &dyn Category {
        match self {
            AsyncOperation::AuthenticateUser | AsyncOperation::ValidateSession => {
                &AsyncAuthCategory
            }

            AsyncOperation::FetchUserProfile
            | AsyncOperation::FetchUserPreferences
            | AsyncOperation::FetchUserHistory
            | AsyncOperation::UpdateUserData
            | AsyncOperation::BatchQuery => &AsyncDatabaseCategory,

            AsyncOperation::EnrichWithExternalData
            | AsyncOperation::CallPaymentApi
            | AsyncOperation::FetchWeatherData => &AsyncHttpCategory,

            AsyncOperation::PublishEvent
            | AsyncOperation::ConsumeMessage
            | AsyncOperation::ProcessNotification => &AsyncMessageCategory,

            AsyncOperation::ProcessUserData
            | AsyncOperation::ComplexAnalysis
            | AsyncOperation::StreamProcessing
            | AsyncOperation::BatchProcessing => &AsyncProcessingCategory,

            AsyncOperation::ServiceCoordination | AsyncOperation::WorkflowOrchestration => {
                &AsyncCoordinationCategory
            }
        }
    }

    fn to_str(&self) -> String {
        match self {
            AsyncOperation::AuthenticateUser => "authenticate_user".to_string(),
            AsyncOperation::ValidateSession => "validate_session".to_string(),
            AsyncOperation::FetchUserProfile => "fetch_user_profile".to_string(),
            AsyncOperation::FetchUserPreferences => "fetch_user_preferences".to_string(),
            AsyncOperation::FetchUserHistory => "fetch_user_history".to_string(),
            AsyncOperation::UpdateUserData => "update_user_data".to_string(),
            AsyncOperation::BatchQuery => "batch_query".to_string(),
            AsyncOperation::EnrichWithExternalData => "enrich_with_external_data".to_string(),
            AsyncOperation::CallPaymentApi => "call_payment_api".to_string(),
            AsyncOperation::FetchWeatherData => "fetch_weather_data".to_string(),
            AsyncOperation::PublishEvent => "publish_event".to_string(),
            AsyncOperation::ConsumeMessage => "consume_message".to_string(),
            AsyncOperation::ProcessNotification => "process_notification".to_string(),
            AsyncOperation::ProcessUserData => "process_user_data".to_string(),
            AsyncOperation::ComplexAnalysis => "complex_analysis".to_string(),
            AsyncOperation::StreamProcessing => "stream_processing".to_string(),
            AsyncOperation::BatchProcessing => "batch_processing".to_string(),
            AsyncOperation::ServiceCoordination => "service_coordination".to_string(),
            AsyncOperation::WorkflowOrchestration => "workflow_orchestration".to_string(),
        }
    }
}

#[tokio::main]
async fn main() {
    println!("=== Async Profiling Example ===\n");

    // Clear any existing data
    ProfileCollector::clear_all();

    // Example 1: Simple async operation profiling
    println!("1. Simple async operation profiling:");
    let result = profile_async!(AsyncOperation::AuthenticateUser, async {
        simulate_async_work(50).await;
        "User authenticated"
    })
    .await;
    println!("   Result: {}", result);

    // Example 2: Database operations
    println!("\n2. Database operations:");
    let result = profile_async!(AsyncOperation::FetchUserProfile, async {
        simulate_async_work(100).await;
        "User profile data"
    })
    .await;
    println!("   Result: {}", result);
    println!("   User data: {}", result);

    // Example 3: Concurrent async operations
    println!("\n3. Running concurrent async operations:");
    run_concurrent_operations().await;

    // Example 4: Complex async workflow
    println!("\n4. Complex async workflow:");
    process_user_request(123).await;

    // Example 5: Stream processing
    println!("\n5. Stream processing:");
    process_data_stream().await;

    // Example 6: Message queue operations
    println!("\n6. Message queue operations:");
    handle_message_queue().await;

    // Example 7: Coordinated workflow
    println!("\n7. Coordinated workflow:");
    // Simulate orchestrating a multi-service workflow
    orchestrate_services().await;

    // Use unused variants to avoid warnings
    if false {
        profile_async!(AsyncOperation::CallPaymentApi, async {
            println!("Processing payment");
        })
        .await;

        profile_async!(AsyncOperation::BatchProcessing, async {
            println!("Batch processing");
        })
        .await;
    }

    // Generate and display report
    show_async_profiling_results();
}

async fn run_concurrent_operations() {
    println!("   Starting 3 concurrent operations...");

    let (result1, result2, result3) = tokio::join!(
        profile_async!(AsyncOperation::BatchQuery, async {
            simulate_async_work(40).await;
            "Batch query complete"
        }),
        profile_async!(AsyncOperation::FetchWeatherData, async {
            simulate_async_work(60).await;
            "Weather data fetched"
        }),
        profile_async!(AsyncOperation::PublishEvent, async {
            simulate_async_work(25).await;
            "Event published"
        })
    );

    println!("   Results: {}, {}, {}", result1, result2, result3);
}

async fn process_user_request(user_id: u64) {
    println!("   Processing request for user {}...", user_id);

    // Step 1: Authentication
    let _auth_token = profile_async!(AsyncOperation::AuthenticateUser, async {
        simulate_async_work(20).await;
        format!("token_{}", user_id)
    })
    .await;

    // Step 2: Validate session
    profile_async!(AsyncOperation::ValidateSession, async {
        simulate_async_work(10).await;
    })
    .await;

    // Step 3: Fetch user data (parallel queries)
    let (profile_data, preferences, history) = tokio::join!(
        profile_async!(AsyncOperation::FetchUserProfile, async {
            simulate_async_work(35).await;
            "User profile"
        }),
        profile_async!(AsyncOperation::FetchUserPreferences, async {
            simulate_async_work(25).await;
            "User preferences"
        }),
        profile_async!(AsyncOperation::FetchUserHistory, async {
            simulate_async_work(45).await;
            "User history"
        })
    );

    // Step 4: Process data
    let processed = profile_async!(AsyncOperation::ProcessUserData, async {
        simulate_async_work(30).await;
        format!(
            "Processed: {} + {} + {}",
            profile_data, preferences, history
        )
    })
    .await;

    // Step 5: Enrich with external data
    let enriched = profile_async!(AsyncOperation::EnrichWithExternalData, async {
        simulate_async_work(70).await;
        format!("{} [enriched]", processed)
    })
    .await;

    // Step 6: Update user data
    profile_async!(AsyncOperation::UpdateUserData, async {
        simulate_async_work(20).await;
        println!("   Updated user data for user {}", user_id);
    })
    .await;

    // Step 7: Publish result
    profile_async!(AsyncOperation::PublishEvent, async {
        simulate_async_work(15).await;
        println!("   Published result for user {}: {}", user_id, enriched);
    })
    .await;

    println!("   Request processing complete for user {}", user_id);
}

async fn process_data_stream() {
    println!("   Processing stream of 5 items...");

    let mut stream = stream::iter(1..=5);

    while let Some(item) = stream.next().await {
        profile_async!(AsyncOperation::StreamProcessing, async {
            simulate_async_work(15 * item).await;
            println!("     Processed stream item: {}", item);
        })
        .await;
    }

    println!("   Stream processing complete");
}

async fn handle_message_queue() {
    println!("   Handling message queue operations...");

    // Simulate consuming messages
    for i in 1..=3 {
        let message = profile_async!(AsyncOperation::ConsumeMessage, async {
            simulate_async_work(20).await;
            format!("Message_{}", i)
        })
        .await;

        // Process notification
        profile_async!(AsyncOperation::ProcessNotification, async {
            simulate_async_work(10).await;
            println!("     Processed notification: {}", message);
        })
        .await;
    }

    println!("   Message queue handling complete");
}

async fn orchestrate_services() {
    println!("   Orchestrating services...");

    // Service coordination
    let coord_op = AsyncOperation::ServiceCoordination;
    profile_async!(coord_op, async {
        simulate_async_work(50).await;
        println!("     Services coordinated");
    });

    // Workflow orchestration
    let workflow_op = AsyncOperation::WorkflowOrchestration;
    profile_async!(workflow_op, async {
        // Simulate complex workflow
        for step in 1..=3 {
            simulate_async_work(30).await;
            println!("     Workflow step {} complete", step);
        }
    });

    // Complex analysis
    let analysis_op = AsyncOperation::ComplexAnalysis;
    profile_async!(analysis_op, async {
        simulate_async_work(80).await;
        println!("     Complex analysis complete");
    });

    println!("   Service orchestration complete");
}

fn show_async_profiling_results() {
    println!("\n{}", "=".repeat(70));
    println!("ASYNC PROFILING RESULTS BY CATEGORY");
    println!("{}", "=".repeat(70));

    ProfileCollector::report_stats();

    let summary = ProfileCollector::get_summary();
    println!("\nSUMMARY:");
    println!("- Total async operations: {}", summary.total_operations);
    println!("- Unique async operations: {}", summary.unique_operations);
    println!("- Total async time: {}μs", summary.total_time_micros);

    println!("\n{}", "=".repeat(70));
    println!("DETAILED ANALYSIS BY ASYNC CATEGORY");
    println!("{}", "=".repeat(70));

    let all_stats = ProfileCollector::get_all_stats();

    // Group operations by category
    let mut categories: std::collections::HashMap<
        String,
        Vec<(String, quantum_pulse::OperationStats)>,
    > = std::collections::HashMap::new();

    for (key, stats) in all_stats {
        if let Some((category, operation)) = key.split_once("::") {
            categories
                .entry(category.to_string())
                .or_insert_with(Vec::new)
                .push((operation.to_string(), stats));
        }
    }

    // Display results grouped by category
    let category_order = vec![
        "AsyncAuth",
        "AsyncDatabase",
        "AsyncHTTP",
        "AsyncMessage",
        "AsyncProcessing",
        "AsyncCoordination",
    ];

    for category_name in category_order {
        if let Some(ops) = categories.get(category_name) {
            println!("\n🔄 {} Category:", category_name);
            let mut total_calls = 0;
            let mut total_time = Duration::ZERO;

            for (op_name, stats) in ops {
                println!(
                    "   {} - {} calls, avg: {:?}",
                    op_name,
                    stats.count,
                    stats.mean()
                );
                total_calls += stats.count;
                total_time += stats.total;
            }

            println!(
                "   📊 Category Total: {} calls, {:?} total time",
                total_calls, total_time
            );
        }
    }

    println!("\n{}", "=".repeat(70));
    println!("TOP ASYNC OPERATIONS");
    println!("{}", "=".repeat(70));

    let all_stats = ProfileCollector::get_all_stats();

    // Top by total time
    let mut sorted_by_total: Vec<_> = all_stats.iter().collect();
    sorted_by_total.sort_by(|a, b| b.1.total.cmp(&a.1.total));

    println!("\n⏱️  Most Time Consuming Async Operations (Top 5):");
    for (i, (name, stats)) in sorted_by_total.iter().take(5).enumerate() {
        println!(
            "  {}. {} - {:?} total ({} calls)",
            i + 1,
            name,
            stats.total,
            stats.count
        );
    }

    // Top by call count
    let mut sorted_by_count: Vec<_> = all_stats.iter().collect();
    sorted_by_count.sort_by(|a, b| b.1.count.cmp(&a.1.count));

    println!("\n📈 Most Frequently Called Async Operations (Top 5):");
    for (i, (name, stats)) in sorted_by_count.iter().take(5).enumerate() {
        println!(
            "  {}. {} - {} calls (avg: {:?})",
            i + 1,
            name,
            stats.count,
            stats.mean()
        );
    }

    // Top by average time
    let mut sorted_by_avg: Vec<_> = all_stats.iter().collect();
    sorted_by_avg.sort_by(|a, b| b.1.mean().cmp(&a.1.mean()));

    println!("\n⚡ Highest Average Latency Async Operations (Top 5):");
    for (i, (name, stats)) in sorted_by_avg.iter().take(5).enumerate() {
        println!(
            "  {}. {} - avg: {:?} ({} calls)",
            i + 1,
            name,
            stats.mean(),
            stats.count
        );
    }
}

async fn simulate_async_work(millis: u64) {
    sleep(Duration::from_millis(millis)).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_async_operation_categories() {
        let auth_op = AsyncOperation::AuthenticateUser;
        assert_eq!(auth_op.get_category().get_name(), "AsyncAuth");
        assert_eq!(auth_op.to_str(), "authenticate_user");

        let db_op = AsyncOperation::FetchUserProfile;
        assert_eq!(db_op.get_category().get_name(), "AsyncDatabase");
        assert_eq!(db_op.to_str(), "fetch_user_profile");

        let http_op = AsyncOperation::EnrichWithExternalData;
        assert_eq!(http_op.get_category().get_name(), "AsyncHTTP");
        assert_eq!(http_op.to_str(), "enrich_with_external_data");
    }

    #[test]
    fn test_async_category_properties() {
        let auth_cat = AsyncAuthCategory;
        assert_eq!(auth_cat.get_name(), "AsyncAuth");
        assert_eq!(auth_cat.priority(), 5);
        assert!(auth_cat.color_hint().is_some());

        let db_cat = AsyncDatabaseCategory;
        assert_eq!(db_cat.get_name(), "AsyncDatabase");
        assert_eq!(db_cat.priority(), 1);
    }

    #[tokio::test]
    async fn test_async_profiling_basic() {
        ProfileCollector::clear_all();

        let op = AsyncOperation::AuthenticateUser;
        let result = profile_async!(op, async {
            sleep(Duration::from_millis(1)).await;
            "authenticated"
        });

        assert_eq!(result, "authenticated");
        assert!(ProfileCollector::has_data());
        let stats = ProfileCollector::get_stats("AsyncAuth::authenticate_user");
        assert!(stats.is_some());
        assert_eq!(stats.unwrap().count, 1);
    }

    #[tokio::test]
    async fn test_concurrent_async_profiling() {
        ProfileCollector::clear_all();

        let op1 = AsyncOperation::FetchUserProfile;
        let op2 = AsyncOperation::FetchUserPreferences;

        let (result1, result2) = tokio::join!(
            profile_async!(op1, async {
                sleep(Duration::from_millis(1)).await;
                "profile"
            }),
            profile_async!(op2, async {
                sleep(Duration::from_millis(1)).await;
                "preferences"
            })
        );

        assert_eq!(result1, "profile");
        assert_eq!(result2, "preferences");
        assert!(ProfileCollector::has_data());
        assert_eq!(ProfileCollector::total_operations(), 2);
    }
}
