//! Example demonstrating custom categories for organizing profiling data

use quantum_pulse::{profile, Category, ProfileReport, Profiler, ReportBuilder};
use std::thread;
use std::time::Duration;

/// Specific web application operations for profiling
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum WebAppOperation {
    // Authentication operations
    CheckAuthToken,

    // Database operations
    FetchUserData,
    VacuumDatabase,

    // Cache operations
    CheckCache,
    UpdateCache,
    WarmCache,

    // Business logic operations
    CalculateRecommendations,

    // External API operations
    PaymentGatewayApi,
    SyncWithCrm,

    // Serialization operations
    SerializeResponse,

    // File I/O operations
    WriteAccessLog,
    ProcessUploadedFiles,
    ProcessFile,
}

/// Custom categories for organizing operations
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum WebAppCategory {
    Database,
    ExternalApi,
    Cache,
    BusinessLogic,
    Serialization,
    Authentication,
    FileIO,
}

impl Category for WebAppOperation {
    fn description(&self) -> Option<&str> {
        match self {
            WebAppOperation::CheckAuthToken => Some("Validate authentication tokens"),
            WebAppOperation::FetchUserData => Some("Retrieve user data from database"),
            WebAppOperation::VacuumDatabase => Some("Database maintenance and cleanup"),
            WebAppOperation::CheckCache => Some("Check cache for existing data"),
            WebAppOperation::UpdateCache => Some("Update cache with new data"),
            WebAppOperation::WarmCache => Some("Pre-populate cache with common data"),
            WebAppOperation::CalculateRecommendations => Some("Generate user recommendations"),
            WebAppOperation::PaymentGatewayApi => Some("Process payments via external API"),
            WebAppOperation::SyncWithCrm => Some("Synchronize data with CRM system"),
            WebAppOperation::SerializeResponse => Some("Serialize response data to JSON"),
            WebAppOperation::WriteAccessLog => Some("Write request logs to file"),
            WebAppOperation::ProcessUploadedFiles => Some("Process batch of uploaded files"),
            WebAppOperation::ProcessFile => Some("Process individual file"),
        }
    }

    fn priority(&self) -> i32 {
        match self {
            // Critical path operations
            WebAppOperation::CheckAuthToken => 1,
            WebAppOperation::SerializeResponse => 1,

            // Core business logic
            WebAppOperation::FetchUserData => 2,
            WebAppOperation::CalculateRecommendations => 2,

            // Cache operations
            WebAppOperation::CheckCache | WebAppOperation::UpdateCache => 3,

            // External dependencies
            WebAppOperation::PaymentGatewayApi | WebAppOperation::SyncWithCrm => 4,

            // Background operations
            WebAppOperation::VacuumDatabase | WebAppOperation::WarmCache => 5,
            WebAppOperation::ProcessUploadedFiles | WebAppOperation::ProcessFile => 5,
            WebAppOperation::WriteAccessLog => 6,
        }
    }
}

impl WebAppOperation {
    fn category(&self) -> WebAppCategory {
        match self {
            WebAppOperation::CheckAuthToken => WebAppCategory::Authentication,
            WebAppOperation::FetchUserData | WebAppOperation::VacuumDatabase => {
                WebAppCategory::Database
            }
            WebAppOperation::CheckCache
            | WebAppOperation::UpdateCache
            | WebAppOperation::WarmCache => WebAppCategory::Cache,
            WebAppOperation::CalculateRecommendations => WebAppCategory::BusinessLogic,
            WebAppOperation::PaymentGatewayApi | WebAppOperation::SyncWithCrm => {
                WebAppCategory::ExternalApi
            }
            WebAppOperation::SerializeResponse => WebAppCategory::Serialization,
            WebAppOperation::WriteAccessLog
            | WebAppOperation::ProcessUploadedFiles
            | WebAppOperation::ProcessFile => WebAppCategory::FileIO,
        }
    }
}

type WebAppProfiler = Profiler<WebAppOperation>;

