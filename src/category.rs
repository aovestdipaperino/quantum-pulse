//! # Category Module
//!
//! Provides traits and types for categorizing profiling operations.
//! Users can define their own categories by implementing the `Category` trait.

use std::fmt;
use std::hash::Hash;

/// Trait for defining custom profiling categories
///
/// Implement this trait to create your own operation categories
/// that can be used to group and organize profiling metrics.
///
/// # Example
/// ```rust
/// use profile_timer::Category;
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// enum MyCategory {
///     Database,
///     Network,
///     Computation,
///     Cache,
/// }
///
/// impl Category for MyCategory {
///     fn description(&self) -> Option<&str> {
///         match self {
///             MyCategory::Database => Some("All database-related operations"),
///             MyCategory::Network => Some("Network communication and I/O"),
///             MyCategory::Computation => Some("CPU-intensive calculations"),
///             MyCategory::Cache => Some("Cache reads and writes"),
///         }
///     }
/// }
/// ```
pub trait Category: Clone + Eq + Hash + Send + Sync + std::fmt::Debug + 'static {
    /// Optional description of the category
    fn description(&self) -> Option<&str> {
        None
    }

    /// Optional color hint for visualization (e.g., "#FF5733")
    fn color_hint(&self) -> Option<&str> {
        None
    }

    /// Priority for sorting categories (lower values appear first)
    fn priority(&self) -> i32 {
        0
    }
}

/// Default category implementation for general-purpose profiling
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DefaultCategory {
    /// I/O operations (file, network, etc.)
    IO,
    /// Computation and CPU-intensive operations
    Compute,
    /// Memory operations (allocation, copying, etc.)
    Memory,
    /// Synchronization operations (locks, channels, etc.)
    Sync,
    /// System operations
    System,
    /// User-defined operations
    Custom(String),
    /// Uncategorized operations
    Other,
}

impl Category for DefaultCategory {
    fn description(&self) -> Option<&str> {
        match self {
            DefaultCategory::IO => Some("Input/Output operations including file and network I/O"),
            DefaultCategory::Compute => Some("CPU-intensive calculations and processing"),
            DefaultCategory::Memory => Some("Memory allocation, deallocation, and copying"),
            DefaultCategory::Sync => Some("Thread synchronization including locks and channels"),
            DefaultCategory::System => Some("System calls and OS interactions"),
            DefaultCategory::Custom(_) => Some("User-defined custom operations"),
            DefaultCategory::Other => Some("Uncategorized operations"),
        }
    }

    fn color_hint(&self) -> Option<&str> {
        match self {
            DefaultCategory::IO => Some("#3498db"),        // Blue
            DefaultCategory::Compute => Some("#e74c3c"),   // Red
            DefaultCategory::Memory => Some("#9b59b6"),    // Purple
            DefaultCategory::Sync => Some("#f39c12"),      // Orange
            DefaultCategory::System => Some("#2ecc71"),    // Green
            DefaultCategory::Custom(_) => Some("#95a5a6"), // Gray
            DefaultCategory::Other => Some("#34495e"),     // Dark Gray
        }
    }

    fn priority(&self) -> i32 {
        match self {
            DefaultCategory::IO => 1,
            DefaultCategory::Compute => 2,
            DefaultCategory::Memory => 3,
            DefaultCategory::Sync => 4,
            DefaultCategory::System => 5,
            DefaultCategory::Custom(_) => 6,
            DefaultCategory::Other => 7,
        }
    }
}

impl fmt::Display for DefaultCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DefaultCategory::Custom(name) => write!(f, "{}", name),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl DefaultCategory {
    /// Create a custom category with the given name
    pub fn custom(name: impl Into<String>) -> Self {
        DefaultCategory::Custom(name.into())
    }
}

/// A no-op category for when categorization is not needed
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NoCategory;

impl Category for NoCategory {}

impl std::fmt::Debug for NoCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Category registry for managing dynamic categories
pub struct CategoryRegistry<C: Category> {
    categories: Vec<C>,
}

impl<C: Category> CategoryRegistry<C> {
    /// Create a new category registry
    pub fn new() -> Self {
        Self {
            categories: Vec::new(),
        }
    }

    /// Register a new category
    pub fn register(&mut self, category: C) -> &mut Self {
        if !self.categories.contains(&category) {
            self.categories.push(category);
        }
        self
    }

    /// Get all registered categories
    pub fn categories(&self) -> &[C] {
        &self.categories
    }

    /// Get categories sorted by priority
    pub fn sorted_categories(&self) -> Vec<C> {
        let mut sorted = self.categories.clone();
        sorted.sort_by_key(|c| c.priority());
        sorted
    }

    /// Find a category by name
    pub fn find_by_name(&self, name: &str) -> Option<&C> {
        self.categories.iter().find(|c| format!("{:?}", c) == name)
    }
}

impl<C: Category> Default for CategoryRegistry<C> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_category() {
        let io_cat = DefaultCategory::IO;
        assert_eq!(format!("{:?}", io_cat), "IO");
        assert!(io_cat.description().is_some());
        assert!(io_cat.color_hint().is_some());
        assert_eq!(io_cat.priority(), 1);
    }

    #[test]
    fn test_custom_category() {
        let custom = DefaultCategory::custom("My Custom Category");
        assert_eq!(format!("{}", custom), "My Custom Category");
        assert_eq!(custom.priority(), 6);
    }

    #[test]
    fn test_no_category() {
        let no_cat = NoCategory;
        assert_eq!(format!("{:?}", no_cat), "NoCategory");
        assert_eq!(no_cat.description(), None);
    }

    #[test]
    fn test_category_registry() {
        let mut registry = CategoryRegistry::new();
        registry
            .register(DefaultCategory::IO)
            .register(DefaultCategory::Compute)
            .register(DefaultCategory::Memory);

        assert_eq!(registry.categories().len(), 3);

        let sorted = registry.sorted_categories();
        assert_eq!(sorted[0], DefaultCategory::IO);
        assert_eq!(sorted[1], DefaultCategory::Compute);
        assert_eq!(sorted[2], DefaultCategory::Memory);

        let found = registry.find_by_name("I/O Operations");
        assert!(found.is_some());
        assert_eq!(*found.unwrap(), DefaultCategory::IO);
    }

    #[test]
    fn test_custom_category_implementation() {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        enum AppCategory {
            Api,
            Database,
            Cache,
        }

        impl Category for AppCategory {
            fn priority(&self) -> i32 {
                match self {
                    AppCategory::Api => 1,
                    AppCategory::Database => 2,
                    AppCategory::Cache => 3,
                }
            }
        }

        impl std::fmt::Debug for AppCategory {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }

        let api = AppCategory::Api;
        assert_eq!(format!("{:?}", api), "Api");
        assert_eq!(api.priority(), 1);
    }
}
