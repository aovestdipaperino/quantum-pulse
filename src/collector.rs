//! # Profile Collector
//!
//! Centralized collection and storage of profiling metrics with thread-safe access
//! and efficient histogram-based storage.

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, RwLock};

#[cfg(feature = "full")]
use hdrhistogram::Histogram as HdrHistogram;

use crate::category::{Category, DefaultCategory};

/// Statistics for a single operation
#[derive(Debug, Clone)]

pub struct OperationStats {
    /// Number of times this operation was recorded
    pub count: u64,
    /// Total time spent in this operation (microseconds)
    pub total_time_micros: u64,
    /// Minimum time recorded (microseconds)
    pub min_time_micros: u64,
    /// Maximum time recorded (microseconds)
    pub max_time_micros: u64,
    /// Mean time (microseconds)
    pub mean_time_micros: f64,
    /// 50th percentile (median) time (microseconds)
    pub p50_micros: u64,
    /// 95th percentile time (microseconds)
    pub p95_micros: u64,
    /// 99th percentile time (microseconds)
    pub p99_micros: u64,
    /// 99.9th percentile time (microseconds)
    pub p999_micros: u64,
    /// Standard deviation (microseconds)
    pub std_dev_micros: f64,
}

impl Default for OperationStats {
    fn default() -> Self {
        Self {
            count: 0,
            total_time_micros: 0,
            min_time_micros: u64::MAX,
            max_time_micros: 0,
            mean_time_micros: 0.0,
            p50_micros: 0,
            p95_micros: 0,
            p99_micros: 0,
            p999_micros: 0,
            std_dev_micros: 0.0,
        }
    }
}

/// Thread-safe histogram wrapper for profiling data
#[cfg(feature = "full")]
struct ThreadSafeHistogram {
    histogram: RwLock<HdrHistogram<u64>>,
}

#[cfg(feature = "full")]
impl ThreadSafeHistogram {
    fn new() -> Self {
        // Support microsecond precision up to 10 seconds with 3 significant figures
        let histogram =
            HdrHistogram::new_with_bounds(1, 10_000_000, 3).expect("Failed to create histogram");

        Self {
            histogram: RwLock::new(histogram),
        }
    }

    fn record(&self, value_micros: u64) {
        if let Ok(mut hist) = self.histogram.write() {
            let _ = hist.record(value_micros.max(1)); // Ensure minimum value of 1
        }
    }

    fn get_stats(&self) -> OperationStats {
        if let Ok(hist) = self.histogram.read() {
            if hist.len() == 0 {
                return OperationStats::default();
            }

            OperationStats {
                count: hist.len(),
                total_time_micros: (hist.mean() * hist.len() as f64) as u64,
                min_time_micros: hist.min(),
                max_time_micros: hist.max(),
                mean_time_micros: hist.mean(),
                p50_micros: hist.value_at_quantile(0.5),
                p95_micros: hist.value_at_quantile(0.95),
                p99_micros: hist.value_at_quantile(0.99),
                p999_micros: hist.value_at_quantile(0.999),
                std_dev_micros: hist.stdev(),
            }
        } else {
            OperationStats::default()
        }
    }

    fn reset(&self) {
        if let Ok(mut hist) = self.histogram.write() {
            hist.reset();
        }
    }
}

/// No-op version for when profiling is disabled
#[cfg(not(feature = "full"))]
struct ThreadSafeHistogram;

#[cfg(not(feature = "full"))]
impl ThreadSafeHistogram {
    fn new() -> Self {
        Self
    }

    fn record(&self, _value_micros: u64) {
        // No-op when profiling is disabled
    }

    fn get_stats(&self) -> OperationStats {
        OperationStats::default()
    }

    fn reset(&self) {
        // No-op when profiling is disabled
    }
}

/// Metadata for an operation
#[derive(Clone)]
struct OperationMetadata {
    category_type_id: TypeId,
    category: Arc<dyn Any + Send + Sync>,
}

/// Global registry of all operation histograms
static HISTOGRAMS: LazyLock<RwLock<HashMap<String, ThreadSafeHistogram>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

/// Global registry of operation metadata (categories)
static METADATA: LazyLock<RwLock<HashMap<String, OperationMetadata>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

/// Central collector for all profiling data
pub struct ProfileCollector;

impl ProfileCollector {
    /// Record a timing measurement for an operation
    pub fn record<T: std::fmt::Debug>(operation: T, duration_micros: u64) {
        Self::record_with_category(operation, DefaultCategory::Other, duration_micros);
    }

