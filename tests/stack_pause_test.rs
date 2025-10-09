//! Tests for stack-based pause/unpause functionality
//!
//! These tests verify that pause_stack!() and unpause_stack!() only affect
//! timers currently on the call stack, not all profiling operations.
//!
//! Key behavior: pause_stack!() marks all currently active timers to NOT record
//! when they are dropped. unpause_stack!() removes the pause mark from currently
//! active timers, allowing them to record again.

use quantum_pulse::{pause_stack, profile, unpause_stack, ProfileCollector, ProfileOp};
use std::thread;
use std::time::Duration;

#[derive(Debug, ProfileOp)]
enum TestOp {
    #[category(name = "Outer", description = "Outer operations")]
    OuterWork,

    #[category(name = "Inner", description = "Inner operations")]
    InnerWork,

    #[category(name = "Sibling", description = "Sibling operations")]
    SiblingWork,
}

#[test]
fn test_stack_pause_prevents_recording() {
    ProfileCollector::clear_all();

    // Start outer timer
    profile!(TestOp::OuterWork, {
        thread::sleep(Duration::from_millis(10));

        // Pause only the timers currently on the stack (OuterWork)
        // This marks them to not record when dropped
        pause_stack!();

        thread::sleep(Duration::from_millis(10));
    });

    // But new timers created after pause should still work
    profile!(TestOp::InnerWork, {
        thread::sleep(Duration::from_millis(5));
    });

    // Check that InnerWork was recorded
    let inner_stats = ProfileCollector::get_stats("Inner::InnerWork");
    assert!(inner_stats.is_some(), "InnerWork should be recorded");
    assert_eq!(inner_stats.unwrap().count, 1);

    // Check that OuterWork was NOT recorded (it was marked as paused)
    let outer_stats = ProfileCollector::get_stats("Outer::OuterWork");
    assert!(
        outer_stats.is_none(),
        "OuterWork should not be recorded because pause_stack was called while it was active"
    );
}

#[test]
fn test_stack_pause_with_unpause() {
    ProfileCollector::clear_all();

    profile!(TestOp::OuterWork, {
        thread::sleep(Duration::from_millis(5));

        // Pause the timer
        pause_stack!();

        thread::sleep(Duration::from_millis(5));

        // Resume it before it drops
        unpause_stack!();

        thread::sleep(Duration::from_millis(5));
    });

    // Since we resumed before drop, it should record
    let outer_stats = ProfileCollector::get_stats("Outer::OuterWork");
    assert!(
        outer_stats.is_some(),
        "OuterWork should be recorded because we called unpause_stack before it dropped"
    );
    assert_eq!(outer_stats.unwrap().count, 1);
}

#[test]
fn test_stack_pause_nested_timers() {
    ProfileCollector::clear_all();

    profile!(TestOp::OuterWork, {
        thread::sleep(Duration::from_millis(5));

        profile!(TestOp::InnerWork, {
            thread::sleep(Duration::from_millis(5));

            // Pause both timers on the stack
            pause_stack!();

            thread::sleep(Duration::from_millis(10));
        });

        thread::sleep(Duration::from_millis(5));
    });

    // Inner was paused and never resumed, should not record
    let inner_stats = ProfileCollector::get_stats("Inner::InnerWork");
    assert!(
        inner_stats.is_none(),
        "InnerWork should not be recorded because it was paused"
    );

    // Outer was also on the stack when pause was called, should not record
    let outer_stats = ProfileCollector::get_stats("Outer::OuterWork");
    assert!(
        outer_stats.is_none(),
        "OuterWork should not be recorded because it was also on the stack when paused"
    );
}

#[test]
fn test_stack_pause_nested_with_unpause() {
    ProfileCollector::clear_all();

    profile!(TestOp::OuterWork, {
        thread::sleep(Duration::from_millis(5));

        profile!(TestOp::InnerWork, {
            thread::sleep(Duration::from_millis(5));

            // Pause both timers on the stack
            pause_stack!();

            thread::sleep(Duration::from_millis(5));

            // Resume both timers
            unpause_stack!();

            thread::sleep(Duration::from_millis(5));
        });

        thread::sleep(Duration::from_millis(5));
    });

    // Both should record since we resumed before they dropped
    let inner_stats = ProfileCollector::get_stats("Inner::InnerWork");
    assert!(inner_stats.is_some(), "InnerWork should be recorded");
    assert_eq!(inner_stats.unwrap().count, 1);

    let outer_stats = ProfileCollector::get_stats("Outer::OuterWork");
    assert!(outer_stats.is_some(), "OuterWork should be recorded");
    assert_eq!(outer_stats.unwrap().count, 1);
}

#[test]
fn test_stack_pause_does_not_affect_new_timers() {
    ProfileCollector::clear_all();

    // Start one timer and pause it
    profile!(TestOp::OuterWork, {
        thread::sleep(Duration::from_millis(5));
        pause_stack!();
        thread::sleep(Duration::from_millis(5));
    });

    // After the paused timer, start a fresh timer
    // This should NOT be affected by the previous pause_stack
    profile!(TestOp::SiblingWork, {
        thread::sleep(Duration::from_millis(5));
    });

    // New timer should be recorded
    let sibling_stats = ProfileCollector::get_stats("Sibling::SiblingWork");
    assert!(
        sibling_stats.is_some(),
        "SiblingWork should be recorded normally"
    );
    assert_eq!(sibling_stats.unwrap().count, 1);

    // Original timer should not be recorded
    let outer_stats = ProfileCollector::get_stats("Outer::OuterWork");
    assert!(outer_stats.is_none(), "OuterWork should not be recorded");
}

#[test]
fn test_multiple_pause_unpause_cycles() {
    ProfileCollector::clear_all();

    profile!(TestOp::OuterWork, {
        thread::sleep(Duration::from_millis(5));

        // First pause
        pause_stack!();
        thread::sleep(Duration::from_millis(5));
        // Resume
        unpause_stack!();

        thread::sleep(Duration::from_millis(5));

        // Second pause
        pause_stack!();
        thread::sleep(Duration::from_millis(5));
        // Resume again
        unpause_stack!();

        thread::sleep(Duration::from_millis(5));
    });

    // Should be recorded since we resumed at the end
    let stats = ProfileCollector::get_stats("Outer::OuterWork");
    assert!(stats.is_some(), "OuterWork should be recorded");
    assert_eq!(stats.unwrap().count, 1);
}
