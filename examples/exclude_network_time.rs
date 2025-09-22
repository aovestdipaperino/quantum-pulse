//! Example demonstrating how to exclude network time from profiling
//!
//! This example shows various techniques for measuring only business logic
//! while excluding network I/O time from performance measurements.

use quantum_pulse::{pause, profile, unpause, PausableTimer, ProfileCollector, ProfileOp};
use std::thread;
use std::time::Duration;

#[derive(Debug, ProfileOp)]
enum PaymentOperation {
    #[category(name = "Business", description = "Core business logic")]
    PaymentProcessing,
    OrderValidation,
    PaymentFinalization,

    #[category(name = "Network", description = "External API calls")]
    BankApiCall,
    FraudCheckApi,
    PaymentGateway,

    #[category(name = "Data", description = "Data processing")]
    ResponseProcessing,
}

// Simulate network calls with delays
async fn call_bank_api(account: &str) -> Result<String, String> {
    println!("    🌐 Calling bank API for account: {}", account);
    thread::sleep(Duration::from_millis(150)); // Simulate network latency
    Ok(format!("Bank response for {}", account))
}

async fn call_fraud_detection(amount: f64) -> Result<bool, String> {
    println!("    🌐 Checking fraud score for amount: ${:.2}", amount);
    thread::sleep(Duration::from_millis(200)); // Simulate network latency
    Ok(amount < 10000.0) // Simple fraud check
}

async fn call_payment_gateway(data: &str) -> Result<String, String> {
    println!("    🌐 Processing payment via gateway: {}", data);
    thread::sleep(Duration::from_millis(100)); // Simulate network latency
    Ok("Payment successful".to_string())
}

// Business logic functions (fast)
fn validate_payment_request(account: &str, amount: f64) -> Result<(), String> {
    println!("    ✓ Validating payment request");
    thread::sleep(Duration::from_millis(5)); // Simulate validation work
    if account.is_empty() {
        return Err("Invalid account".to_string());
    }
    if amount <= 0.0 {
        return Err("Invalid amount".to_string());
    }
    Ok(())
}

fn prepare_payment_data(account: &str, amount: f64, bank_response: &str) -> String {
    println!("    ✓ Preparing payment data");
    thread::sleep(Duration::from_millis(10)); // Simulate data preparation
    format!("Payment: {} -> ${:.2} ({})", account, amount, bank_response)
}

fn finalize_payment(gateway_response: &str) -> String {
    println!("    ✓ Finalizing payment");
    thread::sleep(Duration::from_millis(8)); // Simulate finalization
    format!("Final result: {}", gateway_response)
}

/// Method 1: Using pause!() and unpause!() macros
async fn process_payment_with_pause_macros(account: &str, amount: f64) -> Result<String, String> {
    println!("\n🔧 Method 1: Using pause!()/unpause!() macros");

    let result = profile!(PaymentOperation::PaymentProcessing, {
        // Business logic - measured
        validate_payment_request(account, amount)?;

        // Pause before network call
        pause!();
        let bank_response = call_bank_api(account).await?;
        let fraud_ok = call_fraud_detection(amount).await?;
        unpause!();

        if !fraud_ok {
            return Err("Fraud detected".to_string());
        }

        // Business logic continues - measured
        let payment_data = prepare_payment_data(account, amount, &bank_response);

        // Pause for another network call
        pause!();
        let gateway_response = call_payment_gateway(&payment_data).await?;
        unpause!();

        // Final business logic - measured
        finalize_payment(&gateway_response)
    });

    Ok(result)
}

/// Method 2: Using PausableTimer for fine-grained control
async fn process_payment_with_pausable_timer(account: &str, amount: f64) -> Result<String, String> {
    println!("\n🔧 Method 2: Using PausableTimer");

    let mut timer = PausableTimer::new(&PaymentOperation::PaymentProcessing);

    // Business logic - timed
    validate_payment_request(account, amount)?;

    // Pause for network calls
    timer.pause();
    let bank_response = call_bank_api(account).await?;
    let fraud_ok = call_fraud_detection(amount).await?;
    timer.resume();

    if !fraud_ok {
        timer.stop(); // Stop without recording
        return Err("Fraud detected".to_string());
    }

    // Business logic continues - timed
    let payment_data = prepare_payment_data(account, amount, &bank_response);

    // Pause again
    timer.pause();
    let gateway_response = call_payment_gateway(&payment_data).await?;
    timer.resume();

    // Final business logic - timed
    let result = finalize_payment(&gateway_response);

    // Timer records automatically on drop
    Ok(result)
}

