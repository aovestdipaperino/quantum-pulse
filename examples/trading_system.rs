//! Example showing how a high-frequency trading system integrates with Quantum Pulse
//! This demonstrates microsecond-precision profiling for latency-sensitive applications

use quantum_pulse::{profile, Category, ProfileReport, Profiler, ReportBuilder, TimeFormat};
use std::thread;
use std::time::Duration;

/// Trading system profiling categories for different operation types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum TradingOperation {
    // Order Processing
    OrderValidation,
    OrderSubmission,
    OrderExecution,
    OrderConfirmation,

    // Market Data Processing
    PriceFeedParsing,
    MarketDataUpdate,
    VolatilityCalculation,

    // Risk Management
    RiskAssessment,
    PositionUpdate,
    ExposureCalculation,

    // Portfolio Management
    PnlCalculation,
    RebalanceOperation,

    // External Communications
    ExchangeConnection,
    MarketDataFeed,
}

impl Category for TradingOperation {
    fn description(&self) -> Option<&str> {
        match self {
            TradingOperation::OrderValidation => Some("Validating incoming trading orders"),
            TradingOperation::OrderSubmission => Some("Submitting orders to exchanges"),
            TradingOperation::OrderExecution => Some("Executing trades in the market"),
            TradingOperation::OrderConfirmation => Some("Confirming order execution status"),
            TradingOperation::PriceFeedParsing => Some("Parsing real-time price feeds"),
            TradingOperation::MarketDataUpdate => Some("Updating internal market data"),
            TradingOperation::VolatilityCalculation => {
                Some("Calculating market volatility metrics")
            }
            TradingOperation::RiskAssessment => Some("Assessing trading risk parameters"),
            TradingOperation::PositionUpdate => Some("Updating portfolio positions"),
            TradingOperation::ExposureCalculation => Some("Calculating market exposure"),
            TradingOperation::PnlCalculation => Some("Calculating profit and loss"),
            TradingOperation::RebalanceOperation => Some("Rebalancing portfolio allocations"),
            TradingOperation::ExchangeConnection => Some("Managing exchange connections"),
            TradingOperation::MarketDataFeed => Some("Processing market data feeds"),
        }
    }

    fn priority(&self) -> i32 {
        match self {
            // Critical path - order lifecycle (highest priority)
            TradingOperation::OrderValidation | TradingOperation::OrderSubmission => 1,
            TradingOperation::OrderExecution | TradingOperation::OrderConfirmation => 1,

            // Market data processing (high priority)
            TradingOperation::PriceFeedParsing | TradingOperation::MarketDataUpdate => 2,

            // Risk management (high priority)
            TradingOperation::RiskAssessment | TradingOperation::ExposureCalculation => 2,

            // Portfolio management (medium priority)
            TradingOperation::PositionUpdate | TradingOperation::PnlCalculation => 3,
            TradingOperation::RebalanceOperation => 4,

            // Market analytics (lower priority)
            TradingOperation::VolatilityCalculation => 4,

            // Infrastructure (background)
            TradingOperation::ExchangeConnection | TradingOperation::MarketDataFeed => 5,
        }
    }

    fn color_hint(&self) -> Option<&str> {
        match self {
            // Order operations - Red tones
            TradingOperation::OrderValidation | TradingOperation::OrderSubmission => {
                Some("#FF6B6B")
            }
            TradingOperation::OrderExecution | TradingOperation::OrderConfirmation => {
                Some("#FF8E53")
            }

            // Market data - Blue tones
            TradingOperation::PriceFeedParsing | TradingOperation::MarketDataUpdate => {
                Some("#4ECDC4")
            }
            TradingOperation::MarketDataFeed => Some("#45B7D1"),

            // Risk management - Orange tones
            TradingOperation::RiskAssessment | TradingOperation::ExposureCalculation => {
                Some("#FFA726")
            }

            // Portfolio - Green tones
            TradingOperation::PositionUpdate | TradingOperation::PnlCalculation => Some("#66BB6A"),
            TradingOperation::RebalanceOperation => Some("#96CEB4"),

            // Analytics - Purple tones
            TradingOperation::VolatilityCalculation => Some("#AB47BC"),

            // Infrastructure - Gray
            TradingOperation::ExchangeConnection => Some("#78909C"),
        }
    }
}

