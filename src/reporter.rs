//! # Profile Reporter
//!
//! Provides flexible reporting capabilities for profiling data with
//! multiple output formats and customizable presentation options.

use std::collections::HashMap;
use std::fmt;

use crate::category::Category;
use crate::collector::{OperationStats, ProfileCollector, SummaryStats};

/// Configuration for generating profile reports
#[derive(Debug, Clone)]
pub struct ReportConfig {
    /// Include percentile statistics (p50, p95, p99, p99.9)
    pub include_percentiles: bool,
    /// Sort operations by time instead of alphabetically
    pub sort_by_time: bool,
    /// Sort by a specific percentile (e.g., "p99")
    pub sort_by_percentile: Option<Percentile>,
    /// Minimum number of samples required to include an operation
    pub min_samples: u64,
    /// Group operations by category
    pub group_by_category: bool,
    /// Include summary statistics
    pub include_summary: bool,
    /// Time format for displaying durations
    pub time_format: TimeFormat,
    /// Maximum number of operations to display (0 = unlimited)
    pub max_operations: usize,
    /// Include operations with zero samples
    pub include_empty: bool,
}

impl Default for ReportConfig {
    fn default() -> Self {
        Self {
            include_percentiles: true,
            sort_by_time: false,
            sort_by_percentile: None,
            min_samples: 0,
            group_by_category: true,
            include_summary: true,
            time_format: TimeFormat::Auto,
            max_operations: 0,
            include_empty: false,
        }
    }
}

/// Percentile options for sorting
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Percentile {
    P50,
    P95,
    P99,
    P999,
}

/// Time format for displaying durations
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimeFormat {
    /// Display in microseconds
    Microseconds,
    /// Display in milliseconds
    Milliseconds,
    /// Display in seconds
    Seconds,
    /// Automatically choose based on magnitude
    Auto,
}

impl TimeFormat {
    /// Format a time value according to the format setting
    pub fn format_time(&self, micros: u64) -> String {
        match self {
            TimeFormat::Microseconds => format!("{} µs", micros),
            TimeFormat::Milliseconds => format!("{:.2} ms", micros as f64 / 1000.0),
            TimeFormat::Seconds => format!("{:.3} s", micros as f64 / 1_000_000.0),
            TimeFormat::Auto => {
                if micros < 1000 {
                    format!("{} µs", micros)
                } else if micros < 1_000_000 {
                    format!("{:.2} ms", micros as f64 / 1000.0)
                } else {
                    format!("{:.3} s", micros as f64 / 1_000_000.0)
                }
            }
        }
    }

    /// Format a floating-point time value
    pub fn format_time_f64(&self, micros: f64) -> String {
        match self {
            TimeFormat::Microseconds => format!("{:.1} µs", micros),
            TimeFormat::Milliseconds => format!("{:.2} ms", micros / 1000.0),
            TimeFormat::Seconds => format!("{:.3} s", micros / 1_000_000.0),
            TimeFormat::Auto => {
                if micros < 1000.0 {
                    format!("{:.1} µs", micros)
                } else if micros < 1_000_000.0 {
                    format!("{:.2} ms", micros / 1000.0)
                } else {
                    format!("{:.3} s", micros / 1_000_000.0)
                }
            }
        }
    }
}

/// A comprehensive profiling report
pub struct ProfileReport<C: Category = crate::category::DefaultCategory> {
    config: ReportConfig,
    stats: HashMap<String, OperationStats>,
    categories: HashMap<String, C>,
    generated_at: std::time::SystemTime,
}

impl<C: Category> ProfileReport<C> {
    /// Generate a report with default configuration
    pub fn generate() -> Self {
        Self::generate_with_config(ReportConfig::default())
    }

    /// Generate a report with custom configuration
    pub fn generate_with_config(config: ReportConfig) -> Self {
        let stats = ProfileCollector::get_all_stats();
        let mut categories = HashMap::new();

        for operation in stats.keys() {
            if let Some(category) = ProfileCollector::get_category::<_, C>(operation) {
                categories.insert(operation.clone(), category);
            }
        }

        Self {
            config,
            stats,
            categories,
            generated_at: std::time::SystemTime::now(),
        }
    }

    /// Get a quick summary string
    pub fn quick_summary(&self) -> String {
        let summary = ProfileCollector::get_summary();
        format!(
            "Operations: {} | Total calls: {} | Total time: {}",
            summary.unique_operations,
            summary.total_operations,
            self.config
                .time_format
                .format_time(summary.total_time_micros)
        )
    }