    /// Record a timing measurement with a specific category
    pub fn record_with_category<T, C>(operation: T, category: C, duration_micros: u64)
    where
        T: std::fmt::Debug,
        C: Category,
    {
        #[cfg(feature = "full")]
        {
            let operation_str = format!("{:?}", operation);
            // Try to get existing histogram first (read lock)
            if let Ok(histograms) = HISTOGRAMS.read() {
                if let Some(histogram) = histograms.get(&operation_str) {
                    histogram.record(duration_micros);
                    return;
                }
            }

            // Need to create new histogram (write lock)
            if let Ok(mut histograms) = HISTOGRAMS.write() {
                let histogram = histograms
                    .entry(operation_str.clone())
                    .or_insert_with(ThreadSafeHistogram::new);
                histogram.record(duration_micros);
            }

            // Store category metadata
            if let Ok(mut metadata) = METADATA.write() {
                metadata
                    .entry(format!("{:?}", operation))
                    .or_insert_with(|| OperationMetadata {
                        category_type_id: TypeId::of::<C>(),
                        category: Arc::new(category),
                    });
            }
        }

        #[cfg(not(feature = "full"))]
        {
            let _ = (operation, category, duration_micros);
        }
    }

    /// Get statistics for a specific operation
    pub fn get_stats<T: std::fmt::Debug>(operation: T) -> Option<OperationStats> {
        let operation_str = format!("{:?}", operation);
        if let Ok(histograms) = HISTOGRAMS.read() {
            histograms.get(&operation_str).map(|h| h.get_stats())
        } else {
            None
        }
    }

    /// Get statistics for all operations
    pub fn get_all_stats() -> HashMap<String, OperationStats> {
        let mut stats = HashMap::new();

        if let Ok(histograms) = HISTOGRAMS.read() {
            for (operation, histogram) in histograms.iter() {
                stats.insert(operation.clone(), histogram.get_stats());
            }
        }

        stats
    }

    /// Get statistics grouped by category
    pub fn get_stats_by_category<C>() -> HashMap<C, Vec<(String, OperationStats)>>
    where
        C: Category,
    {
        let mut categorized = HashMap::new();

        if let (Ok(histograms), Ok(metadata)) = (HISTOGRAMS.read(), METADATA.read()) {
            for (operation, histogram) in histograms.iter() {
                if let Some(meta) = metadata.get(operation) {
                    if meta.category_type_id == TypeId::of::<C>() {
                        if let Some(category) = meta.category.downcast_ref::<C>() {
                            categorized
                                .entry(category.clone())
                                .or_insert_with(Vec::new)
                                .push((operation.clone(), histogram.get_stats()));
                        }
                    }
                }
            }
        }

        categorized
    }

    /// Get the category for a specific operation
    pub fn get_category<T, C>(operation: T) -> Option<C>
    where
        T: std::fmt::Debug,
        C: Category,
    {
        let operation_str = format!("{:?}", operation);
        if let Ok(metadata) = METADATA.read() {
            metadata.get(&operation_str).and_then(|meta| {
                if meta.category_type_id == TypeId::of::<C>() {
                    meta.category.downcast_ref::<C>().cloned()
                } else {
                    None
                }
            })
        } else {
            None
        }
    }

    /// Get list of all tracked operations
    pub fn get_operation_names() -> Vec<String> {
        if let Ok(histograms) = HISTOGRAMS.read() {
            histograms.keys().cloned().collect()
        } else {
            Vec::new()
        }
    }

    /// Get operations filtered by category
    pub fn get_operations_by_category<C>(category: &C) -> Vec<String>
    where
        C: Category,
    {
        let mut operations = Vec::new();

        if let Ok(metadata) = METADATA.read() {
            for (operation, meta) in metadata.iter() {
                if meta.category_type_id == TypeId::of::<C>() {
                    if let Some(op_category) = meta.category.downcast_ref::<C>() {
                        if op_category == category {
                            operations.push(operation.clone());
                        }
                    }
                }
            }
        }

        operations
    }

    /// Reset metrics for a specific operation
    pub fn reset_operation<T: std::fmt::Debug>(operation: T) {
        let operation_str = format!("{:?}", operation);
        if let Ok(histograms) = HISTOGRAMS.read() {
            if let Some(histogram) = histograms.get(&operation_str) {
                histogram.reset();
            }
        }
    }

    /// Reset all statistics
    pub fn reset_all() {
        if let Ok(histograms) = HISTOGRAMS.read() {
            for histogram in histograms.values() {
                histogram.reset();
            }
        }
    }

