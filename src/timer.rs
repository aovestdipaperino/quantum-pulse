//! # Profiling Timer
//!
//! Provides RAII-based timers for measuring operation durations.
//! Timers automatically record their duration when dropped.

use std::time::Instant;

use crate::category::{Category, DefaultCategory};
use crate::collector::ProfileCollector;

/// A timer that automatically records duration when dropped
///
/// This timer uses RAII (Resource Acquisition Is Initialization) to ensure
/// that timing measurements are always recorded, even if the code panics.
///
/// # Example
/// ```rust
/// use profile_timer::ProfileTimer;
///
/// {
///     let _timer = ProfileTimer::new("database_query");
///     // Your code here
///     perform_database_query();
///     // Timer automatically records when it goes out of scope
/// }
/// ```
pub struct ProfileTimer<C: Category = DefaultCategory> {
    operation: String,
    category: C,
    start_time: Instant,
    recorded: bool,
}

impl ProfileTimer<DefaultCategory> {
    /// Create a new timer for the given operation with default category
    ///
    /// The timer will start immediately and record its duration
    /// when it goes out of scope.
    pub fn new<T: std::fmt::Debug>(operation: T) -> Self {
        Self {
            operation: format!("{:?}", operation),
            category: DefaultCategory::Other,
            start_time: Instant::now(),
            recorded: false,
        }
    }
}

impl<C: Category> ProfileTimer<C> {
    /// Create a new timer with a specific category
    pub fn with_category<T: std::fmt::Debug>(operation: T, category: C) -> Self {
        Self {
            operation: format!("{:?}", operation),
            category,
            start_time: Instant::now(),
            recorded: false,
        }
    }

    /// Get the operation name
    pub fn operation(&self) -> &str {
        &self.operation
    }

    /// Get the category
    pub fn category(&self) -> &C {
        &self.category
    }

    /// Get the elapsed time since the timer was created
    pub fn elapsed(&self) -> std::time::Duration {
        self.start_time.elapsed()
    }

    /// Get the elapsed time in microseconds
    pub fn elapsed_micros(&self) -> u64 {
        self.elapsed().as_micros() as u64
    }

    /// Get the elapsed time in milliseconds
    pub fn elapsed_millis(&self) -> u64 {
        self.elapsed().as_millis() as u64
    }

    /// Manually record the timer (usually done automatically on drop)
    pub fn record(&mut self) {
        if !self.recorded {
            ProfileCollector::record_with_category(
                &self.operation,
                self.category.clone(),
                self.elapsed_micros(),
            );
            self.recorded = true;
        }
    }

    /// Stop the timer and return the elapsed duration without recording
    ///
    /// This consumes the timer and prevents automatic recording on drop.
    pub fn stop(mut self) -> std::time::Duration {
        let duration = self.elapsed();
        self.recorded = true; // Prevent recording on drop
        duration
    }

    /// Stop the timer, record it, and return the elapsed duration
    pub fn stop_and_record(mut self) -> std::time::Duration {
        let duration = self.elapsed();
        self.record();
        duration
    }
}

impl<C: Category> Drop for ProfileTimer<C> {
    fn drop(&mut self) {
        if !self.recorded {
            self.record();
        }
    }
}

/// A timer that can be paused and resumed
///
/// This timer allows for more complex timing scenarios where you need to
/// exclude certain periods from the measurement (e.g., waiting for I/O).
///
/// # Example
/// ```rust
/// use profile_timer::PausableTimer;
///
/// let mut timer = PausableTimer::new("complex_operation");
///
/// // Do some work
/// perform_step_1();
///
/// // Pause during I/O wait
/// timer.pause();
/// wait_for_io();
/// timer.resume();
///
/// // Continue work
/// perform_step_2();
///
/// // Timer records total time excluding the paused period
/// ```
pub struct PausableTimer<C: Category = DefaultCategory> {
    operation: String,
    category: C,
    total_duration: std::time::Duration,
    start_time: Option<Instant>,
    recorded: bool,
}

