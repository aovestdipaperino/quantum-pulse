//! Integration tests for pause/unpause functionality
//!
//! These tests verify that the pause!() and unpause!() macros work correctly
//! in various scenarios and don't interfere with normal profiling operations.

use quantum_pulse::{pause, profile, unpause, ProfileCollector, ProfileOp};
use std::thread;
use std::time::Duration;

#[derive(Debug, ProfileOp)]
enum TestOperation1 {
    #[category(name = "Core1", description = "Core operations")]
    CriticalWork,
}

#[derive(Debug, ProfileOp)]
enum TestOperation2 {
    #[category(name = "Core2", description = "Core operations")]
    CriticalWork,

    #[category(name = "Background2", description = "Background tasks")]
    MaintenanceWork,

    #[category(name = "Debug2", description = "Debug operations")]
    DiagnosticWork,
}

#[derive(Debug, ProfileOp)]
enum TestOperation3 {
    #[category(name = "Core3", description = "Core operations")]
    CriticalWork,
}

#[derive(Debug, ProfileOp)]
enum TestOperation4 {
    #[category(name = "Core4", description = "Core operations")]
    CriticalWork,
}

#[derive(Debug, ProfileOp)]
enum TestOperation5 {
    #[category(name = "Core5", description = "Core operations")]
    CriticalWork,
}

#[derive(Debug, ProfileOp)]
enum TestOperation6 {
    #[category(name = "Core6", description = "Core operations")]
    CriticalWork,

    #[category(name = "Background6", description = "Background tasks")]
    MaintenanceWork,
}

#[test]
fn test_pause_unpause_basic() {
    ProfileCollector::clear_all();
    ProfileCollector::reset_pause_state();

    // Ensure we start unpaused
    unpause!();
    assert!(!ProfileCollector::is_paused());

    // Record something normally
    profile!(TestOperation1::CriticalWork, {
        thread::sleep(Duration::from_millis(1));
    });

    let stats_before = ProfileCollector::get_stats("Core1::CriticalWork");
    assert!(stats_before.is_some());
    assert_eq!(stats_before.unwrap().count, 1);

    // Pause profiling
    pause!();
    assert!(ProfileCollector::is_paused());

    // This should not be recorded
    profile!(TestOperation1::CriticalWork, {
        thread::sleep(Duration::from_millis(1));
    });

    let stats_during_pause = ProfileCollector::get_stats("Core1::CriticalWork");
    assert!(stats_during_pause.is_some());
    assert_eq!(stats_during_pause.unwrap().count, 1); // No change

    // Resume profiling
    unpause!();
    assert!(!ProfileCollector::is_paused());

    // This should be recorded
    profile!(TestOperation1::CriticalWork, {
        thread::sleep(Duration::from_millis(1));
    });

    let stats_after_unpause = ProfileCollector::get_stats("Core1::CriticalWork");
    assert!(stats_after_unpause.is_some());
    assert_eq!(stats_after_unpause.unwrap().count, 2);
}

#[test]
fn test_pause_unpause_multiple_operations() {
    ProfileCollector::clear_all();
    ProfileCollector::reset_pause_state();
    unpause!();

    // Record multiple operations normally
    profile!(TestOperation2::CriticalWork, {
        thread::sleep(Duration::from_millis(1));
    });

    profile!(TestOperation2::MaintenanceWork, {
        thread::sleep(Duration::from_millis(1));
    });

    profile!(TestOperation2::DiagnosticWork, {
        thread::sleep(Duration::from_millis(1));
    });

    // Check initial state
    assert_eq!(
        ProfileCollector::get_stats("Core2::CriticalWork")
            .unwrap()
            .count,
        1
    );
    assert_eq!(
        ProfileCollector::get_stats("Background2::MaintenanceWork")
            .unwrap()
            .count,
        1
    );
    assert_eq!(
        ProfileCollector::get_stats("Debug2::DiagnosticWork")
            .unwrap()
            .count,
        1
    );

    // Pause and record again - nothing should change
    pause!();

    profile!(TestOperation2::CriticalWork, {
        thread::sleep(Duration::from_millis(1));
    });

    profile!(TestOperation2::MaintenanceWork, {
        thread::sleep(Duration::from_millis(1));
    });

    profile!(TestOperation2::DiagnosticWork, {
        thread::sleep(Duration::from_millis(1));
    });

    // Counts should remain the same
    assert_eq!(
        ProfileCollector::get_stats("Core2::CriticalWork")
            .unwrap()
            .count,
        1
    );
    assert_eq!(
        ProfileCollector::get_stats("Background2::MaintenanceWork")
            .unwrap()
            .count,
        1
    );
    assert_eq!(
        ProfileCollector::get_stats("Debug2::DiagnosticWork")
            .unwrap()
            .count,
        1
    );

    // Resume and record again
    unpause!();

    profile!(TestOperation2::CriticalWork, {
        thread::sleep(Duration::from_millis(1));
    });

    profile!(TestOperation2::MaintenanceWork, {
        thread::sleep(Duration::from_millis(1));
    });

    // Counts should increase
    assert_eq!(
        ProfileCollector::get_stats("Core2::CriticalWork")
            .unwrap()
            .count,
        2
    );
    assert_eq!(
        ProfileCollector::get_stats("Background2::MaintenanceWork")
            .unwrap()
            .count,
        2
    );
    assert_eq!(
        ProfileCollector::get_stats("Debug2::DiagnosticWork")
            .unwrap()
            .count,
        1
    ); // Didn't record this one
}

