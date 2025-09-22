//! Example showing how a high-frequency trading system integrates with Quantum Pulse
//! This demonstrates microsecond-precision profiling for latency-sensitive applications using Operation trait

use quantum_pulse::{profile, profile_async, Category, Operation, ProfileCollector};
use std::thread;
use std::time::Duration;
use tokio::time::sleep;

/// Trading system operations that implement Operation trait
#[derive(Debug)]
enum TradingOperation {
    // Order Processing
    OrderValidation,
    OrderSubmission,
    OrderExecution,
    OrderConfirmation,
    OrderCancellation,

    // Market Data Processing
    PriceFeedParsing,
    MarketDataUpdate,
    VolatilityCalculation,
    TechnicalAnalysis,

    // Risk Management
    RiskAssessment,
    PositionUpdate,
    ExposureCalculation,
    LimitCheck,

    // Portfolio Management
    PnlCalculation,
    RebalanceOperation,
    PortfolioOptimization,

    // External Communications
    ExchangeConnection,
    MarketDataFeed,
    RegulatoryReporting,

    // High-Frequency Operations
    ArbitrageDetection,
    LatencyMeasurement,
    OrderBookUpdate,
}

/// Order processing category for trade lifecycle operations
#[derive(Debug)]
struct OrderProcessingCategory;

impl Category for OrderProcessingCategory {
    fn get_name(&self) -> &str {
        "OrderProcessing"
    }

    fn get_description(&self) -> &str {
        "Order lifecycle operations including validation, submission, and execution"
    }

    fn color_hint(&self) -> Option<&str> {
        Some("#FF6B6B")
    }

    fn priority(&self) -> i32 {
        1 // Highest priority - critical path
    }
}

/// Market data category for price feed and market information
#[derive(Debug)]
struct MarketDataCategory;

impl Category for MarketDataCategory {
    fn get_name(&self) -> &str {
        "MarketData"
    }

    fn get_description(&self) -> &str {
        "Market data processing, price feeds, and market analysis"
    }

    fn color_hint(&self) -> Option<&str> {
        Some("#4ECDC4")
    }

    fn priority(&self) -> i32 {
        2
    }
}

/// Risk management category for risk assessment and control
#[derive(Debug)]
struct RiskManagementCategory;

impl Category for RiskManagementCategory {
    fn get_name(&self) -> &str {
        "RiskManagement"
    }

    fn get_description(&self) -> &str {
        "Risk assessment, position monitoring, and exposure calculations"
    }

    fn color_hint(&self) -> Option<&str> {
        Some("#E74C3C")
    }

    fn priority(&self) -> i32 {
        1 // Also highest priority - risk critical
    }
}

/// Portfolio management category for portfolio operations
#[derive(Debug)]
struct PortfolioCategory;

impl Category for PortfolioCategory {
    fn get_name(&self) -> &str {
        "Portfolio"
    }

    fn get_description(&self) -> &str {
        "Portfolio management, PnL calculations, and optimization"
    }

    fn color_hint(&self) -> Option<&str> {
        Some("#96CEB4")
    }

    fn priority(&self) -> i32 {
        3
    }
}

/// External communication category for exchange and data connections
#[derive(Debug)]
struct ExternalCommCategory;

impl Category for ExternalCommCategory {
    fn get_name(&self) -> &str {
        "ExternalComm"
    }

    fn get_description(&self) -> &str {
        "External communications with exchanges, data providers, and regulators"
    }

    fn color_hint(&self) -> Option<&str> {
        Some("#F39C12")
    }

    fn priority(&self) -> i32 {
        4
    }
}

/// High-frequency operations category for latency-critical operations
#[derive(Debug)]
struct HighFrequencyCategory;

impl Category for HighFrequencyCategory {
    fn get_name(&self) -> &str {
        "HighFrequency"
    }

    fn get_description(&self) -> &str {
        "Ultra-low latency operations for high-frequency trading"
    }

    fn color_hint(&self) -> Option<&str> {
        Some("#9B59B6")
    }

    fn priority(&self) -> i32 {
        1 // Highest priority - latency critical
    }
}