impl std::fmt::Display for TradingOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            TradingOperation::OrderValidation => "Order Validation",
            TradingOperation::OrderSubmission => "Order Submission",
            TradingOperation::OrderExecution => "Order Execution",
            TradingOperation::OrderConfirmation => "Order Confirmation",
            TradingOperation::PriceFeedParsing => "Price Feed Parsing",
            TradingOperation::MarketDataUpdate => "Market Data Update",
            TradingOperation::VolatilityCalculation => "Volatility Calculation",
            TradingOperation::RiskAssessment => "Risk Assessment",
            TradingOperation::PositionUpdate => "Position Update",
            TradingOperation::ExposureCalculation => "Exposure Calculation",
            TradingOperation::PnlCalculation => "P&L Calculation",
            TradingOperation::RebalanceOperation => "Portfolio Rebalance",
            TradingOperation::ExchangeConnection => "Exchange Connection",
            TradingOperation::MarketDataFeed => "Market Data Feed",
        };
        write!(f, "{}", name)
    }
}

/// Type alias for the trading system's profiler
type TradingProfiler = Profiler<TradingOperation>;

fn main() {
    println!("=== High-Frequency Trading System - Profiling Example ===\n");

    // Simulate a typical trading system workload
    simulate_trading_operations();

    // Generate and display the profiling report
    display_profiling_report();

    // Show how to get metrics for monitoring systems
    let metrics_json = get_metrics_for_monitoring();
    println!("\n📊 JSON Metrics (for monitoring):\n{}", metrics_json);

    println!("\n=== Trading System Profiling Complete ===");
}

/// Simulates typical high-frequency trading operations
fn simulate_trading_operations() {
    println!("Simulating trading system operations...\n");

    // Simulate market data processing
    for i in 1..=10 {
        process_market_data_tick(i);
    }

    // Simulate order processing
    for i in 1..=5 {
        process_trading_order(i);
    }

    // Run background portfolio management tasks
    run_portfolio_management();
}

fn process_market_data_tick(tick_id: u32) {
    // Parse incoming price feed
    TradingProfiler::time_with_category(
        &format!("market_tick_{}", tick_id),
        TradingOperation::MarketDataFeed,
        || {
            // Parse raw market data
            profile!(TradingOperation::PriceFeedParsing => {
                simulate_work(2 + (tick_id % 3) as u64); // 2-4 microseconds
            });

            // Update internal market state
            profile!(TradingOperation::MarketDataUpdate => {
                simulate_work(5 + (tick_id % 2) as u64); // 5-6 microseconds
            });

            // Calculate volatility if needed
            if tick_id % 5 == 0 {
                profile!(TradingOperation::VolatilityCalculation => {
                    simulate_work(15); // 15 microseconds for complex calculation
                    println!("  - Recalculated volatility metrics");
                });
            }
        },
    );
}

fn process_trading_order(order_id: u32) {
    println!("Processing trading order #{}", order_id);

    // Main order processing pipeline
    TradingProfiler::time_with_category(
        &format!("trading_order_{}", order_id),
        TradingOperation::OrderSubmission,
        || {
            // Validate incoming order
            profile!(TradingOperation::OrderValidation => {
                simulate_work(8); // 8 microseconds validation
            });

            // Assess risk for this order
            profile!(TradingOperation::RiskAssessment => {
                simulate_work(12 + (order_id * 2) as u64); // Variable risk calculation
            });

            // Calculate exposure impact
            profile!(TradingOperation::ExposureCalculation => {
                simulate_work(6);
            });

            // Execute the order
            let execution_time = if order_id == 3 {
                // Simulate a slower execution (market conditions)
                profile!(TradingOperation::OrderExecution => {
                    simulate_work(45);
                    println!("  - Order executed with higher latency due to market conditions");
                });
                45
            } else {
                profile!(TradingOperation::OrderExecution => {
                    simulate_work(20 + (order_id * 3) as u64);
                });
                20 + (order_id * 3) as u64
            };

            // Confirm execution
            profile!(TradingOperation::OrderConfirmation => {
                simulate_work(4);
            });

            // Update positions
            profile!(TradingOperation::PositionUpdate => {
                simulate_work(7);
            });

            println!("  - Order #{} executed in {}μs", order_id, execution_time);
        },
    );
}

