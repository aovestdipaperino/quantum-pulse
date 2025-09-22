//! Example demonstrating pause/unpause functionality
//!
//! This example shows how to use the `pause!()` and `unpause!()` macros
//! to selectively control which operations are profiled.

use quantum_pulse::{pause, profile, unpause, ProfileCollector, ProfileOp};
use std::thread;
use std::time::Duration;

#[derive(Debug, ProfileOp)]
enum AppOperation {
    #[category(name = "Core", description = "Core business operations")]
    CriticalWork,

    #[category(name = "Maintenance", description = "Background maintenance tasks")]
    BackgroundTask,

    #[category(name = "Debug", description = "Debug and diagnostic operations")]
    DiagnosticWork,
}

fn perform_critical_work() {
    // Simulate some critical work
    thread::sleep(Duration::from_millis(50));
    println!("  ✓ Critical work completed");
}

fn perform_background_task() {
    // Simulate background maintenance
    thread::sleep(Duration::from_millis(30));
    println!("  ✓ Background task completed");
}

fn perform_diagnostic_work() {
    // Simulate diagnostic work
    thread::sleep(Duration::from_millis(20));
    println!("  ✓ Diagnostic work completed");
}

fn main() {
    println!("🔍 Demonstrating Pause/Unpause Profiling");
    println!("==========================================\n");

    // Clear any existing data
    ProfileCollector::clear_all();

    println!("📊 Phase 1: Normal profiling (all operations recorded)");

    profile!(AppOperation::CriticalWork, {
        perform_critical_work();
    });

    profile!(AppOperation::BackgroundTask, {
        perform_background_task();
    });

    profile!(AppOperation::DiagnosticWork, {
        perform_diagnostic_work();
    });

    // Check stats after phase 1
    let stats = ProfileCollector::get_all_stats();
    println!("📈 Operations recorded: {}", stats.len());
    for (name, stat) in &stats {
        println!(
            "  - {}: {} calls, avg {}μs",
            name,
            stat.count,
            stat.mean().as_micros()
        );
    }
    println!();

    println!("⏸️  Phase 2: Paused profiling (no operations recorded)");

    // Pause all profiling
    pause!();
    println!("  Profiling paused: {}", ProfileCollector::is_paused());

    profile!(AppOperation::CriticalWork, {
        perform_critical_work();
    });

    profile!(AppOperation::BackgroundTask, {
        perform_background_task();
    });

    profile!(AppOperation::DiagnosticWork, {
        perform_diagnostic_work();
    });

    // Check stats after phase 2 - should be the same
    let stats_paused = ProfileCollector::get_all_stats();
    println!("📈 Operations still recorded: {}", stats_paused.len());
    for (name, stat) in &stats_paused {
        println!(
            "  - {}: {} calls, avg {}μs",
            name,
            stat.count,
            stat.mean().as_micros()
        );
    }
    println!("  (Notice: call counts didn't increase during pause)\n");

    println!("▶️  Phase 3: Resumed profiling (operations recorded again)");

    // Resume profiling
    unpause!();
    println!("  Profiling paused: {}", ProfileCollector::is_paused());

    profile!(AppOperation::CriticalWork, {
        perform_critical_work();
    });

    profile!(AppOperation::BackgroundTask, {
        perform_background_task();
    });

    // Check final stats
    let final_stats = ProfileCollector::get_all_stats();
    println!("📈 Final operations recorded: {}", final_stats.len());
    for (name, stat) in &final_stats {
        println!(
            "  - {}: {} calls, avg {}μs",
            name,
            stat.count,
            stat.mean().as_micros()
        );
    }
    println!();

    println!("💡 Use Cases for Pause/Unpause:");
    println!("  - Exclude initialization/cleanup from measurements");
    println!("  - Focus profiling on specific code sections");
    println!("  - Reduce overhead during non-critical operations");
    println!("  - Debug performance issues in targeted areas");

    println!("\n🎯 Example: Selective profiling in a loop");
    ProfileCollector::clear_all();

    for i in 0..5 {
        if i == 2 {
            println!("  Pausing profiling for iteration {}", i);
            pause!();
        } else if i == 4 {
            println!("  Resuming profiling for iteration {}", i);
            unpause!();
        }

        profile!(AppOperation::CriticalWork, {
            thread::sleep(Duration::from_millis(10));
            println!("    Iteration {} completed", i);
        });
    }

    let loop_stats = ProfileCollector::get_stats("Core::CriticalWork");
    if let Some(stats) = loop_stats {
        println!(
            "📊 Loop profiling results: {} calls recorded (should be 3, not 5)",
            stats.count
        );
        println!("   (Iterations 2 and 3 were excluded due to pause)");
    }
}