    /// Get detailed summary statistics
    pub fn summary_stats(&self) -> SummaryStats {
        ProfileCollector::get_summary()
    }

    /// Convert the report to a console-friendly string
    pub fn to_console_string(&self) -> String {
        let mut output = String::new();

        // Header
        output.push_str("╔════════════════════════════════════════════════╗\n");
        output.push_str("║           PROFILING REPORT                     ║\n");
        output.push_str("╚════════════════════════════════════════════════╝\n\n");

        // Summary
        if self.config.include_summary {
            output.push_str(&self.format_summary());
            output.push_str("\n");
        }

        // Operations
        if self.config.group_by_category {
            output.push_str(&self.format_by_category());
        } else {
            output.push_str(&self.format_all_operations());
        }

        // Footer
        output.push_str(&format!("\nGenerated at: {:?}\n", self.generated_at));

        output
    }

    fn format_summary(&self) -> String {
        let summary = self.summary_stats();
        let mut output = String::new();

        output.push_str("═══ Summary ═══\n");
        output.push_str(&format!("Total Operations: {}\n", summary.total_operations));
        output.push_str(&format!(
            "Unique Operations: {}\n",
            summary.unique_operations
        ));
        output.push_str(&format!(
            "Total Time: {}\n",
            self.config
                .time_format
                .format_time(summary.total_time_micros)
        ));

        if !summary.slowest_operation.is_empty() {
            output.push_str(&format!(
                "Slowest Operation: {} (p99: {})\n",
                summary.slowest_operation,
                self.config
                    .time_format
                    .format_time(summary.slowest_p99_micros)
            ));
        }

        if !summary.busiest_operation.is_empty() {
            output.push_str(&format!(
                "Busiest Operation: {} ({} calls)\n",
                summary.busiest_operation, summary.busiest_count
            ));
        }

        output
    }

    fn format_by_category(&self) -> String {
        let mut output = String::new();
        let mut categorized: HashMap<C, Vec<(&String, &OperationStats)>> = HashMap::new();

        // Group operations by category
        for (operation, stats) in &self.stats {
            if stats.count < self.config.min_samples && !self.config.include_empty {
                continue;
            }

            let category = self.categories.get(operation).cloned().unwrap_or_else(|| {
                ProfileCollector::get_category::<_, C>(operation).unwrap_or_else(|| {
                    // This is a bit of a hack - we need a default category
                    // In real usage, every operation should have a category
                    panic!("Operation {} has no category", operation)
                })
            });

            categorized
                .entry(category)
                .or_insert_with(Vec::new)
                .push((operation, stats));
        }

        // Sort categories by priority
        let mut categories: Vec<_> = categorized.keys().cloned().collect();
        categories.sort_by_key(|c| c.priority());

        // Format each category
        for category in categories {
            if let Some(operations) = categorized.get(&category) {
                output.push_str(&format!("\n═══ {:?} ═══\n", category));
                if let Some(desc) = category.description() {
                    output.push_str(&format!("  {}\n", desc));
                }
                output.push_str(&self.format_operations_table(operations));
            }
        }

        output
    }

    fn format_all_operations(&self) -> String {
        let operations: Vec<_> = self
            .stats
            .iter()
            .filter(|(_, stats)| {
                stats.count >= self.config.min_samples || self.config.include_empty
            })
            .collect();

        self.format_operations_table(&operations)
    }