impl PausableTimer<DefaultCategory> {
    /// Create a new pausable timer with default category
    pub fn new<T: std::fmt::Debug>(operation: T) -> Self {
        Self {
            operation: format!("{:?}", operation),
            category: DefaultCategory::Other,
            total_duration: std::time::Duration::ZERO,
            start_time: Some(Instant::now()),
            recorded: false,
        }
    }

    /// Create a new pausable timer that starts paused
    pub fn new_paused<T: std::fmt::Debug>(operation: T) -> Self {
        Self {
            operation: format!("{:?}", operation),
            category: DefaultCategory::Other,
            total_duration: std::time::Duration::ZERO,
            start_time: None,
            recorded: false,
        }
    }
}

impl<C: Category> PausableTimer<C> {
    /// Create a new pausable timer with a specific category
    pub fn with_category<T: std::fmt::Debug>(operation: T, category: C) -> Self {
        Self {
            operation: format!("{:?}", operation),
            category,
            total_duration: std::time::Duration::ZERO,
            start_time: Some(Instant::now()),
            recorded: false,
        }
    }

    /// Create a new pausable timer with category that starts paused
    pub fn with_category_paused<T: std::fmt::Debug>(operation: T, category: C) -> Self {
        Self {
            operation: format!("{:?}", operation),
            category,
            total_duration: std::time::Duration::ZERO,
            start_time: None,
            recorded: false,
        }
    }

    /// Pause the timer
    ///
    /// If the timer is already paused, this has no effect.
    pub fn pause(&mut self) {
        if let Some(start) = self.start_time.take() {
            self.total_duration += start.elapsed();
        }
    }

    /// Resume the timer
    ///
    /// If the timer is already running, this has no effect.
    pub fn resume(&mut self) {
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }
    }

    /// Get the total elapsed time (excluding paused periods)
    pub fn total_elapsed(&self) -> std::time::Duration {
        let mut total = self.total_duration;
        if let Some(start) = self.start_time {
            total += start.elapsed();
        }
        total
    }

    /// Get the total elapsed time in microseconds
    pub fn total_elapsed_micros(&self) -> u64 {
        self.total_elapsed().as_micros() as u64
    }

    /// Get the total elapsed time in milliseconds
    pub fn total_elapsed_millis(&self) -> u64 {
        self.total_elapsed().as_millis() as u64
    }

    /// Check if the timer is currently running
    pub fn is_running(&self) -> bool {
        self.start_time.is_some()
    }

    /// Get the operation name
    pub fn operation(&self) -> &str {
        &self.operation
    }

    /// Get the category
    pub fn category(&self) -> &C {
        &self.category
    }

    /// Record the current total duration
    pub fn record(&mut self) {
        if !self.recorded {
            ProfileCollector::record_with_category(
                &self.operation,
                self.category.clone(),
                self.total_elapsed_micros(),
            );
            self.recorded = true;
        }
    }

    /// Stop the timer and return the total elapsed duration without recording
    pub fn stop(mut self) -> std::time::Duration {
        self.pause();
        let duration = self.total_duration;
        self.recorded = true; // Prevent recording on drop
        duration
    }

    /// Stop the timer, record it, and return the total elapsed duration
    pub fn stop_and_record(mut self) -> std::time::Duration {
        self.pause();
        let duration = self.total_duration;
        self.record();
        duration
    }

    /// Reset the timer to zero and start it
    pub fn reset(&mut self) {
        self.total_duration = std::time::Duration::ZERO;
        self.start_time = Some(Instant::now());
        self.recorded = false;
    }

    /// Reset the timer to zero and pause it
    pub fn reset_paused(&mut self) {
        self.total_duration = std::time::Duration::ZERO;
        self.start_time = None;
        self.recorded = false;
    }
}

impl<C: Category> Drop for PausableTimer<C> {
    fn drop(&mut self) {
        if !self.recorded {
            self.record();
        }
    }
}

