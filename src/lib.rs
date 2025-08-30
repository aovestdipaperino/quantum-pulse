//! # Quantum Pulse - Zero-Cost Profiling Library
//!
//! A profiling library that provides true zero-cost abstractions through compile-time
//! feature selection. When disabled, all profiling code compiles to nothing.
//!
//! ## Architecture
//!
//! This library provides two implementations:
//! - **Stub (default)**: Empty implementations that compile away completely
//! - **Full (opt-in)**: Complete profiling with HDR histograms and reporting
//!
//! Both implementations expose the exact same API, allowing you to write clean,
//! unconditional code that works in both development and production.
//!
//! ## Features
//!
//! - **True Zero-Cost**: Stub implementations are inlined and eliminated by the optimizer
//! - **Type-Safe Categories**: Define custom categories with compile-time guarantees
//! - **Percentile Statistics**: Accurate p50, p95, p99, p99.9 using HDR histograms (full mode)
//! - **Clean API**: No conditional compilation needed in your code
//! - **Async Support**: Full support for async/await patterns
//! - **Thread-Safe**: Safe to use from multiple threads
//!
//! ## Quick Start
//!
//! ```rust
//! use quantum_pulse::{Profiler, Category, profile};
//!
//! // Your code always looks the same, regardless of features
//! let result = profile!("database_query" => {
//!     expensive_database_query()
//! });
//!
//! // With default features (stub): compiles to just `expensive_database_query()`
//! // With "full" feature: includes timing and statistics
//!
//! // Generate a report (empty in stub mode, full in full mode)
//! let report = Profiler::report();
//! println!("{}", report);
//! ```
//!
//! ## Zero-Cost Guarantee
//!
//! When the "full" feature is not enabled, all methods have empty bodies that
//! are marked for inlining. The compiler's optimizer completely removes these
//! calls, resulting in zero runtime overhead and minimal binary size impact.

// Implementation selection based on features
// When "full" feature is enabled, use the complete implementation
// Otherwise, use stub implementations that compile to nothing

// Full implementation modules - complete profiling functionality
#[cfg(feature = "full")]
pub mod category;
#[cfg(feature = "full")]
pub mod collector;
#[cfg(feature = "full")]
pub mod metrics;
#[cfg(feature = "full")]
pub mod reporter;
#[cfg(feature = "full")]
pub mod timer;

// Stub implementation modules - zero-cost abstractions
// These modules provide the same API but with empty implementations
// that are completely optimized away by the compiler
#[cfg(not(feature = "full"))]
pub mod category {
    pub trait Category: Clone + Eq + std::hash::Hash + Send + Sync + 'static {
        fn description(&self) -> Option<&str> {
            None
        }
        fn color_hint(&self) -> Option<&str> {
            None
        }
        fn priority(&self) -> i32 {
            0
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum DefaultCategory {
        IO,
        Compute,
        Network,
        Database,
        Other,
    }

    impl Category for DefaultCategory {}
}

#[cfg(not(feature = "full"))]
pub mod collector {
    use super::category::Category;
    use std::collections::HashMap;

    #[derive(Debug, Clone, Default)]
    pub struct OperationStats {
        pub count: u64,
        pub total_micros: u64,
        pub min_micros: u64,
        pub max_micros: u64,
        pub mean_micros: u64,
    }

    pub struct ProfileCollector;

    impl ProfileCollector {
        pub fn record(_operation: &str, _duration_micros: u64) {}
        pub fn record_with_category<C: Category>(
            _operation: &str,
            _category: C,
            _duration_micros: u64,
        ) {
        }
        pub fn get_stats(_operation: &str) -> Option<OperationStats> {
            None
        }
        pub fn get_all_stats() -> HashMap<String, OperationStats> {
            HashMap::new()
        }
        pub fn get_category<C: Category>(_operation: &str) -> Option<C> {
            None
        }
        pub fn clear_all() {}
        pub fn reset_all() {}
        pub fn reset_operation(_operation: &str) {}
        pub fn has_data() -> bool {
            false
        }
        pub fn total_operations() -> u64 {
            0
        }
        pub fn get_summary() -> SummaryStats {
            SummaryStats::default()
        }
        pub fn get_stats_by_category<C: Category>() -> HashMap<C, Vec<(String, OperationStats)>> {
            HashMap::new()
        }
    }