    fn format_operations_table(&self, operations: &[(&String, &OperationStats)]) -> String {
        if operations.is_empty() {
            return "  No operations recorded\n".to_string();
        }

        let mut sorted_ops = operations.to_vec();

        // Sort operations
        if self.config.sort_by_time {
            sorted_ops.sort_by(|a, b| {
                b.1.mean_time_micros
                    .partial_cmp(&a.1.mean_time_micros)
                    .unwrap()
            });
        } else if let Some(percentile) = self.config.sort_by_percentile {
            match percentile {
                Percentile::P50 => {
                    sorted_ops.sort_by_key(|(_, stats)| std::cmp::Reverse(stats.p50_micros))
                }
                Percentile::P95 => {
                    sorted_ops.sort_by_key(|(_, stats)| std::cmp::Reverse(stats.p95_micros))
                }
                Percentile::P99 => {
                    sorted_ops.sort_by_key(|(_, stats)| std::cmp::Reverse(stats.p99_micros))
                }
                Percentile::P999 => {
                    sorted_ops.sort_by_key(|(_, stats)| std::cmp::Reverse(stats.p999_micros))
                }
            }
        } else {
            sorted_ops.sort_by_key(|(name, _)| name.as_str());
        }

        // Apply max operations limit
        if self.config.max_operations > 0 && sorted_ops.len() > self.config.max_operations {
            sorted_ops.truncate(self.config.max_operations);
        }

        let mut output = String::new();

        // Table header
        if self.config.include_percentiles {
            output.push_str("  Operation                          Count      Mean       P50        P95        P99        P99.9\n");
            output.push_str("  ─────────────────────────────────────────────────────────────────────────────────────────────\n");
        } else {
            output.push_str(
                "  Operation                          Count      Mean       Min        Max\n",
            );
            output.push_str(
                "  ──────────────────────────────────────────────────────────────────────\n",
            );
        }

        // Table rows
        for (operation, stats) in sorted_ops {
            let name = if operation.len() > 35 {
                format!("{}...", &operation[..32])
            } else {
                format!("{:?}", operation)
            };

            if self.config.include_percentiles {
                output.push_str(&format!(
                    "  {:<35} {:>8} {:>10} {:>10} {:>10} {:>10} {:>10}\n",
                    name,
                    stats.count,
                    self.config
                        .time_format
                        .format_time_f64(stats.mean_time_micros),
                    self.config.time_format.format_time(stats.p50_micros),
                    self.config.time_format.format_time(stats.p95_micros),
                    self.config.time_format.format_time(stats.p99_micros),
                    self.config.time_format.format_time(stats.p999_micros),
                ));
            } else {
                output.push_str(&format!(
                    "  {:<35} {:>8} {:>10} {:>10} {:>10}\n",
                    name,
                    stats.count,
                    self.config
                        .time_format
                        .format_time_f64(stats.mean_time_micros),
                    self.config.time_format.format_time(stats.min_time_micros),
                    self.config.time_format.format_time(stats.max_time_micros),
                ));
            }
        }

        output
    }

    /// Convert the report to CSV format
    pub fn to_csv(&self) -> String {
        let mut csv = String::new();

        // Header
        csv.push_str("Operation,Category,Count,Mean (µs),Min (µs),Max (µs),P50 (µs),P95 (µs),P99 (µs),P99.9 (µs),Std Dev (µs)\n");

        // Sort operations
        let mut operations: Vec<_> = self.stats.iter().collect();
        operations.sort_by_key(|(name, _)| name.as_str());

        // Data rows
        for (name, stats) in operations {
            if stats.count < self.config.min_samples && !self.config.include_empty {
                continue;
            }

            let category = self
                .categories
                .get(name)
                .map(|c| format!("{:?}", c))
                .unwrap_or_else(|| "Uncategorized".to_string());

            csv.push_str(&format!(
                "{},{},{},{:.2},{},{},{},{},{},{},{:.2}\n",
                name,
                category,
                stats.count,
                stats.mean_time_micros,
                stats.min_time_micros,
                stats.max_time_micros,
                stats.p50_micros,
                stats.p95_micros,
                stats.p99_micros,
                stats.p999_micros,
                stats.std_dev_micros,
            ));
        }

        csv
    }

    /// Get operations sorted by a specific metric
    pub fn top_operations_by(
        &self,
        metric: SortMetric,
        limit: usize,
    ) -> Vec<(String, OperationStats)> {
        let mut operations: Vec<_> = self
            .stats
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        match metric {
            SortMetric::Count => {
                operations.sort_by_key(|(_, stats)| std::cmp::Reverse(stats.count))
            }
            SortMetric::TotalTime => {
                operations.sort_by_key(|(_, stats)| std::cmp::Reverse(stats.total_time_micros))
            }
            SortMetric::MeanTime => operations.sort_by(|a, b| {
                b.1.mean_time_micros
                    .partial_cmp(&a.1.mean_time_micros)
                    .unwrap()
            }),
            SortMetric::P50 => {
                operations.sort_by_key(|(_, stats)| std::cmp::Reverse(stats.p50_micros))
            }
            SortMetric::P95 => {
                operations.sort_by_key(|(_, stats)| std::cmp::Reverse(stats.p95_micros))
            }
            SortMetric::P99 => {
                operations.sort_by_key(|(_, stats)| std::cmp::Reverse(stats.p99_micros))
            }
        }

        operations.truncate(limit);
        operations
    }
}

impl<C: Category> fmt::Display for ProfileReport<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_console_string())
    }
}

/// Metrics for sorting operations
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortMetric {
    Count,
    TotalTime,
    MeanTime,
    P50,
    P95,
    P99,
}

/// Builder for creating customized reports
pub struct ReportBuilder<C: Category = crate::category::DefaultCategory> {
    config: ReportConfig,
    _phantom: std::marker::PhantomData<C>,
}