    /// Clear all data including histogram instances
    pub fn clear_all() {
        if let Ok(mut histograms) = HISTOGRAMS.write() {
            histograms.clear();
        }
        if let Ok(mut metadata) = METADATA.write() {
            metadata.clear();
        }
    }

    /// Check if any profiling data has been collected
    pub fn has_data() -> bool {
        if let Ok(histograms) = HISTOGRAMS.read() {
            !histograms.is_empty() && histograms.values().any(|h| h.get_stats().count > 0)
        } else {
            false
        }
    }

    /// Get total number of operations recorded across all metrics
    pub fn total_operations() -> u64 {
        if let Ok(histograms) = HISTOGRAMS.read() {
            histograms.values().map(|h| h.get_stats().count).sum()
        } else {
            0
        }
    }

    /// Get summary statistics across all operations
    pub fn get_summary() -> SummaryStats {
        let all_stats = Self::get_all_stats();

        let total_operations: u64 = all_stats.values().map(|s| s.count).sum();
        let total_time: u64 = all_stats.values().map(|s| s.total_time_micros).sum();
        let unique_operations = all_stats.len();

        let (slowest_operation, slowest_time) = all_stats
            .iter()
            .max_by_key(|(_, stats)| stats.p99_micros)
            .map(|(name, stats)| (name.clone(), stats.p99_micros))
            .unwrap_or_default();

        let (busiest_operation, busiest_count) = all_stats
            .iter()
            .max_by_key(|(_, stats)| stats.count)
            .map(|(name, stats)| (name.clone(), stats.count))
            .unwrap_or_default();

        SummaryStats {
            total_operations,
            total_time_micros: total_time,
            unique_operations,
            slowest_operation,
            slowest_p99_micros: slowest_time,
            busiest_operation,
            busiest_count,
        }
    }
}

/// Summary statistics across all profiled operations
#[derive(Debug, Clone)]

pub struct SummaryStats {
    /// Total number of operations recorded
    pub total_operations: u64,
    /// Total time spent across all operations
    pub total_time_micros: u64,
    /// Number of unique operation types
    pub unique_operations: usize,
    /// Name of the slowest operation (by p99)
    pub slowest_operation: String,
    /// P99 time of the slowest operation
    pub slowest_p99_micros: u64,
    /// Name of the most frequently called operation
    pub busiest_operation: String,
    /// Count of the busiest operation
    pub busiest_count: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::category::DefaultCategory;

    #[test]
    fn test_record_and_retrieve() {
        ProfileCollector::clear_all();

        ProfileCollector::record("test_op", 1000);
        ProfileCollector::record("test_op", 2000);
        ProfileCollector::record("test_op", 1500);

        let stats = ProfileCollector::get_stats("test_op");
        assert!(stats.is_some());

        let stats = stats.unwrap();
        assert_eq!(stats.count, 3);
        assert!(stats.mean_time_micros > 0.0);
    }

    #[test]
    fn test_categories() {
        ProfileCollector::clear_all();

        ProfileCollector::record_with_category("io_op", DefaultCategory::IO, 100);
        ProfileCollector::record_with_category("compute_op", DefaultCategory::Compute, 200);

        let io_category = ProfileCollector::get_category::<DefaultCategory>("io_op");
        assert_eq!(io_category, Some(DefaultCategory::IO));

        let compute_category = ProfileCollector::get_category::<DefaultCategory>("compute_op");
        assert_eq!(compute_category, Some(DefaultCategory::Compute));
    }

    #[test]
    fn test_stats_by_category() {
        ProfileCollector::clear_all();

        ProfileCollector::record_with_category("read_file", DefaultCategory::IO, 100);
        ProfileCollector::record_with_category("write_file", DefaultCategory::IO, 150);
        ProfileCollector::record_with_category("calculate", DefaultCategory::Compute, 200);

        let by_category = ProfileCollector::get_stats_by_category::<DefaultCategory>();

        assert!(by_category.contains_key(&DefaultCategory::IO));
        assert!(by_category.contains_key(&DefaultCategory::Compute));

        let io_ops = &by_category[&DefaultCategory::IO];
        assert_eq!(io_ops.len(), 2);
    }

    #[test]
    fn test_summary_stats() {
        ProfileCollector::clear_all();

        for i in 0..10 {
            ProfileCollector::record("op1", 100 + i * 10);
        }

        for i in 0..5 {
            ProfileCollector::record("op2", 200 + i * 20);
        }

        let summary = ProfileCollector::get_summary();
        assert_eq!(summary.total_operations, 15);
        assert_eq!(summary.unique_operations, 2);
        assert_eq!(summary.busiest_operation, "op1");
        assert_eq!(summary.busiest_count, 10);
    }
}