/// A guard that measures the time spent in a scope
///
/// This is useful for measuring specific code blocks without
/// having to create a separate function.
///
/// # Example
/// ```rust
/// use profile_timer::ScopedTimer;
///
/// fn complex_function() {
///     let _guard = ScopedTimer::new("setup");
///     perform_setup();
///     // Guard automatically records when it goes out of scope
/// }
/// ```
pub type ScopedTimer<C = DefaultCategory> = ProfileTimer<C>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_profile_timer_basic() {
        ProfileCollector::clear_all();

        {
            let timer = ProfileTimer::new("test_operation");
            thread::sleep(Duration::from_millis(1));
            assert!(timer.elapsed_micros() > 0);
            assert!(timer.elapsed_millis() >= 1);
        }

        assert!(ProfileCollector::has_data());
        let stats = ProfileCollector::get_stats("test_operation");
        assert!(stats.is_some());
        assert_eq!(stats.unwrap().count, 1);
    }

    #[test]
    fn test_profile_timer_with_category() {
        ProfileCollector::clear_all();

        {
            let _timer = ProfileTimer::with_category("categorized_op", DefaultCategory::Compute);
            thread::sleep(Duration::from_millis(1));
        }

        let category = ProfileCollector::get_category::<DefaultCategory>("categorized_op");
        assert_eq!(category, Some(DefaultCategory::Compute));
    }

    #[test]
    fn test_profile_timer_stop() {
        ProfileCollector::clear_all();

        let timer = ProfileTimer::new("stopped_operation");
        thread::sleep(Duration::from_millis(1));
        let duration = timer.stop();

        assert!(duration.as_millis() >= 1);
        // Should not be recorded since we called stop()
        thread::sleep(Duration::from_millis(10));
        assert!(!ProfileCollector::has_data());
    }

    #[test]
    fn test_profile_timer_stop_and_record() {
        ProfileCollector::clear_all();

        let timer = ProfileTimer::new("stop_and_record_op");
        thread::sleep(Duration::from_millis(1));
        let duration = timer.stop_and_record();

        assert!(duration.as_millis() >= 1);
        assert!(ProfileCollector::has_data());
        let stats = ProfileCollector::get_stats("stop_and_record_op");
        assert!(stats.is_some());
        assert_eq!(stats.unwrap().count, 1);
    }

    #[test]
    fn test_pausable_timer() {
        ProfileCollector::clear_all();

        let mut timer = PausableTimer::new("test_pausable");
        assert!(timer.is_running());

        thread::sleep(Duration::from_millis(10));
        timer.pause();
        assert!(!timer.is_running());

        let paused_duration = timer.total_elapsed();
        thread::sleep(Duration::from_millis(10));
        // Duration shouldn't change while paused
        assert_eq!(timer.total_elapsed(), paused_duration);

        timer.resume();
        assert!(timer.is_running());
        thread::sleep(Duration::from_millis(10));
        assert!(timer.total_elapsed() > paused_duration);

        drop(timer);

        assert!(ProfileCollector::has_data());
        let stats = ProfileCollector::get_stats("test_pausable");
        assert!(stats.is_some());
    }

    #[test]
    fn test_pausable_timer_start_paused() {
        let timer = PausableTimer::new_paused("start_paused");
        assert!(!timer.is_running());
        assert_eq!(timer.total_elapsed_micros(), 0);
    }

    #[test]
    fn test_pausable_timer_reset() {
        let mut timer = PausableTimer::new("test_reset");
        thread::sleep(Duration::from_millis(10));
        assert!(timer.total_elapsed_millis() >= 10);

        timer.reset();
        assert!(timer.is_running());
        assert!(timer.total_elapsed_millis() < 10);
    }

    #[test]
    fn test_manual_record() {
        ProfileCollector::clear_all();

        let mut timer = ProfileTimer::new("manual_record");
        thread::sleep(Duration::from_millis(1));
        timer.record();

        // Recording again should not duplicate
        timer.record();

        drop(timer);

        let stats = ProfileCollector::get_stats("manual_record");
        assert!(stats.is_some());
        assert_eq!(stats.unwrap().count, 1);
    }

    #[test]
    fn test_custom_category() {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        enum TestCategory {
            Fast,
            Slow,
        }

        impl Category for TestCategory {}

        ProfileCollector::clear_all();

        {
            let _timer = ProfileTimer::with_category("fast_op", TestCategory::Fast);
            thread::sleep(Duration::from_millis(1));
        }

        let category = ProfileCollector::get_category::<TestCategory>("fast_op");
        assert_eq!(category, Some(TestCategory::Fast));
    }
}