fn run_portfolio_management() {
    println!("\nRunning portfolio management tasks...");

    // Calculate P&L for all positions
    profile!(TradingOperation::PnlCalculation => {
        simulate_work(80);
        println!("  - Updated P&L calculations");
    });

    // Check if rebalancing is needed
    profile!(TradingOperation::RebalanceOperation => {
        simulate_work(120);
        println!("  - Completed portfolio rebalancing analysis");
    });

    // Maintain exchange connections
    profile!(TradingOperation::ExchangeConnection => {
        simulate_work(30);
        println!("  - Verified exchange connection health");
    });
}

fn display_profiling_report() {
    println!("\n{}", "=".repeat(70));
    println!("TRADING SYSTEM PROFILING REPORT");
    println!("{}\n", "=".repeat(70));

    // Generate categorized report
    let report = ReportBuilder::<TradingOperation>::new()
        .group_by_category(true)
        .include_percentiles(true)
        .time_format(TimeFormat::Auto)
        .sort_by_time(true)
        .build();

    println!("{:#?}", report);

    // Show top operations analysis
    show_top_operations(&report);

    // Show detailed statistics for critical operations
    show_critical_operations_analysis();
}

fn show_top_operations(report: &ProfileReport<TradingOperation>) {
    use quantum_pulse::SortMetric;

    println!("\n{}", "=".repeat(70));
    println!("PERFORMANCE ANALYSIS");
    println!("{}", "=".repeat(70));

    println!("\n📊 Most Time Consuming Operations:");
    let top_by_time = report.top_operations_by(SortMetric::TotalTime, 5);
    for (i, (name, stats)) in top_by_time.iter().enumerate() {
        println!(
            "{}. {} - {:.1}μs total ({} calls, {:.1}μs avg)",
            i + 1,
            name,
            stats.total_micros as f64,
            stats.count,
            stats.mean_micros
        );
    }

    println!("\n⚡ Highest Latency Operations:");
    let top_by_latency = report.top_operations_by(SortMetric::MeanTime, 5);
    for (i, (name, stats)) in top_by_latency.iter().enumerate() {
        println!(
            "{}. {} - {:.1}μs avg ({:.1}μs max)",
            i + 1,
            name,
            stats.mean_micros,
            stats.max_micros as f64
        );
    }
}

fn show_critical_operations_analysis() {
    println!("\n{}", "=".repeat(70));
    println!("CRITICAL PATH ANALYSIS");
    println!("{}", "=".repeat(70));

    // Focus on order processing operations (critical path)
    let critical_operations = [
        "OrderValidation",
        "OrderSubmission",
        "OrderExecution",
        "RiskAssessment",
    ];

    for op_name in &critical_operations {
        if let Some(stats) = TradingProfiler::get_stats(op_name) {
            println!("\n🎯 {}:", op_name);
            println!("   Calls: {}", stats.count);
            println!("   Mean: {:.1}μs", stats.mean_micros);
            println!("   Min: {:.1}μs", stats.min_micros as f64);
            println!("   Max: {:.1}μs", stats.max_micros as f64);

            // Alert for operations that might be too slow for HFT
            if stats.mean_micros > 50 {
                println!("   ⚠️  WARNING: Average latency >50μs may impact trading performance");
            }
        }
    }
}

fn get_metrics_for_monitoring() -> String {
    // Try to export as JSON first
    // Simple JSON-like format (no external JSON dependency needed)
    {
        // Fallback to simple format
        let stats = TradingProfiler::get_all_stats();
        let mut output = String::from("{\n");
        output.push_str("  \"operations\": [\n");

        for (i, (name, stat)) in stats.iter().enumerate() {
            if i > 0 {
                output.push_str(",\n");
            }
            output.push_str(&format!(
                "    {{\"name\": \"{}\", \"avg_micros\": {:.1}, \"count\": {}}}",
                name, stat.mean_micros, stat.count
            ));
        }

        output.push_str("\n  ]\n}");
        output
    }
}

/// Simulates work by sleeping for the specified number of microseconds
/// In a real trading system, this would be actual computation time
fn simulate_work(micros: u64) {
    thread::sleep(Duration::from_micros(micros));
}