impl Operation for TradingOperation {
    fn get_category(&self) -> &dyn Category {
        match self {
            TradingOperation::OrderValidation
            | TradingOperation::OrderSubmission
            | TradingOperation::OrderExecution
            | TradingOperation::OrderConfirmation
            | TradingOperation::OrderCancellation => &OrderProcessingCategory,

            TradingOperation::PriceFeedParsing
            | TradingOperation::MarketDataUpdate
            | TradingOperation::VolatilityCalculation
            | TradingOperation::TechnicalAnalysis => &MarketDataCategory,

            TradingOperation::RiskAssessment
            | TradingOperation::PositionUpdate
            | TradingOperation::ExposureCalculation
            | TradingOperation::LimitCheck => &RiskManagementCategory,

            TradingOperation::PnlCalculation
            | TradingOperation::RebalanceOperation
            | TradingOperation::PortfolioOptimization => &PortfolioCategory,

            TradingOperation::ExchangeConnection
            | TradingOperation::MarketDataFeed
            | TradingOperation::RegulatoryReporting => &ExternalCommCategory,

            TradingOperation::ArbitrageDetection
            | TradingOperation::LatencyMeasurement
            | TradingOperation::OrderBookUpdate => &HighFrequencyCategory,
        }
    }

    fn to_str(&self) -> String {
        match self {
            TradingOperation::OrderValidation => "order_validation".to_string(),
            TradingOperation::OrderSubmission => "order_submission".to_string(),
            TradingOperation::OrderExecution => "order_execution".to_string(),
            TradingOperation::OrderConfirmation => "order_confirmation".to_string(),
            TradingOperation::OrderCancellation => "order_cancellation".to_string(),
            TradingOperation::PriceFeedParsing => "price_feed_parsing".to_string(),
            TradingOperation::MarketDataUpdate => "market_data_update".to_string(),
            TradingOperation::VolatilityCalculation => "volatility_calculation".to_string(),
            TradingOperation::TechnicalAnalysis => "technical_analysis".to_string(),
            TradingOperation::RiskAssessment => "risk_assessment".to_string(),
            TradingOperation::PositionUpdate => "position_update".to_string(),
            TradingOperation::ExposureCalculation => "exposure_calculation".to_string(),
            TradingOperation::LimitCheck => "limit_check".to_string(),
            TradingOperation::PnlCalculation => "pnl_calculation".to_string(),
            TradingOperation::RebalanceOperation => "rebalance_operation".to_string(),
            TradingOperation::PortfolioOptimization => "portfolio_optimization".to_string(),
            TradingOperation::ExchangeConnection => "exchange_connection".to_string(),
            TradingOperation::MarketDataFeed => "market_data_feed".to_string(),
            TradingOperation::RegulatoryReporting => "regulatory_reporting".to_string(),
            TradingOperation::ArbitrageDetection => "arbitrage_detection".to_string(),
            TradingOperation::LatencyMeasurement => "latency_measurement".to_string(),
            TradingOperation::OrderBookUpdate => "order_book_update".to_string(),
        }
    }
}

