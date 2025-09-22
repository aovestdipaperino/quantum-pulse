//! Example demonstrating custom categories with enum operations using Operation trait

use quantum_pulse::{profile, profile_async, Category, Operation, ProfileCollector};
use std::thread;
use std::time::Duration;
use tokio::time::sleep;

/// Web application operations that implement Operation trait
#[derive(Debug)]
enum WebAppOperation {
    // Authentication operations
    CheckAuthToken,
    ValidatePermissions,

    // Database operations
    FetchUserData,
    UpdateUserProfile,
    VacuumDatabase,

    // Cache operations
    CheckCache,
    UpdateCache,
    WarmCache,

    // Business logic operations
    CalculateRecommendations,
    ProcessPayment,

    // External API operations
    PaymentGatewayApi,
    SyncWithCrm,
    SendNotification,

    // Serialization operations
    SerializeResponse,
    ParseRequest,

    // File I/O operations
    WriteAccessLog,
    ProcessUploadedFile,
    BackupData,
}

/// Database category for all database operations
#[derive(Debug)]
struct DatabaseCategory;

impl Category for DatabaseCategory {
    fn get_name(&self) -> &str {
        "Database"
    }

    fn get_description(&self) -> &str {
        "All database queries, transactions, and maintenance operations"
    }

    fn color_hint(&self) -> Option<&str> {
        Some("#FF6B6B")
    }

    fn priority(&self) -> i32 {
        1
    }
}

/// External API category for third-party service calls
#[derive(Debug)]
struct ExternalApiCategory;

impl Category for ExternalApiCategory {
    fn get_name(&self) -> &str {
        "ExternalAPI"
    }

    fn get_description(&self) -> &str {
        "Calls to third-party services and external APIs"
    }

    fn color_hint(&self) -> Option<&str> {
        Some("#4ECDC4")
    }

    fn priority(&self) -> i32 {
        2
    }
}

/// Cache category for caching operations
#[derive(Debug)]
struct CacheCategory;

impl Category for CacheCategory {
    fn get_name(&self) -> &str {
        "Cache"
    }

    fn get_description(&self) -> &str {
        "Redis, Memcached, and other caching operations"
    }

    fn color_hint(&self) -> Option<&str> {
        Some("#45B7D1")
    }

    fn priority(&self) -> i32 {
        3
    }
}

/// Business logic category for core application logic
#[derive(Debug)]
struct BusinessLogicCategory;

impl Category for BusinessLogicCategory {
    fn get_name(&self) -> &str {
        "BusinessLogic"
    }

