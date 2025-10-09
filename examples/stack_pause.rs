//! Example demonstrating stack-based pause/unpause functionality
//!
//! This example shows how to use pause_stack!() and unpause_stack!() to
//! selectively exclude portions of time from specific timers on the call stack.

use quantum_pulse::{pause_stack, profile, unpause_stack, ProfileCollector, ProfileOp};
use std::thread;
use std::time::Duration;

#[derive(Debug, ProfileOp)]
enum AppOperation {
    #[category(name = "Processing", description = "Data processing operations")]
    ProcessData,

    #[category(name = "IO", description = "Input/output operations")]
    DatabaseQuery,
    FileRead,

    #[category(name = "Network", description = "Network operations")]
    ApiCall,
}

fn simulate_work(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

fn main() {
    println!("🔍 Stack-Based Pause/Unpause Example");
    println!("=====================================\n");

    ProfileCollector::clear_all();

    // Example 1: Exclude I/O wait time from processing timer
    println!("📊 Example 1: Excluding I/O wait from processing timer");
    profile!(AppOperation::ProcessData, {
        println!("  - Processing data (10ms)");
        simulate_work(10);

        // We're about to do I/O - pause the ProcessData timer
        pause_stack!();

        println!("  - Waiting for database (paused, 50ms)");
        profile!(AppOperation::DatabaseQuery, {
            simulate_work(50);
        });

        // Resume the ProcessData timer
        unpause_stack!();

        println!("  - More processing (10ms)");
        simulate_work(10);
    });

    let stats = ProfileCollector::get_stats("Processing::ProcessData");
    if let Some(s) = stats {
        println!(
            "  ✓ ProcessData recorded: ~{}ms (excludes DB wait)",
            s.mean().as_millis()
        );
    }

    let db_stats = ProfileCollector::get_stats("IO::DatabaseQuery");
    if let Some(s) = db_stats {
        println!("  ✓ DatabaseQuery recorded: ~{}ms\n", s.mean().as_millis());
    }

    // Example 2: Stack pause affects all timers on the stack
    println!("📊 Example 2: Stack pause affects nested timers");
    ProfileCollector::clear_all();

    profile!(AppOperation::ProcessData, {
        println!("  - Outer processing started");
        simulate_work(10);

        profile!(AppOperation::FileRead, {
            println!("  - Inner file read started");
            simulate_work(10);

            // Pause ALL timers currently on the stack
            pause_stack!();

            println!("  - Network delay (paused for both timers)");
            profile!(AppOperation::ApiCall, {
                simulate_work(50);
            });

            unpause_stack!();

            println!("  - Inner file read continued");
            simulate_work(10);
        });

        println!("  - Outer processing continued");
        simulate_work(10);
    });

    let process_stats = ProfileCollector::get_stats("Processing::ProcessData");
    let file_stats = ProfileCollector::get_stats("IO::FileRead");
    let api_stats = ProfileCollector::get_stats("Network::ApiCall");

    if let Some(s) = process_stats {
        println!(
            "  ✓ ProcessData: ~{}ms (excluded network delay)",
            s.mean().as_millis()
        );
    } else {
        println!("  ✗ ProcessData: NOT recorded");
    }

    if let Some(s) = file_stats {
        println!(
            "  ✓ FileRead: ~{}ms (excluded network delay)",
            s.mean().as_millis()
        );
    } else {
        println!("  ✗ FileRead: NOT recorded");
    }

    if let Some(s) = api_stats {
        println!("  ✓ ApiCall: ~{}ms (not affected)\n", s.mean().as_millis());
    } else {
        println!("  ✗ ApiCall: NOT recorded\n");
    }

    // Example 3: Pause without unpause = don't record
    println!("📊 Example 3: Paused timer without resume doesn't record");
    ProfileCollector::clear_all();

    profile!(AppOperation::ProcessData, {
        println!("  - Processing started");
        simulate_work(10);

        // Pause and never resume - timer won't be recorded
        pause_stack!();

        println!("  - Timer is now paused");
        simulate_work(10);
    });

    let paused_stats = ProfileCollector::get_stats("Processing::ProcessData");
    if paused_stats.is_none() {
        println!("  ✓ ProcessData was NOT recorded (never resumed)\n");
    }

    // Example 4: Multiple pause/unpause cycles
    println!("📊 Example 4: Multiple pause/unpause cycles");
    ProfileCollector::clear_all();

    profile!(AppOperation::ProcessData, {
        println!("  - Work phase 1");
        simulate_work(10);

        pause_stack!();
        println!("  - I/O wait 1 (paused)");
        simulate_work(20);
        unpause_stack!();

        println!("  - Work phase 2");
        simulate_work(10);

        pause_stack!();
        println!("  - I/O wait 2 (paused)");
        simulate_work(20);
        unpause_stack!();

        println!("  - Work phase 3");
        simulate_work(10);
    });

    let cycle_stats = ProfileCollector::get_stats("Processing::ProcessData");
    if let Some(s) = cycle_stats {
        println!(
            "  ✓ ProcessData: ~{}ms (excluded both I/O waits)\n",
            s.mean().as_millis()
        );
    }

    println!("💡 Use Cases:");
    println!("  - Exclude I/O wait time from algorithm profiling");
    println!("  - Measure only CPU-bound work in mixed operations");
    println!("  - Exclude network latency from processing metrics");
    println!("  - Conditional profiling based on runtime conditions");
}