impl Category for WebAppCategory {
    fn description(&self) -> Option<&str> {
        match self {
            WebAppCategory::Database => Some("All database queries and transactions"),
            WebAppCategory::ExternalApi => Some("Calls to third-party services"),
            WebAppCategory::Cache => Some("Redis/Memcached operations"),
            WebAppCategory::BusinessLogic => Some("Core application logic"),
            WebAppCategory::Serialization => Some("JSON/XML parsing and generation"),
            WebAppCategory::Authentication => Some("User authentication and permission checks"),
            WebAppCategory::FileIO => Some("File system read/write operations"),
        }
    }

    fn color_hint(&self) -> Option<&str> {
        match self {
            WebAppCategory::Database => Some("#FF6B6B"),       // Red
            WebAppCategory::ExternalApi => Some("#4ECDC4"),    // Teal
            WebAppCategory::Cache => Some("#45B7D1"),          // Light Blue
            WebAppCategory::BusinessLogic => Some("#96CEB4"),  // Green
            WebAppCategory::Serialization => Some("#FFEAA7"),  // Yellow
            WebAppCategory::Authentication => Some("#DDA0DD"), // Plum
            WebAppCategory::FileIO => Some("#F4A460"),         // Sandy Brown
        }
    }

    fn priority(&self) -> i32 {
        match self {
            WebAppCategory::Database => 1,
            WebAppCategory::ExternalApi => 2,
            WebAppCategory::Cache => 3,
            WebAppCategory::BusinessLogic => 4,
            WebAppCategory::Authentication => 5,
            WebAppCategory::Serialization => 6,
            WebAppCategory::FileIO => 7,
        }
    }
}

impl std::fmt::Display for WebAppCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            WebAppCategory::Database => "Database Operations",
            WebAppCategory::ExternalApi => "External API Calls",
            WebAppCategory::Cache => "Cache Operations",
            WebAppCategory::BusinessLogic => "Business Logic",
            WebAppCategory::Serialization => "Data Serialization",
            WebAppCategory::Authentication => "Authentication & Authorization",
            WebAppCategory::FileIO => "File I/O Operations",
        };
        write!(f, "{}", name)
    }
}

fn main() {
    println!("=== Custom Categories Example - Web Application ===\n");

    // Simulate various web application operations
    simulate_web_requests();

    // Generate and display categorized report
    let report = WebAppProfiler::report();
    println!("\n{:#?}", report);

    // Show top operations by different metrics
    show_top_operations(&report);

    // Export to different formats
    export_reports(&report);
}

fn simulate_web_requests() {
    println!("Simulating web application operations...\n");

    // Simulate multiple user requests
    for request_id in 1..=5 {
        println!("Processing request #{}...", request_id);
        handle_user_request(request_id);
    }

    // Simulate background jobs
    println!("\nRunning background jobs...");
    run_background_jobs();
}

fn handle_user_request(request_id: u32) {
    // Authentication
    profile!(WebAppOperation::CheckAuthToken => {
        simulate_work(5 + (request_id % 3) as u64);
    });

    // Database query for user data
    let _user_data = profile!(WebAppOperation::FetchUserData => {
        simulate_work(20 + (request_id * 2) as u64);
        format!("User_{}", request_id)
    });

    // Check cache for computed results
    let cached = profile!(WebAppOperation::CheckCache => {
        simulate_work(2);
        request_id % 3 == 0 // Some requests hit cache
    });

    if !cached {
        // Business logic processing
        profile!(WebAppOperation::CalculateRecommendations => {
            simulate_work(50 + (request_id * 5) as u64);
        });

        // Store in cache
        profile!(WebAppOperation::UpdateCache => {
            simulate_work(3);
        });
    }

    // External API call (e.g., payment processing)
    if request_id % 2 == 0 {
        profile!(WebAppOperation::PaymentGatewayApi => {
            simulate_work(100 + (request_id * 10) as u64);
        });
    }

    // Serialize response
    profile!(WebAppOperation::SerializeResponse => {
        simulate_work(8);
    });

    // Log to file
    profile!(WebAppOperation::WriteAccessLog => {
        simulate_work(5);
    });
}