    #[derive(Debug, Default)]
    pub struct SummaryStats {
        pub total_operations: u64,
        pub unique_operations: usize,
        pub total_time_micros: u64,
    }
}

#[cfg(not(feature = "full"))]
pub mod reporter {
    use super::category::Category;
    use super::collector::OperationStats;
    use std::collections::HashMap;

    #[derive(Debug, Clone, Copy)]
    pub enum TimeFormat {
        Microseconds,
        Milliseconds,
        Seconds,
        Auto,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum SortMetric {
        TotalTime,
        MeanTime,
        MaxTime,
        CallCount,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Percentile {
        pub p50: u64,
        pub p95: u64,
        pub p99: u64,
        pub p999: u64,
    }

    #[derive(Debug)]
    pub struct ReportConfig {
        pub include_percentiles: bool,
        pub group_by_category: bool,
        pub time_format: TimeFormat,
        pub sort_by: SortMetric,
        pub sort_by_time: bool,
        pub min_samples: u64,
    }

    impl Default for ReportConfig {
        fn default() -> Self {
            Self {
                include_percentiles: false,
                group_by_category: false,
                time_format: TimeFormat::Auto,
                sort_by: SortMetric::TotalTime,
                sort_by_time: false,
                min_samples: 0,
            }
        }
    }

    pub struct ProfileReport<C: Category> {
        pub stats: HashMap<String, OperationStats>,
        pub categories: HashMap<String, C>,
        pub config: ReportConfig,
    }

    impl<C: Category> ProfileReport<C> {
        pub fn generate() -> Self {
            Self {
                stats: HashMap::new(),
                categories: HashMap::new(),
                config: ReportConfig::default(),
            }
        }

        pub fn generate_with_config(_config: ReportConfig) -> Self {
            Self {
                stats: HashMap::new(),
                categories: HashMap::new(),
                config: ReportConfig::default(),
            }
        }

        pub fn quick_summary(&self) -> String {
            String::new()
        }

        pub fn summary_stats(&self) -> super::collector::SummaryStats {
            super::collector::SummaryStats::default()
        }

        pub fn to_string(&self) -> String {
            String::new()
        }

        pub fn top_operations_by(
            &self,
            _metric: SortMetric,
            _limit: usize,
        ) -> Vec<(String, OperationStats)> {
            Vec::new()
        }
    }

    impl<C: Category> std::fmt::Debug for ProfileReport<C> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "")
        }
    }

    pub struct ReportBuilder<C: Category> {
        _phantom: std::marker::PhantomData<C>,
    }

    impl<C: Category> Default for ReportBuilder<C> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<C: Category> ReportBuilder<C> {
        pub fn new() -> Self {
            Self {
                _phantom: std::marker::PhantomData,
            }
        }

        pub fn group_by_category(self, _enabled: bool) -> Self {
            self
        }
        pub fn include_percentiles(self, _enabled: bool) -> Self {
            self
        }
        pub fn time_format(self, _format: TimeFormat) -> Self {
            self
        }
        pub fn sort_by_time(self, _enabled: bool) -> Self {
            self
        }
        pub fn build(self) -> ProfileReport<C> {
            ProfileReport::generate()
        }
    }
}

#[cfg(not(feature = "full"))]
pub mod timer {
    use super::category::Category;

    pub struct ProfileTimer;

    impl ProfileTimer {
        pub fn new(_operation: &str) -> Self {
            ProfileTimer
        }

        pub fn with_category<C: Category>(_operation: &str, _category: C) -> Self {
            ProfileTimer
        }
    }

    impl Drop for ProfileTimer {
        fn drop(&mut self) {
            // No-op in stub mode
        }
    }

    pub struct PausableTimer;
}

// Re-export the appropriate implementations based on feature flags
pub use category::{Category, DefaultCategory};
pub use collector::{OperationStats, ProfileCollector};
pub use reporter::{
    Percentile, ProfileReport, ReportBuilder, ReportConfig, SortMetric, TimeFormat,
};
pub use timer::{PausableTimer, ProfileTimer};

#[cfg(not(feature = "full"))]
pub use collector::SummaryStats;
#[cfg(feature = "full")]
pub use collector::SummaryStats;

use std::marker::PhantomData;

/// Main profiling interface with customizable categories
///
/// This struct provides a unified API for profiling that works in both
/// stub and full modes. In stub mode, all methods are no-ops that compile
/// away. In full mode, complete profiling functionality is provided.
///
/// # Examples
///
/// ```rust
/// use quantum_pulse::Profiler;
///
/// // This code works identically in both modes
/// let result = Profiler::time("operation", || {
///     perform_operation()
/// });
/// ```
pub struct Profiler<C: Category = DefaultCategory> {
    _phantom: PhantomData<C>,
}

