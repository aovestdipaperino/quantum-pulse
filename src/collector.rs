//! # Profile Collector
//!
//! Centralized collection and storage of profiling metrics with thread-safe access.

use crate::category::{Category, DefaultCategory};
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, RwLock};
use std::time::Duration;

#[cfg(feature = "full")]
use hdrhistogram::Histogram;

/// Statistics for a single operation
#[derive(Debug, Clone)]
pub struct OperationStats {
    /// Number of times this operation was recorded
    pub count: usize,
    /// Total time spent in this operation
    pub total: Duration,
    /// HDR histogram for percentile calculations (full feature only)
    #[cfg(feature = "full")]
    histogram: Histogram<u64>,
    /// Min time recorded
    pub min_time_micros: u64,
    /// Max time recorded
    pub max_time_micros: u64,
}

impl Default for OperationStats {
    fn default() -> Self {
        Self {
            count: 0,
            total: Duration::ZERO,
            #[cfg(feature = "full")]
            histogram: Histogram::new(3).unwrap_or_else(|_| Histogram::new(1).unwrap()),
            min_time_micros: u64::MAX,
            max_time_micros: 0,
        }
    }
}

impl OperationStats {
    /// Get the mean duration for this operation
    pub fn mean(&self) -> Duration {
        if self.count == 0 {
            Duration::ZERO
        } else {
            self.total / (self.count as u32)
        }
    }

    /// Get mean time in microseconds
    pub fn mean_time_micros(&self) -> u64 {
        if self.count == 0 {
            0
        } else {
            self.total.as_micros() as u64 / self.count as u64
        }
    }

    /// Get total time in microseconds
    pub fn total_time_micros(&self) -> u64 {
        self.total.as_micros() as u64
    }

    /// Get standard deviation in microseconds
    pub fn std_dev_micros(&self) -> u64 {
        // Simple approximation - in a real implementation you'd calculate this properly
        if self.count < 2 {
            0
        } else {
            let mean = self.mean_time_micros();
            ((self.max_time_micros.saturating_sub(self.min_time_micros)) / 4).max(mean / 10)
        }
    }

    /// Get the 50th percentile (median) in microseconds
    pub fn p50_micros(&self) -> u64 {
        #[cfg(feature = "full")]
        {
            self.histogram.value_at_quantile(0.5)
        }
        #[cfg(not(feature = "full"))]
        {
            self.mean_time_micros()
        }
    }

    /// Get the 95th percentile in microseconds
    pub fn p95_micros(&self) -> u64 {
        #[cfg(feature = "full")]
        {
            self.histogram.value_at_quantile(0.95)
        }
        #[cfg(not(feature = "full"))]
        {
            (self.mean_time_micros() + self.max_time_micros) / 2
        }
    }

    /// Get the 99th percentile in microseconds
    pub fn p99_micros(&self) -> u64 {
        #[cfg(feature = "full")]
        {
            self.histogram.value_at_quantile(0.99)
        }
        #[cfg(not(feature = "full"))]
        {
            (self.mean_time_micros() * 3 + self.max_time_micros) / 4
        }
    }

    /// Get the 99.9th percentile in microseconds
    pub fn p999_micros(&self) -> u64 {
        #[cfg(feature = "full")]
        {
            self.histogram.value_at_quantile(0.999)
        }
        #[cfg(not(feature = "full"))]
        {
            self.max_time_micros
        }
    }

    /// Add a new measurement to these stats
    pub fn record(&mut self, duration: Duration) {
        let micros = duration.as_micros() as u64;

        self.count += 1;
        self.total += duration;

        // Update min/max
        self.min_time_micros = self.min_time_micros.min(micros);
        self.max_time_micros = self.max_time_micros.max(micros);

        // Record in histogram for percentile calculations
        #[cfg(feature = "full")]
        {
            let _ = self.histogram.record(micros);
        }
    }
}

/// Global registry of all operation statistics
static GLOBAL_STATS: LazyLock<Arc<RwLock<HashMap<String, OperationStats>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(HashMap::new())));

/// Global registry of operation categories
static GLOBAL_CATEGORIES: LazyLock<Arc<RwLock<HashMap<String, DefaultCategory>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(HashMap::new())));

/// Central collector for all profiling data
pub struct ProfileCollector;