fn run_background_jobs() {
    // Database maintenance
    profile!(WebAppOperation::VacuumDatabase => {
        simulate_work(200);
        println!("  - Database maintenance completed");
    });

    // Cache warming
    profile!(WebAppOperation::WarmCache => {
        simulate_work(50);
        println!("  - Cache warmed");
    });

    // Batch file processing
    profile!(WebAppOperation::ProcessUploadedFiles => {
        for _i in 1..=3 {
            profile!(WebAppOperation::ProcessFile => {
                simulate_work(30);
            });
        }
        println!("  - Processed 3 uploaded files");
    });

    // External API sync
    profile!(WebAppOperation::SyncWithCrm => {
        simulate_work(150);
        println!("  - CRM sync completed");
    });
}

fn show_top_operations(_report: &ProfileReport<WebAppOperation>) {
    println!("\n{}", "=".repeat(70));
    println!("TOP OPERATIONS ANALYSIS");
    println!("{}", "=".repeat(70));

    // Get all stats for manual sorting
    let all_stats = WebAppProfiler::get_all_stats();

    // Top 3 by call count (using manual sorting since Count variant may not exist)
    println!("\n📊 Most Frequently Called (Top 3):");
    let mut sorted_by_count: Vec<_> = all_stats.iter().collect();
    sorted_by_count.sort_by(|a, b| b.1.count.cmp(&a.1.count));
    for (i, (name, stats)) in sorted_by_count.iter().take(3).enumerate() {
        println!("  {}. {} - {} calls", i + 1, name, stats.count);
    }

    // Top 3 by total time
    println!("\n📊 Most Time Consuming (Total):");
    let mut sorted_by_total: Vec<_> = all_stats.iter().collect();
    sorted_by_total.sort_by(|a, b| b.1.total_micros.partial_cmp(&a.1.total_micros).unwrap());

    for (i, (name, stats)) in sorted_by_total.iter().take(3).enumerate() {
        println!(
            "  {}. {} - {:.1} μs total ({} calls)",
            i + 1,
            name,
            stats.total_micros,
            stats.count
        );
    }

    println!("\n⚡ Highest Average Latency:");
    let mut sorted_by_avg: Vec<_> = all_stats.iter().collect();
    sorted_by_avg.sort_by(|a, b| b.1.mean_micros.partial_cmp(&a.1.mean_micros).unwrap());

    for (i, (name, stats)) in sorted_by_avg.iter().take(3).enumerate() {
        println!("  {}. {} - {:.1} μs avg", i + 1, name, stats.mean_micros);
    }

    println!("\n📈 Most Frequent:");
    let mut sorted_by_count: Vec<_> = all_stats.iter().collect();
    sorted_by_count.sort_by(|a, b| b.1.count.cmp(&a.1.count));

    for (i, (name, stats)) in sorted_by_count.iter().take(3).enumerate() {
        println!("  {}. {} - {} calls", i + 1, name, stats.count);
    }
}

fn export_reports(_report: &ProfileReport<WebAppOperation>) {
    println!("\n{}", "=".repeat(70));
    println!("EXPORT OPTIONS");
    println!("{}", "=".repeat(70));

    // Simple CSV-like export (no external CSV dependency needed)
    println!("\n📄 CSV-like Export (first 5 operations):");
    println!("  Operation,Count,Mean(μs),Min(μs),Max(μs)");
    let all_stats = WebAppProfiler::get_all_stats();
    for (name, stats) in all_stats.iter().take(5) {
        println!(
            "  {},{},{:.1},{},{}",
            name, stats.count, stats.mean_micros, stats.min_micros, stats.max_micros
        );
    }
    println!("  ...");

    // Simple JSON-like export (no external JSON dependency needed)
    println!("\n📋 JSON-like Export:");
    println!("  {{");
    println!("    \"operations\": [");
    for (i, (name, stats)) in all_stats.iter().take(3).enumerate() {
        let comma = if i < 2 { "," } else { "" };
        println!(
            "      {{\"name\": \"{}\", \"count\": {}, \"avg_micros\": {:.1}}}{}",
            name, stats.count, stats.mean_micros, comma
        );
    }
    println!("    ]");
    println!("  }}");

    // Custom report with different configuration
    println!("\n📊 Custom Report Configuration:");
    let custom_report = ReportBuilder::<WebAppOperation>::new()
        .include_percentiles(false)
        .time_format(quantum_pulse::TimeFormat::Milliseconds)
        .build();

    println!("{:#?}", custom_report);
}

fn simulate_work(millis: u64) {
    thread::sleep(Duration::from_millis(millis));
}