    fn get_description(&self) -> &str {
        "Core application business logic and computations"
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
struct AuthCategory;

impl Category for AuthCategory {
    fn get_name(&self) -> &str {
        "Authentication"
    }

    fn get_description(&self) -> &str {
        "Authentication, authorization, and security operations"
    }

    fn color_hint(&self) -> Option<&str> {
        Some("#DDA0DD")
    }

    fn priority(&self) -> i32 {
        5
    }
}

/// Serialization category for data transformation
#[derive(Debug)]
struct SerializationCategory;

impl Category for SerializationCategory {
    fn get_name(&self) -> &str {
        "Serialization"
    }

    fn get_description(&self) -> &str {
        "JSON, XML parsing and data serialization operations"
    }

    fn color_hint(&self) -> Option<&str> {
        Some("#FFEAA7")
    }

    fn priority(&self) -> i32 {
        6
    }
}

/// File I/O category for file operations
#[derive(Debug)]
struct FileIOCategory;

impl Category for FileIOCategory {
    fn get_name(&self) -> &str {
        "FileIO"
    }

    fn get_description(&self) -> &str {
        "File system read/write operations and logging"
    }

    fn color_hint(&self) -> Option<&str> {
        Some("#F4A460")
    }

    fn priority(&self) -> i32 {
        7
    }
}

impl Operation for WebAppOperation {
    fn get_category(&self) -> &dyn Category {
        match self {
            WebAppOperation::FetchUserData
            | WebAppOperation::UpdateUserProfile
            | WebAppOperation::VacuumDatabase => &DatabaseCategory,

            WebAppOperation::PaymentGatewayApi
            | WebAppOperation::SyncWithCrm
            | WebAppOperation::SendNotification => &ExternalApiCategory,

            WebAppOperation::CheckCache
            | WebAppOperation::UpdateCache
            | WebAppOperation::WarmCache => &CacheCategory,

            WebAppOperation::CalculateRecommendations | WebAppOperation::ProcessPayment => {
                &BusinessLogicCategory
            }

            WebAppOperation::CheckAuthToken | WebAppOperation::ValidatePermissions => &AuthCategory,

            WebAppOperation::SerializeResponse | WebAppOperation::ParseRequest => {
                &SerializationCategory
            }

            WebAppOperation::WriteAccessLog
            | WebAppOperation::ProcessUploadedFile
            | WebAppOperation::BackupData => &FileIOCategory,
        }
    }

    fn to_str(&self) -> String {
        match self {
            WebAppOperation::CheckAuthToken => "check_auth_token".to_string(),
            WebAppOperation::ValidatePermissions => "validate_permissions".to_string(),
            WebAppOperation::FetchUserData => "fetch_user_data".to_string(),
            WebAppOperation::UpdateUserProfile => "update_user_profile".to_string(),
            WebAppOperation::VacuumDatabase => "vacuum_database".to_string(),
            WebAppOperation::CheckCache => "check_cache".to_string(),
            WebAppOperation::UpdateCache => "update_cache".to_string(),
            WebAppOperation::WarmCache => "warm_cache".to_string(),
            WebAppOperation::CalculateRecommendations => "calculate_recommendations".to_string(),
            WebAppOperation::ProcessPayment => "process_payment".to_string(),
            WebAppOperation::PaymentGatewayApi => "payment_gateway_api".to_string(),
            WebAppOperation::SyncWithCrm => "sync_with_crm".to_string(),
            WebAppOperation::SendNotification => "send_notification".to_string(),
            WebAppOperation::SerializeResponse => "serialize_response".to_string(),
            WebAppOperation::ParseRequest => "parse_request".to_string(),
            WebAppOperation::WriteAccessLog => "write_access_log".to_string(),
            WebAppOperation::ProcessUploadedFile => "process_uploaded_file".to_string(),
            WebAppOperation::BackupData => "backup_data".to_string(),
        }
    }
}

#[tokio::main]
async fn main() {
    println!("=== Custom Categories Example - Web Application ===\n");

    // Clear any existing data
    ProfileCollector::clear_all();

    // Simulate various web application operations
    simulate_web_requests().await;

    // Use UpdateUserProfile to avoid unused warning
    if false {
        profile!(WebAppOperation::UpdateUserProfile, {
            println!("Updating user profile");
        });
    }

    // Generate and display categorized report
    show_profiling_results();
}

async fn simulate_web_requests() {
    println!("Simulating web application operations...\n");

    // Simulate multiple user requests
    for request_id in 1..=3 {
        println!("Processing request #{}...", request_id);
        handle_user_request(request_id).await;
    }

    // Simulate background jobs
    println!("\nRunning background jobs...");
    run_background_jobs().await;
}

async fn handle_user_request(request_id: u32) {
    // Authentication
    let auth_op = WebAppOperation::CheckAuthToken;
    profile!(auth_op, {
        simulate_work(5 + (request_id % 3) as u64);
    });

    // Validate permissions
    let perm_op = WebAppOperation::ValidatePermissions;
    profile!(perm_op, {
        simulate_work(3);
    });

    // Parse incoming request
    let parse_op = WebAppOperation::ParseRequest;
    profile!(parse_op, {
        simulate_work(2);
    });

    // Database query for user data
    let fetch_op = WebAppOperation::FetchUserData;
    let _user_data = profile!(fetch_op, {
        simulate_work(20 + (request_id * 2) as u64);
        format!("User_{}", request_id)
    });

    // Check cache for computed results
    let cache_check_op = WebAppOperation::CheckCache;
    let cached = profile!(cache_check_op, {
        simulate_work(2);
        request_id % 3 == 0 // Some requests hit cache
    });

    if !cached {
        // Business logic processing
        let rec_op = WebAppOperation::CalculateRecommendations;
        profile!(rec_op, {
            simulate_work(50 + (request_id * 5) as u64);
        });

        // Store in cache
        let cache_update_op = WebAppOperation::UpdateCache;
        profile!(cache_update_op, {
            simulate_work(3);
        });
    }

    // External API call (e.g., payment processing)
    if request_id % 2 == 0 {
        let payment_op = WebAppOperation::PaymentGatewayApi;
        profile_async!(payment_op, async {
            sleep(Duration::from_millis(80 + (request_id * 10) as u64)).await;
        })
        .await;
    }

    // Process payment (business logic)
    if request_id == 2 {
        let process_payment_op = WebAppOperation::ProcessPayment;
        profile!(process_payment_op, {
            simulate_work(30);
        });
    }

    // Serialize response
    let serialize_op = WebAppOperation::SerializeResponse;
    profile!(serialize_op, {
        simulate_work(8);
    });

    // Log to file
    let log_op = WebAppOperation::WriteAccessLog;
    profile!(log_op, {
        simulate_work(5);
    });
}

async fn run_background_jobs() {
    // Database maintenance
    let vacuum_op = WebAppOperation::VacuumDatabase;
    profile!(vacuum_op, {
        simulate_work(200);
        println!("  - Database maintenance completed");
    });

    // Cache warming
    let warm_op = WebAppOperation::WarmCache;
    profile_async!(warm_op, async {
        sleep(Duration::from_millis(50)).await;
        println!("  - Cache warmed");
    })
    .await;

    // File processing
    for i in 1..=2 {
        let file_op = WebAppOperation::ProcessUploadedFile;
        profile!(file_op, {
            simulate_work(30);
            println!("  - Processed uploaded file #{}", i);
        });
    }

    // External API sync
    let sync_op = WebAppOperation::SyncWithCrm;
    profile_async!(sync_op, async {
        sleep(Duration::from_millis(120)).await;
        println!("  - CRM sync completed");
    })
    .await;

    // Send notifications
    let notify_op = WebAppOperation::SendNotification;
    profile_async!(notify_op, async {
        sleep(Duration::from_millis(15)).await;
        println!("  - Notifications sent");
    })
    .await;

    // Backup data
    let backup_op = WebAppOperation::BackupData;
    profile!(backup_op, {
        simulate_work(100);
        println!("  - Data backup completed");
    });
}

fn show_profiling_results() {
    println!("\n{}", "=".repeat(70));
    println!("PROFILING RESULTS BY CATEGORY");
    println!("{}", "=".repeat(70));

    ProfileCollector::report_stats();

    let summary = ProfileCollector::get_summary();
    println!("\nSUMMARY:");
    println!("- Total operations: {}", summary.total_operations);
    println!("- Unique operations: {}", summary.unique_operations);
    println!("- Total time: {}μs", summary.total_time_micros);

    println!("\n{}", "=".repeat(70));
    println!("DETAILED ANALYSIS BY CATEGORY");
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
        "Authentication",
        "Database",
        "Cache",
        "BusinessLogic",
        "ExternalAPI",
        "Serialization",
        "FileIO",
    ];

    for category_name in category_order {
        if let Some(ops) = categories.get(category_name) {
            println!("\n📂 {} Category:", category_name);
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
    println!("TOP OPERATIONS");
    println!("{}", "=".repeat(70));

    let all_stats = ProfileCollector::get_all_stats();

    // Top by total time
    let mut sorted_by_total: Vec<_> = all_stats.iter().collect();
    sorted_by_total.sort_by(|a, b| b.1.total.cmp(&a.1.total));

    println!("\n⏱️  Most Time Consuming (Top 5):");
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

    println!("\n📈 Most Frequently Called (Top 5):");
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

    println!("\n⚡ Highest Average Latency (Top 5):");
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

fn simulate_work(millis: u64) {
    thread::sleep(Duration::from_millis(millis));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web_app_operation_categories() {
        let db_op = WebAppOperation::FetchUserData;
        assert_eq!(db_op.get_category().get_name(), "Database");
        assert_eq!(db_op.to_str(), "fetch_user_data");

        let api_op = WebAppOperation::PaymentGatewayApi;
        assert_eq!(api_op.get_category().get_name(), "ExternalAPI");
        assert_eq!(api_op.to_str(), "payment_gateway_api");

        let auth_op = WebAppOperation::CheckAuthToken;
        assert_eq!(auth_op.get_category().get_name(), "Authentication");
        assert_eq!(auth_op.to_str(), "check_auth_token");
    }

    #[test]
    fn test_category_properties() {
        let db_cat = DatabaseCategory;
        assert_eq!(db_cat.get_name(), "Database");
        assert_eq!(db_cat.priority(), 1);
        assert!(db_cat.color_hint().is_some());

        let ext_cat = ExternalApiCategory;
        assert_eq!(ext_cat.get_name(), "ExternalAPI");
        assert_eq!(ext_cat.priority(), 2);
    }

    #[test]
    fn test_profiling_with_categories() {
        ProfileCollector::clear_all();

        let op = WebAppOperation::FetchUserData;
        profile!(op, {
            std::thread::sleep(Duration::from_millis(1));
        });

        assert!(ProfileCollector::has_data());
        let stats = ProfileCollector::get_stats("Database::fetch_user_data");
        assert!(stats.is_some());
        assert_eq!(stats.unwrap().count, 1);
    }

    #[tokio::test]
    async fn test_async_profiling_with_categories() {
        ProfileCollector::clear_all();

        let op = WebAppOperation::PaymentGatewayApi;
        profile_async!(op, async {
            sleep(Duration::from_millis(1)).await;
        })
        .await;

        assert!(ProfileCollector::has_data());
        let stats = ProfileCollector::get_stats("ExternalAPI::payment_gateway_api");
        assert!(stats.is_some());
        assert_eq!(stats.unwrap().count, 1);
    }
}