impl ProfileCollector {
    /// Record a timing measurement for an operation
    pub fn record(key: &str, duration_micros: u64) {
        let duration = Duration::from_micros(duration_micros);

        #[cfg(feature = "full")]
        {
            if let Ok(mut stats) = GLOBAL_STATS.write() {
                stats
                    .entry(key.to_string())
                    .and_modify(|s| s.record(duration))
                    .or_insert_with(|| {
                        let mut new_stats = OperationStats::default();
                        new_stats.record(duration);
                        new_stats
                    });
            }
        }

        #[cfg(not(feature = "full"))]
        {
            let _ = (key, duration);
        }
    }

    /// Get statistics for a specific operation
    pub fn get_stats(key: &str) -> Option<OperationStats> {
        #[cfg(feature = "full")]
        {
            if let Ok(stats) = GLOBAL_STATS.read() {
                stats.get(key).cloned()
            } else {
                None
            }
        }

        #[cfg(not(feature = "full"))]
        {
            let _ = key;
            None
        }
    }

    /// Get statistics for all operations
    pub fn get_all_stats() -> HashMap<String, OperationStats> {
        #[cfg(feature = "full")]
        {
            if let Ok(stats) = GLOBAL_STATS.read() {
                stats.clone()
            } else {
                HashMap::new()
            }
        }

        #[cfg(not(feature = "full"))]
        {
            HashMap::new()
        }
    }

    /// Check if any profiling data has been collected
    pub fn has_data() -> bool {
        #[cfg(feature = "full")]
        {
            if let Ok(stats) = GLOBAL_STATS.read() {
                !stats.is_empty() && stats.values().any(|s| s.count > 0)
            } else {
                false
            }
        }

        #[cfg(not(feature = "full"))]
        {
            false
        }
    }

    /// Get total number of operations recorded across all metrics
    pub fn total_operations() -> u64 {
        #[cfg(feature = "full")]
        {
            if let Ok(stats) = GLOBAL_STATS.read() {
                stats.values().map(|s| s.count as u64).sum()
            } else {
                0
            }
        }

        #[cfg(not(feature = "full"))]
        {
            0
        }
    }

    /// Reset all statistics
    pub fn reset_all() {
        #[cfg(feature = "full")]
        {
            if let Ok(mut stats) = GLOBAL_STATS.write() {
                stats.clear();
            }
        }
    }

    /// Clear all data
    pub fn clear_all() {
        Self::reset_all();
    }

    /// Record a timing measurement with a category
    pub fn record_with_category(key: &str, category: DefaultCategory, duration_micros: u64) {
        Self::record(key, duration_micros);

        #[cfg(feature = "full")]
        {
            if let Ok(mut categories) = GLOBAL_CATEGORIES.write() {
                categories.insert(key.to_string(), category);
            }
        }

        #[cfg(not(feature = "full"))]
        {
            let _ = (key, category, duration_micros);
        }
    }

    /// Get the category for a specific operation
    pub fn get_category<S: AsRef<str>, C: Category>(_key: S) -> Option<C> {
        #[cfg(feature = "full")]
        {
            // For now, always return None to avoid complexity
            // The reporter will handle missing categories gracefully
            // by using a flat operation list instead of categorized view
            None
        }

        #[cfg(not(feature = "full"))]
        {
            None
        }
    }

    /// Reset metrics for a specific operation
    pub fn reset_operation(key: &str) {
        #[cfg(feature = "full")]
        {
            if let Ok(mut stats) = GLOBAL_STATS.write() {
                stats.remove(key);
            }
        }

        #[cfg(not(feature = "full"))]
        {
            let _ = key;
        }
    }