impl<C: Category> Default for Profiler<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: Category> Profiler<C> {
    /// Create a new profiler with custom categories
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }

    /// Profile a code block and automatically record timing
    ///
    /// # Example
    /// ```rust
    /// use profile_timer::Profiler;
    ///
    /// let result = Profiler::time("my_operation", || {
    ///     // Your code here
    ///     expensive_operation()
    /// });
    /// ```
    #[cfg(feature = "full")]
    pub fn time<T, F>(operation: &str, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        let _timer = ProfileTimer::new(operation);
        f()
    }

    #[cfg(not(feature = "full"))]
    pub fn time<T, F>(_operation: &str, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        f()
    }

    /// Profile a code block with a specific category
    #[cfg(feature = "full")]
    pub fn time_with_category<T, F>(operation: &str, category: C, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        let _timer = ProfileTimer::with_category(operation, category);
        f()
    }

    #[cfg(not(feature = "full"))]
    pub fn time_with_category<T, F>(_operation: &str, _category: C, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        f()
    }

    /// Profile an async code block
    #[cfg(feature = "full")]
    pub async fn time_async<T, F, Fut>(operation: &str, f: F) -> T
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = T>,
    {
        let _timer = ProfileTimer::new(operation);
        f().await
    }

    #[cfg(not(feature = "full"))]
    pub async fn time_async<T, F, Fut>(_operation: &str, f: F) -> T
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = T>,
    {
        f().await
    }

    /// Profile an async code block with a specific category
    #[cfg(feature = "full")]
    pub async fn time_async_with_category<T, F, Fut>(operation: &str, category: C, f: F) -> T
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = T>,
    {
        let _timer = ProfileTimer::with_category(operation, category);
        f().await
    }

    #[cfg(not(feature = "full"))]
    pub async fn time_async_with_category<T, F, Fut>(_operation: &str, _category: C, f: F) -> T
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = T>,
    {
        f().await
    }

    /// Get a comprehensive profiling report
    pub fn report() -> ProfileReport<C> {
        ProfileReport::generate()
    }

    /// Get a report with custom configuration
    pub fn report_with_config(config: ReportConfig) -> ProfileReport<C> {
        ProfileReport::generate_with_config(config)
    }

    /// Reset all profiling metrics
    pub fn reset() {
        ProfileCollector::reset_all()
    }

    /// Reset metrics for a specific operation
    pub fn reset_operation(operation: &str) {
        ProfileCollector::reset_operation(operation)
    }

    /// Record a raw metric value (in microseconds)
    pub fn record(operation: &str, value_micros: u64) {
        ProfileCollector::record(operation, value_micros);
    }

    /// Record a metric with a specific category
    pub fn record_with_category(operation: &str, category: C, value_micros: u64) {
        ProfileCollector::record_with_category(operation, category, value_micros);
    }

    /// Check if any profiling data has been collected
    pub fn has_data() -> bool {
        ProfileCollector::has_data()
    }

    /// Get total number of operations recorded
    pub fn total_operations() -> u64 {
        ProfileCollector::total_operations()
    }

    /// Get statistics for a specific operation
    pub fn get_stats(operation: &str) -> Option<OperationStats> {
        ProfileCollector::get_stats(operation)
    }

    /// Get all statistics
    pub fn get_all_stats() -> std::collections::HashMap<String, OperationStats> {
        ProfileCollector::get_all_stats()
    }
}

/// Builder for creating configured profiler instances
pub struct ProfilerBuilder<C: Category = DefaultCategory> {
    config: ReportConfig,
    _phantom: PhantomData<C>,
}

impl<C: Category> ProfilerBuilder<C> {
    /// Create a new profiler builder
    pub fn new() -> Self {
        Self {
            config: ReportConfig::default(),
            _phantom: PhantomData,
        }
    }

    /// Set whether to include percentiles in reports
    pub fn with_percentiles(mut self, include: bool) -> Self {
        self.config.include_percentiles = include;
        self
    }

    /// Set whether to sort operations by time in reports
    pub fn sort_by_time(mut self, sort: bool) -> Self {
        self.config.sort_by_time = sort;
        self
    }

    /// Set minimum number of samples required for an operation to appear in reports
    pub fn min_samples(mut self, min: u64) -> Self {
        self.config.min_samples = min;
        self
    }