#[test]
fn test_pause_unpause_in_loop() {
    ProfileCollector::clear_all();
    ProfileCollector::reset_pause_state();
    unpause!();

    let mut recorded_iterations = 0;

    for i in 0..10 {
        // Pause on even iterations
        if i % 2 == 0 {
            pause!();
        } else {
            unpause!();
            recorded_iterations += 1;
        }

        profile!(TestOperation3::CriticalWork, {
            thread::sleep(Duration::from_millis(1));
        });
    }

    // Should have recorded only odd iterations (1, 3, 5, 7, 9) = 5 times
    let stats = ProfileCollector::get_stats("Core3::CriticalWork");
    assert!(stats.is_some());
    assert_eq!(stats.unwrap().count, recorded_iterations);
    assert_eq!(recorded_iterations, 5);
}

#[test]
fn test_nested_pause_unpause() {
    ProfileCollector::clear_all();
    ProfileCollector::reset_pause_state();
    unpause!();

    // Record something
    profile!(TestOperation4::CriticalWork, {
        thread::sleep(Duration::from_millis(1));
    });

    assert_eq!(
        ProfileCollector::get_stats("Core4::CriticalWork")
            .unwrap()
            .count,
        1
    );

    // Pause multiple times
    pause!();
    pause!(); // Should be idempotent
    pause!();

    assert!(ProfileCollector::is_paused());

    // Nothing should be recorded
    profile!(TestOperation4::CriticalWork, {
        thread::sleep(Duration::from_millis(1));
    });

    assert_eq!(
        ProfileCollector::get_stats("Core4::CriticalWork")
            .unwrap()
            .count,
        1
    );

    // Unpause multiple times
    unpause!();
    unpause!(); // Should be idempotent
    unpause!();

    assert!(!ProfileCollector::is_paused());

    // Should record again
    profile!(TestOperation4::CriticalWork, {
        thread::sleep(Duration::from_millis(1));
    });

    assert_eq!(
        ProfileCollector::get_stats("Core4::CriticalWork")
            .unwrap()
            .count,
        2
    );
}

#[test]
fn test_clear_all_resets_pause_state() {
    ProfileCollector::clear_all();
    ProfileCollector::reset_pause_state();

    // Pause profiling
    pause!();
    assert!(ProfileCollector::is_paused());

    // Clear all data (should also reset pause state)
    ProfileCollector::clear_all();
    assert!(!ProfileCollector::is_paused());

    // Should be able to record normally
    profile!(TestOperation5::CriticalWork, {
        thread::sleep(Duration::from_millis(1));
    });

    let stats = ProfileCollector::get_stats("Core5::CriticalWork");
    assert!(stats.is_some());
    assert_eq!(stats.unwrap().count, 1);
}

#[test]
fn test_pause_state_isolation() {
    // This test ensures that pause state doesn't leak between operations
    ProfileCollector::clear_all();
    ProfileCollector::reset_pause_state();

    // Function that pauses internally
    fn paused_work() {
        pause!();
        profile!(TestOperation6::MaintenanceWork, {
            thread::sleep(Duration::from_millis(1));
        });
        // Intentionally don't unpause
    }

    // Function that should work normally
    fn normal_work() {
        profile!(TestOperation6::CriticalWork, {
            thread::sleep(Duration::from_millis(1));
        });
    }

    // Start unpaused
    unpause!();

    // Do normal work - should be recorded
    normal_work();
    assert_eq!(
        ProfileCollector::get_stats("Core6::CriticalWork")
            .unwrap()
            .count,
        1
    );

    // Do paused work - should not be recorded
    paused_work();
    assert!(ProfileCollector::get_stats("Background6::MaintenanceWork").is_none());

    // Pause state should still be active
    assert!(ProfileCollector::is_paused());

    // Normal work should not be recorded while paused
    normal_work();
    assert_eq!(
        ProfileCollector::get_stats("Core6::CriticalWork")
            .unwrap()
            .count,
        1
    ); // No change

    // Resume and try again
    unpause!();
    normal_work();
    assert_eq!(
        ProfileCollector::get_stats("Core6::CriticalWork")
            .unwrap()
            .count,
        2
    );
}