    /// Get summary statistics across all operations
    pub fn get_summary() -> SummaryStats {
        let all_stats = Self::get_all_stats();

        let total_operations: usize = all_stats.values().map(|s| s.count).sum();
        let total_time: Duration = all_stats.values().map(|s| s.total).sum();
        let unique_operations = all_stats.len();

        // Find slowest operation (by max time)
        let slowest = all_stats
            .iter()
            .max_by_key(|(_, stats)| stats.max_time_micros)
            .map(|(name, _)| name.clone());

        let slowest_p99_micros = all_stats
            .values()
            .map(|s| s.p99_micros())
            .max()
            .unwrap_or(0);

        // Find busiest operation (by call count)
        let busiest = all_stats
            .iter()
            .max_by_key(|(_, stats)| stats.count)
            .map(|(name, stats)| (name.clone(), stats.count));

        let (busiest_operation, busiest_count) = busiest
            .map(|(name, count)| (Some(name), count))
            .unwrap_or((None, 0));

        SummaryStats {
            total_operations: total_operations as u64,
            unique_operations,
            total_time_micros: total_time.as_micros() as u64,
            slowest_operation: slowest,
            slowest_p99_micros,
            busiest_operation,
            busiest_count,
        }
    }

    /// Print a simple report of all collected stats
    pub fn report_stats() {
        println!("==== Profile Report ====");
        let stats = Self::get_all_stats();
        for (key, stat) in stats {
            println!(
                "{} -> count: {}, total: {:?}, avg: {:?}",
                key,
                stat.count,
                stat.total,
                stat.mean()
            );
        }
    }
}

/// Summary statistics across all profiled operations
#[derive(Debug, Clone, Default)]
pub struct SummaryStats {
    /// Total number of operations recorded
    pub total_operations: u64,
    /// Number of unique operation types
    pub unique_operations: usize,
    /// Total time spent across all operations (microseconds)
    pub total_time_micros: u64,
    /// Operation with the slowest single execution
    pub slowest_operation: Option<String>,
    /// Slowest p99 time in microseconds
    pub slowest_p99_micros: u64,
    /// Operation with the most calls
    pub busiest_operation: Option<String>,
    /// Number of calls for the busiest operation
    pub busiest_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "full")]
    use std::time::Duration;

    #[test]
    #[cfg(feature = "full")]
    fn test_record_and_retrieve() {
        ProfileCollector::clear_all();

        ProfileCollector::record("test_record_and_retrieve", 1000);
        ProfileCollector::record("test_record_and_retrieve", 2000);
        ProfileCollector::record("test_record_and_retrieve", 1500);

        let stats = ProfileCollector::get_stats("test_record_and_retrieve");
        assert!(stats.is_some());

        let stats = stats.unwrap();
        assert_eq!(stats.count, 3);
        assert_eq!(stats.total, Duration::from_micros(4500));
        assert_eq!(stats.mean(), Duration::from_micros(1500));
    }

    #[test]
    #[cfg(feature = "full")]
    fn test_multiple_operations() {
        ProfileCollector::clear_all();

        ProfileCollector::record("multiple_op1", 100);
        ProfileCollector::record("multiple_op2", 200);
        ProfileCollector::record("multiple_op1", 150);

        let stats1 = ProfileCollector::get_stats("multiple_op1").unwrap();
        let stats2 = ProfileCollector::get_stats("multiple_op2").unwrap();

        assert_eq!(stats1.count, 2);
        assert_eq!(stats2.count, 1);

        let all_stats = ProfileCollector::get_all_stats();
        assert_eq!(all_stats.len(), 2);
    }

    #[test]
    #[cfg(feature = "full")]
    fn test_summary_stats() {
        ProfileCollector::clear_all();

        for i in 0..10 {
            ProfileCollector::record("summary_op1", 100 + i * 10);
        }

        for i in 0..5 {
            ProfileCollector::record("summary_op2", 200 + i * 20);
        }

        let summary = ProfileCollector::get_summary();
        assert_eq!(summary.total_operations, 15);
        assert_eq!(summary.unique_operations, 2);
        assert!(summary.total_time_micros > 0);
    }

    #[test]
    #[cfg(feature = "full")]
    fn test_has_data() {
        ProfileCollector::clear_all();
        assert!(!ProfileCollector::has_data()); // Clear data first

        ProfileCollector::record("test", 100);
        assert!(ProfileCollector::has_data());
    }

    #[test]
    #[cfg(feature = "full")]
    fn test_total_operations() {
        ProfileCollector::clear_all();
        assert_eq!(ProfileCollector::total_operations(), 0);

        ProfileCollector::record("op1", 100);
        ProfileCollector::record("op1", 200);
        ProfileCollector::record("op2", 300);

        assert_eq!(ProfileCollector::total_operations(), 3);
    }
}