    /// Set whether to group operations by category in reports
    pub fn group_by_category(mut self, group: bool) -> Self {
        self.config.group_by_category = group;
        self
    }

    /// Set the time format for reports
    pub fn time_format(mut self, format: TimeFormat) -> Self {
        self.config.time_format = format;
        self
    }

    /// Build the profiler
    pub fn build(self) -> Profiler<C> {
        Profiler::new()
    }
}

impl<C: Category> Default for ProfilerBuilder<C> {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience macro for profiling code blocks
///
/// # Example
/// ```rust
/// use profile_timer::profile;
///
/// let result = profile!("database_query" => {
///     database.query("SELECT * FROM users")
/// });
/// ```
#[macro_export]
macro_rules! profile {
    // Enum-based profiling with automatic Debug formatting and Category trait
    ($operation:expr => $block:block) => {{
        let op_name = format!("{:?}", $operation);
        $crate::Profiler::time_with_category(&op_name, $operation, || $block)
    }};
    ($operation:expr => async $block:block) => {{
        let op_name = format!("{:?}", $operation);
        $crate::Profiler::time_async_with_category(&op_name, $operation, || async move $block).await
    }};
    // Legacy string-based profiling with explicit category (deprecated - use enums instead)
    ($name:expr, $category:expr => $block:block) => {
        $crate::Profiler::time_with_category($name, $category, || $block)
    };
    ($name:expr, $category:expr => async $block:block) => {
        $crate::Profiler::time_async_with_category($name, $category, || async move $block).await
    };
}

/// Conditional profiling that only activates when a condition is met
#[macro_export]
macro_rules! profile_if {
    // Conditional enum-based profiling
    ($condition:expr, $operation:expr => $block:block) => {
        if $condition {
            $crate::profile!($operation => $block)
        } else {
            $block
        }
    };
    // Legacy conditional string-based profiling (deprecated)
    ($condition:expr, $name:expr, $category:expr => $block:block) => {
        if $condition {
            $crate::profile!($name, $category => $block)
        } else {
            $block
        }
    };
}

/// Create a scoped timer that records on drop
#[macro_export]
macro_rules! scoped_timer {
    ($name:expr) => {
        let _timer = $crate::ProfileTimer::<$crate::DefaultCategory>::new($name);
    };
    ($name:expr, $category:expr) => {
        let _timer = $crate::ProfileTimer::with_category($name, $category);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_profiling() {
        Profiler::<DefaultCategory>::reset();

        let result = Profiler::<DefaultCategory>::time("test_operation", || {
            std::thread::sleep(std::time::Duration::from_millis(1));
            42
        });

        assert_eq!(result, 42);
        assert!(Profiler::<DefaultCategory>::has_data());

        let stats = Profiler::<DefaultCategory>::get_stats("test_operation");
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

        Profiler::<TestCategory>::reset();

        Profiler::<TestCategory>::record_with_category("op1", TestCategory::Fast, 100);
        Profiler::<TestCategory>::record_with_category("op2", TestCategory::Slow, 1000);

        assert_eq!(Profiler::<TestCategory>::total_operations(), 2);
    }

    #[tokio::test]
    async fn test_async_profiling() {
        Profiler::<DefaultCategory>::reset();

        let result = Profiler::<DefaultCategory>::time_async("async_test", || async {
            tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
            "async_result"
        })
        .await;

        assert_eq!(result, "async_result");
        assert!(Profiler::<DefaultCategory>::has_data());
    }

    #[test]
    fn test_profile_macro() {
        Profiler::<DefaultCategory>::reset();

        let result = profile!("macro_test", DefaultCategory::Other => {
            std::thread::sleep(std::time::Duration::from_millis(1));
            100
        });

        assert_eq!(result, 100);
        assert!(Profiler::<DefaultCategory>::get_stats("macro_test").is_some());
    }

    #[test]
    fn test_conditional_profiling() {
        Profiler::<DefaultCategory>::reset();

        let should_profile = true;
        let result = profile_if!(should_profile, "conditional_test", DefaultCategory::Other => {
            42
        });

        assert_eq!(result, 42);
        assert!(Profiler::<DefaultCategory>::get_stats("conditional_test").is_some());

        let should_not_profile = false;
        let result2 = profile_if!(should_not_profile, "conditional_test2", DefaultCategory::Other => {
            84
        });

        assert_eq!(result2, 84);
        assert!(Profiler::<DefaultCategory>::get_stats("conditional_test2").is_none());
    }
}
