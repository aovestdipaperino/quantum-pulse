//! # Metrics Module
//!
//! Provides a flexible system for defining and organizing profiling metrics.
//! Users can create their own metric definitions and organize them by categories.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::category::Category;

/// Definition of a profiling metric
#[derive(Debug, Clone)]
pub struct MetricDefinition<C: Category> {
    /// Unique identifier for the metric
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Detailed description of what this metric measures
    pub description: Option<String>,
    /// Category this metric belongs to
    pub category: C,
    /// Expected unit of measurement (e.g., "microseconds", "bytes", "requests")
    pub unit: Option<String>,
    /// Tags for additional metadata
    pub tags: Vec<String>,
}

impl<C: Category> MetricDefinition<C> {
    /// Create a new metric definition
    pub fn new(id: impl Into<String>, name: impl Into<String>, category: C) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: None,
            category,
            unit: None,
            tags: Vec::new(),
        }
    }

    /// Set the description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the unit of measurement
    pub fn with_unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = Some(unit.into());
        self
    }

    /// Add a tag
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Add multiple tags
    pub fn with_tags(mut self, tags: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.tags.extend(tags.into_iter().map(|t| t.into()));
        self
    }
}

/// A registry for managing metric definitions
pub struct MetricRegistry<C: Category> {
    metrics: Arc<RwLock<HashMap<String, MetricDefinition<C>>>>,
}

impl<C: Category> MetricRegistry<C> {
    /// Create a new metric registry
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a new metric definition
    pub fn register(&self, metric: MetricDefinition<C>) -> Result<(), MetricRegistryError> {
        let mut metrics = self
            .metrics
            .write()
            .map_err(|_| MetricRegistryError::LockError)?;

        if metrics.contains_key(&metric.id) {
            return Err(MetricRegistryError::DuplicateMetric(metric.id.clone()));
        }

        metrics.insert(metric.id.clone(), metric);
        Ok(())
    }

    /// Register multiple metrics at once
    pub fn register_all(
        &self,
        metrics: impl IntoIterator<Item = MetricDefinition<C>>,
    ) -> Result<(), MetricRegistryError> {
        for metric in metrics {
            self.register(metric)?;
        }
        Ok(())
    }

    /// Get a metric definition by ID
    pub fn get(&self, id: &str) -> Option<MetricDefinition<C>> {
        self.metrics.read().ok()?.get(id).cloned()
    }