impl<C: Category> ReportBuilder<C> {
    /// Create a new report builder
    pub fn new() -> Self {
        Self {
            config: ReportConfig::default(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Set whether to include percentiles
    pub fn include_percentiles(mut self, include: bool) -> Self {
        self.config.include_percentiles = include;
        self
    }

    /// Set whether to sort by time
    pub fn sort_by_time(mut self, sort: bool) -> Self {
        self.config.sort_by_time = sort;
        self
    }

    /// Sort by a specific percentile
    pub fn sort_by_percentile(mut self, percentile: Percentile) -> Self {
        self.config.sort_by_percentile = Some(percentile);
        self
    }

    /// Set minimum samples required
    pub fn min_samples(mut self, min: u64) -> Self {
        self.config.min_samples = min;
        self
    }

    /// Set whether to group by category
    pub fn group_by_category(mut self, group: bool) -> Self {
        self.config.group_by_category = group;
        self
    }

    /// Set whether to include summary
    pub fn include_summary(mut self, include: bool) -> Self {
        self.config.include_summary = include;
        self
    }

    /// Set time format
    pub fn time_format(mut self, format: TimeFormat) -> Self {
        self.config.time_format = format;
        self
    }

    /// Set maximum operations to display
    pub fn max_operations(mut self, max: usize) -> Self {
        self.config.max_operations = max;
        self
    }

    /// Set whether to include empty operations
    pub fn include_empty(mut self, include: bool) -> Self {
        self.config.include_empty = include;
        self
    }

    /// Generate the report
    pub fn build(self) -> ProfileReport<C> {
        ProfileReport::generate_with_config(self.config)
    }
}

impl<C: Category> Default for ReportBuilder<C> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::category::DefaultCategory;
    use crate::ProfileCollector;

    #[test]
    fn test_time_format() {
        assert_eq!(TimeFormat::Microseconds.format_time(500), "500 µs");
        assert_eq!(TimeFormat::Milliseconds.format_time(1500), "1.50 ms");
        assert_eq!(TimeFormat::Seconds.format_time(1_500_000), "1.500 s");

        assert_eq!(TimeFormat::Auto.format_time(500), "500 µs");
        assert_eq!(TimeFormat::Auto.format_time(1500), "1.50 ms");
        assert_eq!(TimeFormat::Auto.format_time(1_500_000), "1.500 s");
    }

    #[test]
    fn test_report_generation() {
        ProfileCollector::clear_all();

        ProfileCollector::record_with_category("test_op1", DefaultCategory::IO, 1000);
        ProfileCollector::record_with_category("test_op1", DefaultCategory::IO, 2000);
        ProfileCollector::record_with_category("test_op2", DefaultCategory::Compute, 500);

        let report = ProfileReport::<DefaultCategory>::generate();
        assert!(!report.stats.is_empty());

        let summary = report.quick_summary();
        assert!(summary.contains("Operations: 2"));
        assert!(summary.contains("Total calls: 3"));
    }

    #[test]
    fn test_report_builder() {
        ProfileCollector::clear_all();

        ProfileCollector::record("test_op", 1000);

        let report = ReportBuilder::<DefaultCategory>::new()
            .include_percentiles(false)
            .sort_by_time(true)
            .min_samples(1)
            .group_by_category(false)
            .time_format(TimeFormat::Milliseconds)
            .build();

        assert!(!report.config.include_percentiles);
        assert!(report.config.sort_by_time);
        assert_eq!(report.config.min_samples, 1);
        assert_eq!(report.config.time_format, TimeFormat::Milliseconds);
    }

    #[test]
    fn test_csv_output() {
        ProfileCollector::clear_all();

        ProfileCollector::record_with_category("csv_test", DefaultCategory::IO, 1000);

        let report = ProfileReport::<DefaultCategory>::generate();
        let csv = report.to_csv();

        assert!(csv.contains("Operation,Category,Count"));
        assert!(csv.contains("csv_test"));
    }

    #[test]
    fn test_top_operations() {
        ProfileCollector::clear_all();

        for i in 0..5 {
            ProfileCollector::record("op1", 100 * (i + 1));
        }
        for i in 0..3 {
            ProfileCollector::record("op2", 200 * (i + 1));
        }
        ProfileCollector::record("op3", 1000);

        let report = ProfileReport::<DefaultCategory>::generate();
        let top_by_count = report.top_operations_by(SortMetric::Count, 2);

        assert_eq!(top_by_count.len(), 2);
        assert_eq!(top_by_count[0].0, "op1");
        assert_eq!(top_by_count[0].1.count, 5);
    }
}