/// Method 3: Separate profiling for business logic and network (recommended)
async fn process_payment_with_separate_profiling(
    account: &str,
    amount: f64,
) -> Result<String, String> {
    println!("\n🔧 Method 3: Separate profiling (Recommended)");

    // Profile business validation separately
    profile!(PaymentOperation::OrderValidation, {
        validate_payment_request(account, amount)?;
    });

    // Profile network calls separately (optional - you might exclude these)
    let bank_response = profile!(PaymentOperation::BankApiCall, {
        call_bank_api(account).await?
    });

    let fraud_ok = profile!(PaymentOperation::FraudCheckApi, {
        call_fraud_detection(amount).await?
    });

    if !fraud_ok {
        return Err("Fraud detected".to_string());
    }

    // Profile data processing
    let payment_data = profile!(PaymentOperation::ResponseProcessing, {
        prepare_payment_data(account, amount, &bank_response)
    });

    // Profile gateway call separately
    let gateway_response = profile!(PaymentOperation::PaymentGateway, {
        call_payment_gateway(&payment_data).await?
    });

    // Profile finalization
    let result = profile!(PaymentOperation::PaymentFinalization, {
        finalize_payment(&gateway_response)
    });

    Ok(result)
}

/// Method 4: Nested profiling with comprehensive categorization
async fn process_payment_with_nested_profiling(
    account: &str,
    amount: f64,
) -> Result<String, String> {
    println!("\n🔧 Method 4: Nested profiling with categorization");

    let result = profile!(PaymentOperation::PaymentProcessing, {
        // Business validation
        profile!(PaymentOperation::OrderValidation, {
            validate_payment_request(account, amount)?;
        });

        // External API calls - separate category for filtering
        let bank_response = profile!(PaymentOperation::BankApiCall, {
            call_bank_api(account).await?
        });

        let fraud_ok = profile!(PaymentOperation::FraudCheckApi, {
            call_fraud_detection(amount).await?
        });

        if !fraud_ok {
            return Err("Fraud detected".to_string());
        }

        // Data processing - separate category
        let payment_data = profile!(PaymentOperation::ResponseProcessing, {
            prepare_payment_data(account, amount, &bank_response)
        });

        // Payment gateway - network category
        let gateway_response = profile!(PaymentOperation::PaymentGateway, {
            call_payment_gateway(&payment_data).await?
        });

        // Business finalization
        profile!(PaymentOperation::PaymentFinalization, {
            finalize_payment(&gateway_response)
        })
    });

    Ok(result)
}

fn print_profiling_stats(method_name: &str) {
    println!("\n📊 {} Results:", method_name);
    let all_stats = ProfileCollector::get_all_stats();

    // Separate business logic from network calls
    let mut business_stats = Vec::new();
    let mut network_stats = Vec::new();
    let mut data_stats = Vec::new();

    for (name, stats) in all_stats {
        if name.starts_with("Business::") {
            business_stats.push((name, stats));
        } else if name.starts_with("Network::") {
            network_stats.push((name, stats));
        } else if name.starts_with("Data::") {
            data_stats.push((name, stats));
        }
    }

    if !business_stats.is_empty() {
        println!("  🏢 Business Logic (what we care about):");
        for (name, stats) in business_stats {
            println!(
                "    - {}: {} calls, avg {}μs",
                name,
                stats.count,
                stats.mean().as_micros()
            );
        }
    }

    if !data_stats.is_empty() {
        println!("  💾 Data Processing:");
        for (name, stats) in data_stats {
            println!(
                "    - {}: {} calls, avg {}μs",
                name,
                stats.count,
                stats.mean().as_micros()
            );
        }
    }

    if !network_stats.is_empty() {
        println!("  🌐 Network Calls (excluded from business metrics):");
        for (name, stats) in network_stats {
            println!(
                "    - {}: {} calls, avg {}μs",
                name,
                stats.count,
                stats.mean().as_micros()
            );
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Excluding Network Time from Profiling");
    println!("========================================\n");

    println!("This example shows different ways to measure only business logic");
    println!("performance while excluding network I/O time.\n");

    let account = "user123";
    let amount = 500.0;

    // Method 1: pause!/unpause! macros
    ProfileCollector::clear_all();
    let _result1 = process_payment_with_pause_macros(account, amount).await?;
    print_profiling_stats("Method 1 (Pause/Unpause Macros)");

    // Method 2: PausableTimer
    ProfileCollector::clear_all();
    let _result2 = process_payment_with_pausable_timer(account, amount).await?;
    print_profiling_stats("Method 2 (PausableTimer)");

    // Method 3: Separate profiling (recommended)
    ProfileCollector::clear_all();
    let _result3 = process_payment_with_separate_profiling(account, amount).await?;
    print_profiling_stats("Method 3 (Separate Profiling)");

    // Method 4: Nested profiling
    ProfileCollector::clear_all();
    let _result4 = process_payment_with_nested_profiling(account, amount).await?;
    print_profiling_stats("Method 4 (Nested Profiling)");

    println!("\n💡 Recommendations:");
    println!("  1. Use Method 3 (Separate Profiling) for most cases");
    println!("  2. Use Method 1 (pause!/unpause!) for simple exclusions");
    println!("  3. Use Method 2 (PausableTimer) for complex timing control");
    println!("  4. Use Method 4 (Nested) for comprehensive analysis");

    println!("\n🎯 Benefits of excluding network time:");
    println!("  - Focus on code you can actually optimize");
    println!("  - Separate concerns (business logic vs. infrastructure)");
    println!("  - More stable performance baselines");
    println!("  - Better identification of actual bottlenecks");

    Ok(())
}