    /// Get all metrics in a specific category
    pub fn get_by_category(&self, category: &C) -> Vec<MetricDefinition<C>> {
        self.metrics
            .read()
            .ok()
            .map(|metrics| {
                metrics
                    .values()
                    .filter(|m| &m.category == category)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get all metrics with a specific tag
    pub fn get_by_tag(&self, tag: &str) -> Vec<MetricDefinition<C>> {
        self.metrics
            .read()
            .ok()
            .map(|metrics| {
                metrics
                    .values()
                    .filter(|m| m.tags.contains(&tag.to_string()))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get all registered metrics
    pub fn all(&self) -> Vec<MetricDefinition<C>> {
        self.metrics
            .read()
            .ok()
            .map(|metrics| metrics.values().cloned().collect())
            .unwrap_or_default()
    }

    /// Get all metric IDs
    pub fn ids(&self) -> Vec<String> {
        self.metrics
            .read()
            .ok()
            .map(|metrics| metrics.keys().cloned().collect())
            .unwrap_or_default()
    }

    /// Remove a metric definition
    pub fn unregister(&self, id: &str) -> Option<MetricDefinition<C>> {
        self.metrics.write().ok()?.remove(id)
    }

    /// Clear all metric definitions
    pub fn clear(&self) {
        if let Ok(mut metrics) = self.metrics.write() {
            metrics.clear();
        }
    }

    /// Get the number of registered metrics
    pub fn len(&self) -> usize {
        self.metrics.read().ok().map(|m| m.len()).unwrap_or(0)
    }

    /// Check if the registry is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<C: Category> Default for MetricRegistry<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: Category> Clone for MetricRegistry<C> {
    fn clone(&self) -> Self {
        Self {
            metrics: Arc::clone(&self.metrics),
        }
    }
}

/// Errors that can occur when working with the metric registry
#[derive(Debug, Clone, PartialEq)]
pub enum MetricRegistryError {
    /// A metric with the same ID already exists
    DuplicateMetric(String),
    /// Failed to acquire lock on the registry
    LockError,
}

impl std::fmt::Display for MetricRegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetricRegistryError::DuplicateMetric(id) => {
                write!(f, "Metric with ID '{}' already exists", id)
            }
            MetricRegistryError::LockError => {
                write!(f, "Failed to acquire lock on metric registry")
            }
        }
    }
}

impl std::error::Error for MetricRegistryError {}

/// Builder for creating a set of metric definitions
pub struct MetricSetBuilder<C: Category> {
    metrics: Vec<MetricDefinition<C>>,
}

impl<C: Category> MetricSetBuilder<C> {
    /// Create a new metric set builder
    pub fn new() -> Self {
        Self {
            metrics: Vec::new(),
        }
    }

    /// Add a metric to the set
    pub fn add(mut self, metric: MetricDefinition<C>) -> Self {
        self.metrics.push(metric);
        self
    }

    /// Add a metric using a builder pattern
    pub fn metric(mut self, id: impl Into<String>, name: impl Into<String>, category: C) -> Self {
        self.metrics.push(MetricDefinition::new(id, name, category));
        self
    }

    /// Add a metric with full details
    pub fn metric_full(
        mut self,
        id: impl Into<String>,
        name: impl Into<String>,
        category: C,
        description: impl Into<String>,
        unit: impl Into<String>,
    ) -> Self {
        self.metrics.push(
            MetricDefinition::new(id, name, category)
                .with_description(description)
                .with_unit(unit),
        );
        self
    }

    /// Build the metric set
    pub fn build(self) -> Vec<MetricDefinition<C>> {
        self.metrics
    }

    /// Register all metrics in the set to a registry
    pub fn register_to(self, registry: &MetricRegistry<C>) -> Result<(), MetricRegistryError> {
        registry.register_all(self.metrics)
    }
}

impl<C: Category> Default for MetricSetBuilder<C> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::category::DefaultCategory;

    #[test]
    fn test_metric_definition() {
        let metric = MetricDefinition::new("test_metric", "Test Metric", DefaultCategory::IO)
            .with_description("A test metric for I/O operations")
            .with_unit("microseconds")
            .with_tag("test")
            .with_tags(["performance", "critical"]);

        assert_eq!(metric.id, "test_metric");
        assert_eq!(metric.name, "Test Metric");
        assert_eq!(metric.category, DefaultCategory::IO);
        assert_eq!(
            metric.description,
            Some("A test metric for I/O operations".to_string())
        );
        assert_eq!(metric.unit, Some("microseconds".to_string()));
        assert_eq!(metric.tags.len(), 3);
        assert!(metric.tags.contains(&"test".to_string()));
        assert!(metric.tags.contains(&"performance".to_string()));
        assert!(metric.tags.contains(&"critical".to_string()));
    }

    #[test]
    fn test_metric_registry() {
        let registry = MetricRegistry::new();

        let metric1 = MetricDefinition::new("metric1", "Metric 1", DefaultCategory::IO);
        let metric2 = MetricDefinition::new("metric2", "Metric 2", DefaultCategory::Compute);

        registry.register(metric1.clone()).unwrap();
        registry.register(metric2.clone()).unwrap();

        assert_eq!(registry.len(), 2);
        assert!(!registry.is_empty());

        let retrieved = registry.get("metric1");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "Metric 1");

        let io_metrics = registry.get_by_category(&DefaultCategory::IO);
        assert_eq!(io_metrics.len(), 1);
        assert_eq!(io_metrics[0].id, "metric1");

        let ids = registry.ids();
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&"metric1".to_string()));
        assert!(ids.contains(&"metric2".to_string()));
    }

    #[test]
    fn test_duplicate_metric_error() {
        let registry = MetricRegistry::new();

        let metric1 = MetricDefinition::new("same_id", "Metric 1", DefaultCategory::IO);
        let metric2 = MetricDefinition::new("same_id", "Metric 2", DefaultCategory::Compute);

        registry.register(metric1).unwrap();
        let result = registry.register(metric2);

        assert!(result.is_err());
        match result.unwrap_err() {
            MetricRegistryError::DuplicateMetric(id) => assert_eq!(id, "same_id"),
            _ => panic!("Expected DuplicateMetric error"),
        }
    }

    #[test]
    fn test_metric_set_builder() {
        let registry = MetricRegistry::new();

        let metrics = MetricSetBuilder::new()
            .metric("op1", "Operation 1", DefaultCategory::IO)
            .metric("op2", "Operation 2", DefaultCategory::Compute)
            .metric_full(
                "op3",
                "Operation 3",
                DefaultCategory::Memory,
                "Memory allocation operation",
                "bytes",
            )
            .build();

        assert_eq!(metrics.len(), 3);

        registry.register_all(metrics).unwrap();
        assert_eq!(registry.len(), 3);

        let op3 = registry.get("op3").unwrap();
        assert_eq!(
            op3.description,
            Some("Memory allocation operation".to_string())
        );
        assert_eq!(op3.unit, Some("bytes".to_string()));
    }

    #[test]
    fn test_get_by_tag() {
        let registry = MetricRegistry::new();

        let metric1 =
            MetricDefinition::new("m1", "Metric 1", DefaultCategory::IO).with_tag("critical");
        let metric2 = MetricDefinition::new("m2", "Metric 2", DefaultCategory::Compute)
            .with_tag("performance");
        let metric3 = MetricDefinition::new("m3", "Metric 3", DefaultCategory::Memory)
            .with_tags(["critical", "performance"]);

        registry
            .register_all(vec![metric1, metric2, metric3])
            .unwrap();

        let critical = registry.get_by_tag("critical");
        assert_eq!(critical.len(), 2);

        let performance = registry.get_by_tag("performance");
        assert_eq!(performance.len(), 2);
    }

    #[test]
    fn test_unregister_and_clear() {
        let registry = MetricRegistry::new();

        let metric = MetricDefinition::new("test", "Test", DefaultCategory::IO);
        registry.register(metric).unwrap();

        assert_eq!(registry.len(), 1);

        let removed = registry.unregister("test");
        assert!(removed.is_some());
        assert_eq!(registry.len(), 0);

        registry
            .register(MetricDefinition::new("m1", "M1", DefaultCategory::IO))
            .unwrap();
        registry
            .register(MetricDefinition::new("m2", "M2", DefaultCategory::Compute))
            .unwrap();

        assert_eq!(registry.len(), 2);

        registry.clear();
        assert_eq!(registry.len(), 0);
        assert!(registry.is_empty());
    }
}