/// Simulated order structure
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Order {
    id: u64,
    symbol: String,
    quantity: i64,
    price: f64,
    order_type: OrderType,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum OrderType {
    Market,
    Limit,
    Stop,
}

/// Simulated market data
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct MarketData {
    symbol: String,
    bid: f64,
    ask: f64,
    volume: u64,
    timestamp: std::time::SystemTime,
}

#[tokio::main]
async fn main() {
    println!("=== High-Frequency Trading System - Quantum Pulse Integration ===\n");

    // Clear any existing profiling data
    ProfileCollector::clear_all();

    // Simulate various trading system operations
    println!("🚀 Starting trading system simulation...\n");

    // Example 1: Order processing workflow
    println!("1. Order Processing Workflow:");
    process_trading_orders().await;

    // Example 2: Market data processing
    println!("\n2. Market Data Processing:");
    process_market_data().await;

    // Example 3: Risk management operations
    println!("\n3. Risk Management Operations:");
    perform_risk_checks().await;

    // Example 4: High-frequency operations (microsecond precision)
    println!("\n4. High-Frequency Operations:");
    execute_hft_operations();

    // Example 5: Portfolio management
    println!("\n5. Portfolio Management:");
    manage_portfolio().await;

    // Example 6: Concurrent trading operations
    println!("\n6. Concurrent Trading Operations:");
    run_concurrent_trading_ops().await;

    // Example 7: Latency-critical path simulation
    println!("\n7. Critical Path Latency Test:");
    test_critical_path_latency();

    // Generate comprehensive trading system report
    generate_trading_system_report();
}

async fn process_trading_orders() {
    let orders = vec![
        Order {
            id: 1001,
            symbol: "AAPL".to_string(),
            quantity: 100,
            price: 150.25,
            order_type: OrderType::Limit,
        },
        Order {
            id: 1002,
            symbol: "MSFT".to_string(),
            quantity: -200,
            price: 0.0, // Market order
            order_type: OrderType::Market,
        },
        Order {
            id: 1003,
            symbol: "GOOGL".to_string(),
            quantity: 50,
            price: 2800.0,
            order_type: OrderType::Stop,
        },
    ];

    for order in orders {
        println!("   Processing order {} for {}...", order.id, order.symbol);

        // Step 1: Validate order
        let validation_op = TradingOperation::OrderValidation;
        let is_valid = profile!(validation_op, {
            // Simulate validation logic
            thread::sleep(Duration::from_micros(50));
            validate_order(&order)
        });

        if !is_valid {
            println!("   ❌ Order {} failed validation", order.id);
            continue;
        }

        // Step 2: Risk assessment
        let risk_op = TradingOperation::RiskAssessment;
        let risk_approved = profile!(risk_op, {
            thread::sleep(Duration::from_micros(30));
            assess_order_risk(&order)
        });

        if !risk_approved {
            println!("   ⚠️  Order {} rejected by risk management", order.id);
            continue;
        }

        // Step 3: Submit order
        let submission_op = TradingOperation::OrderSubmission;
        profile_async!(submission_op, async {
            sleep(Duration::from_micros(100)).await;
            println!("   📤 Order {} submitted to exchange", order.id);
        })
        .await;

        // Step 4: Simulate execution
        let execution_op = TradingOperation::OrderExecution;
        profile_async!(execution_op, async {
            sleep(Duration::from_micros(200)).await;
            println!("   ✅ Order {} executed", order.id);
        })
        .await;

        // Step 5: Confirm execution
        let confirmation_op = TradingOperation::OrderConfirmation;
        profile!(confirmation_op, {
            thread::sleep(Duration::from_micros(25));
            println!("   📋 Order {} confirmed", order.id);
        });

        // Step 6: Update position
        let position_op = TradingOperation::PositionUpdate;
        profile!(position_op, {
            thread::sleep(Duration::from_micros(40));
            println!("   📊 Position updated for {}", order.symbol);
        });
    }
}

async fn process_market_data() {
    let market_data = vec![
        MarketData {
            symbol: "AAPL".to_string(),
            bid: 150.20,
            ask: 150.25,
            volume: 1000000,
            timestamp: std::time::SystemTime::now(),
        },
        MarketData {
            symbol: "MSFT".to_string(),
            bid: 310.15,
            ask: 310.20,
            volume: 800000,
            timestamp: std::time::SystemTime::now(),
        },
        MarketData {
            symbol: "GOOGL".to_string(),
            bid: 2799.50,
            ask: 2800.00,
            volume: 500000,
            timestamp: std::time::SystemTime::now(),
        },
    ];

    for data in market_data {
        println!("   Processing market data for {}...", data.symbol);

        // Parse price feed
        let parsing_op = TradingOperation::PriceFeedParsing;
        profile!(parsing_op, {
            // Ultra-fast parsing - microsecond precision
            thread::sleep(Duration::from_micros(5));
        });

        // Update market data
        let update_op = TradingOperation::MarketDataUpdate;
        profile!(update_op, {
            thread::sleep(Duration::from_micros(10));
        });

        // Calculate volatility
        let volatility_op = TradingOperation::VolatilityCalculation;
        profile_async!(volatility_op, async {
            sleep(Duration::from_micros(80)).await;
            let volatility = calculate_volatility(&data);
            println!("   📈 Volatility for {}: {:.2}%", data.symbol, volatility);
        })
        .await;

        // Technical analysis
        let analysis_op = TradingOperation::TechnicalAnalysis;
        profile!(analysis_op, {
            thread::sleep(Duration::from_micros(120));
            println!("   🔍 Technical analysis completed for {}", data.symbol);
        });
    }
}

async fn perform_risk_checks() {
    println!("   Performing comprehensive risk assessment...");

    // Risk assessment
    let risk_op = TradingOperation::RiskAssessment;
    profile!(risk_op, {
        thread::sleep(Duration::from_micros(150));
        println!("   🛡️  Risk assessment completed");
    });

    // Calculate exposure
    let exposure_op = TradingOperation::ExposureCalculation;
    profile_async!(exposure_op, async {
        sleep(Duration::from_micros(200)).await;
        println!("   📊 Market exposure calculated");
    })
    .await;

    // Perform limit checks
    for limit_type in &["position_limit", "var_limit", "concentration_limit"] {
        let limit_op = TradingOperation::LimitCheck;
        profile!(limit_op, {
            thread::sleep(Duration::from_micros(30));
            println!("   ✅ {} check passed", limit_type);
        });
    }
}

fn execute_hft_operations() {
    println!("   Executing high-frequency trading operations...");

    // Ultra-low latency operations (sub-microsecond targeting)
    for i in 0..10 {
        // Arbitrage detection
        let arb_op = TradingOperation::ArbitrageDetection;
        profile!(arb_op, {
            // Simulate ultra-fast arbitrage detection
            thread::sleep(Duration::from_nanos(500)); // Sub-microsecond
        });

        // Order book update
        let book_op = TradingOperation::OrderBookUpdate;
        profile!(book_op, {
            thread::sleep(Duration::from_nanos(300)); // Sub-microsecond
        });

        // Latency measurement
        let latency_op = TradingOperation::LatencyMeasurement;
        profile!(latency_op, {
            thread::sleep(Duration::from_nanos(100)); // Sub-microsecond
        });

        if i % 3 == 0 {
            println!("   ⚡ HFT batch {} completed", i / 3 + 1);
        }
    }
}

async fn manage_portfolio() {
    println!("   Managing portfolio...");

    // PnL calculation
    let pnl_op = TradingOperation::PnlCalculation;
    profile_async!(pnl_op, async {
        sleep(Duration::from_micros(300)).await;
        println!("   💰 PnL calculation completed: $125,430.50");
    })
    .await;

    // Portfolio rebalancing
    let rebalance_op = TradingOperation::RebalanceOperation;
    profile_async!(rebalance_op, async {
        sleep(Duration::from_millis(2)).await;
        println!("   ⚖️  Portfolio rebalanced");
    })
    .await;

    // Portfolio optimization
    let optimization_op = TradingOperation::PortfolioOptimization;
    profile_async!(optimization_op, async {
        sleep(Duration::from_millis(5)).await;
        println!("   🎯 Portfolio optimization completed");
    })
    .await;
}

async fn run_concurrent_trading_ops() {
    println!("   Running concurrent trading operations...");

    // Using the OrderCancellation variant to avoid unused warning
    if false {
        profile!(TradingOperation::OrderCancellation, {
            println!("Cancelling order");
        });
    }

    let feed_op = TradingOperation::MarketDataFeed;
    let exchange_op = TradingOperation::ExchangeConnection;
    let reporting_op = TradingOperation::RegulatoryReporting;

    let (feed_result, exchange_result, reporting_result) = tokio::join!(
        profile_async!(feed_op, async {
            sleep(Duration::from_micros(500)).await;
            "Market data feed active"
        }),
        profile_async!(exchange_op, async {
            sleep(Duration::from_micros(800)).await;
            "Exchange connection established"
        }),
        profile_async!(reporting_op, async {
            sleep(Duration::from_millis(10)).await;
            "Regulatory reports submitted"
        })
    );

    println!("   📡 {}", feed_result);
    println!("   🔗 {}", exchange_result);
    println!("   📋 {}", reporting_result);
}

fn test_critical_path_latency() {
    println!("   Testing critical path latency (Order -> Execution)...");

    let iterations = 1000;

    for i in 0..iterations {
        // Simulate the critical path: Validation -> Risk -> Submission -> Execution
        let validation_op = TradingOperation::OrderValidation;
        profile!(validation_op, {
            // Target: < 10 microseconds
            thread::sleep(Duration::from_nanos(200));
        });

        let risk_op = TradingOperation::RiskAssessment;
        profile!(risk_op, {
            // Target: < 5 microseconds
            thread::sleep(Duration::from_nanos(100));
        });

        let submission_op = TradingOperation::OrderSubmission;
        profile!(submission_op, {
            // Target: < 50 microseconds
            thread::sleep(Duration::from_nanos(800));
        });

        let execution_op = TradingOperation::OrderExecution;
        profile!(execution_op, {
            // Target: < 100 microseconds
            thread::sleep(Duration::from_micros(2));
        });

        if i % 100 == 0 && i > 0 {
            println!("   ⚡ Completed {} critical path iterations", i);
        }
    }

    println!(
        "   🎯 Critical path latency test completed ({} iterations)",
        iterations
    );
}

fn generate_trading_system_report() {
    println!("\n{}", "=".repeat(80));
    println!("TRADING SYSTEM PERFORMANCE REPORT");
    println!("{}", "=".repeat(80));

    ProfileCollector::report_stats();

    let summary = ProfileCollector::get_summary();
    println!("\nSYSTEM SUMMARY:");
    println!("- Total trading operations: {}", summary.total_operations);
    println!("- Unique operation types: {}", summary.unique_operations);
    println!("- Total processing time: {}μs", summary.total_time_micros);
    println!(
        "- Average operation time: {:.2}μs",
        summary.total_time_micros as f64 / summary.total_operations as f64
    );

    println!("\n{}", "=".repeat(80));
    println!("PERFORMANCE ANALYSIS BY TRADING CATEGORY");
    println!("{}", "=".repeat(80));

    let all_stats = ProfileCollector::get_all_stats();

    // Group by category
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

    // Display by category with trading-specific insights
    let category_order = vec![
        "HighFrequency",
        "OrderProcessing",
        "RiskManagement",
        "MarketData",
        "Portfolio",
        "ExternalComm",
    ];

    for category_name in category_order {
        if let Some(ops) = categories.get(category_name) {
            println!("\n📊 {} Operations:", category_name);

            let mut total_calls = 0;
            let mut total_time = Duration::ZERO;
            let mut min_time = Duration::MAX;
            let mut max_time = Duration::ZERO;

            for (op_name, stats) in ops {
                let avg_time = stats.mean();
                println!(
                    "   {} - {} calls, avg: {:?}",
                    op_name, stats.count, avg_time
                );

                total_calls += stats.count;
                total_time += stats.total;
                min_time = min_time.min(avg_time);
                max_time = max_time.max(avg_time);
            }

            let avg_category_time = if total_calls > 0 {
                total_time / total_calls as u32
            } else {
                Duration::ZERO
            };

            println!(
                "   📈 Category Summary: {} calls, {:?} total, {:?} avg",
                total_calls, total_time, avg_category_time
            );

            // Category-specific performance insights
            match category_name {
                "HighFrequency" => {
                    if avg_category_time > Duration::from_micros(1) {
                        println!("   ⚠️  WARNING: HFT operations averaging > 1μs");
                    } else {
                        println!("   ✅ HFT latency within target");
                    }
                }
                "OrderProcessing" => {
                    if avg_category_time > Duration::from_micros(100) {
                        println!("   ⚠️  WARNING: Order processing > 100μs average");
                    } else {
                        println!("   ✅ Order processing within SLA");
                    }
                }
                "RiskManagement" => {
                    if avg_category_time > Duration::from_micros(50) {
                        println!("   ⚠️  WARNING: Risk checks > 50μs average");
                    } else {
                        println!("   ✅ Risk management performance good");
                    }
                }
                _ => {}
            }
        }
    }

    println!("\n{}", "=".repeat(80));
    println!("LATENCY ANALYSIS - TOP PERFORMERS");
    println!("{}", "=".repeat(80));

    let all_stats = ProfileCollector::get_all_stats();
    let mut sorted_by_avg: Vec<_> = all_stats.iter().collect();
    sorted_by_avg.sort_by(|a, b| a.1.mean().cmp(&b.1.mean()));

    println!("\n🏆 FASTEST Operations (Top 10):");
    for (i, (name, stats)) in sorted_by_avg.iter().take(10).enumerate() {
        println!(
            "  {}. {} - avg: {:?} ({} calls)",
            i + 1,
            name,
            stats.mean(),
            stats.count
        );
    }

    println!("\n🐌 SLOWEST Operations (Top 5):");
    for (i, (name, stats)) in sorted_by_avg.iter().rev().take(5).enumerate() {
        println!(
            "  {}. {} - avg: {:?} ({} calls)",
            i + 1,
            name,
            stats.mean(),
            stats.count
        );
    }

    // Trading-specific performance recommendations
    println!("\n{}", "=".repeat(80));
    println!("PERFORMANCE RECOMMENDATIONS");
    println!("{}", "=".repeat(80));

    let high_freq_ops: Vec<_> = all_stats
        .iter()
        .filter(|(key, _)| key.starts_with("HighFrequency::"))
        .collect();

    if !high_freq_ops.is_empty() {
        let avg_hf_time: Duration = high_freq_ops
            .iter()
            .map(|(_, stats)| stats.mean())
            .sum::<Duration>()
            / high_freq_ops.len() as u32;

        println!("\n🚀 High-Frequency Trading Performance:");
        println!("   Average HFT operation time: {:?}", avg_hf_time);

        if avg_hf_time > Duration::from_micros(1) {
            println!("   📋 RECOMMENDATION: Optimize HFT operations to < 1μs");
            println!("      - Consider CPU affinity optimization");
            println!("      - Review memory allocation patterns");
            println!("      - Implement lock-free data structures");
        } else {
            println!("   ✅ HFT performance excellent!");
        }
    }

    println!("\n💡 General Recommendations:");
    println!("   - Monitor order processing latency closely");
    println!("   - Consider async processing for non-critical path operations");
    println!("   - Implement circuit breakers for risk management");
    println!("   - Use dedicated hardware for HFT operations");
}

// Utility functions
fn validate_order(order: &Order) -> bool {
    // Simulate order validation logic
    !order.symbol.is_empty() && order.quantity != 0
}

fn assess_order_risk(order: &Order) -> bool {
    // Simulate risk assessment
    order.quantity.abs() <= 10000 // Simple position size limit
}

fn calculate_volatility(market_data: &MarketData) -> f64 {
    // Simulate volatility calculation
    let spread = market_data.ask - market_data.bid;
    spread / market_data.bid * 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trading_operation_categories() {
        let order_op = TradingOperation::OrderValidation;
        assert_eq!(order_op.get_category().get_name(), "OrderProcessing");
        assert_eq!(order_op.to_str(), "order_validation");

        let risk_op = TradingOperation::RiskAssessment;
        assert_eq!(risk_op.get_category().get_name(), "RiskManagement");
        assert_eq!(risk_op.to_str(), "risk_assessment");

        let hft_op = TradingOperation::ArbitrageDetection;
        assert_eq!(hft_op.get_category().get_name(), "HighFrequency");
        assert_eq!(hft_op.to_str(), "arbitrage_detection");
    }

    #[test]
    fn test_category_priorities() {
        let order_cat = OrderProcessingCategory;
        assert_eq!(order_cat.priority(), 1);

        let risk_cat = RiskManagementCategory;
        assert_eq!(risk_cat.priority(), 1);

        let hft_cat = HighFrequencyCategory;
        assert_eq!(hft_cat.priority(), 1);

        let portfolio_cat = PortfolioCategory;
        assert_eq!(portfolio_cat.priority(), 3);
    }

    #[test]
    fn test_order_validation() {
        let valid_order = Order {
            id: 1,
            symbol: "AAPL".to_string(),
            quantity: 100,
            price: 150.0,
            order_type: OrderType::Limit,
        };

        let invalid_order = Order {
            id: 2,
            symbol: "".to_string(),
            quantity: 0,
            price: 0.0,
            order_type: OrderType::Market,
        };

        assert!(validate_order(&valid_order));
        assert!(!validate_order(&invalid_order));
    }

    #[test]
    fn test_risk_assessment() {
        let safe_order = Order {
            id: 1,
            symbol: "AAPL".to_string(),
            quantity: 1000,
            price: 150.0,
            order_type: OrderType::Limit,
        };

        let risky_order = Order {
            id: 2,
            symbol: "AAPL".to_string(),
            quantity: 50000, // Exceeds limit
            price: 150.0,
            order_type: OrderType::Limit,
        };

        assert!(assess_order_risk(&safe_order));
        assert!(!assess_order_risk(&risky_order));
    }

    #[test]
    fn test_volatility_calculation() {
        let market_data = MarketData {
            symbol: "AAPL".to_string(),
            bid: 150.0,
            ask: 150.5,
            volume: 1000,
            timestamp: std::time::SystemTime::now(),
        };

        let volatility = calculate_volatility(&market_data);
        assert!((volatility - 0.3333).abs() < 0.01); // Should be ~0.33%
    }

    #[tokio::test]
    async fn test_trading_operation_profiling() {
        ProfileCollector::clear_all();

        let op = TradingOperation::OrderValidation;
        profile!(op, {
            thread::sleep(Duration::from_micros(1));
        });

        assert!(ProfileCollector::has_data());
        let stats = ProfileCollector::get_stats("OrderProcessing::order_validation");
        assert!(stats.is_some());
        assert_eq!(stats.unwrap().count, 1);
    }
}
