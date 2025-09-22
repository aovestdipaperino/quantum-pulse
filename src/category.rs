//! # Category Module
//!
//! Traits for organizing profiling operations into categories.

/// Trait for defining custom profiling categories
///
/// Implement this trait to create your own operation categories
/// that can be used to group and organize profiling metrics.
///
/// # Example
/// ```rust
/// use quantum_pulse::Category;
///
/// #[derive(Debug)]
/// struct DatabaseCategory;
///
/// impl Category for DatabaseCategory {
///     fn get_name(&self) -> &str {
///         "Database"
///     }
///
///     fn get_description(&self) -> &str {
///         "All database-related operations"
///     }
///
///     fn color_hint(&self) -> Option<&str> {
///         Some("#3498db")
///     }
///
///     fn priority(&self) -> i32 {
///         1
///     }
/// }
/// ```
pub trait Category: Send + Sync {
    /// Get the name of this category
    fn get_name(&self) -> &str;

    /// Get a description of this category
    fn get_description(&self) -> &str;

    /// Optional color hint for visualization (e.g., "#FF5733")
    fn color_hint(&self) -> Option<&str> {
        None
    }

    /// Priority for sorting categories (lower values appear first)
    fn priority(&self) -> i32 {
        0
    }
}

/// A no-op category for when categorization is not needed
///
/// This is the default category returned by `Operation::get_category()`
/// unless explicitly overridden.
#[derive(Debug)]
pub struct NoCategory;

impl Category for NoCategory {
    fn get_name(&self) -> &str {
        ""
    }

    fn get_description(&self) -> &str {
        "Default category when none is specified"
    }

    fn color_hint(&self) -> Option<&str> {
        Some("#95a5a6")
    }

    fn priority(&self) -> i32 {
        999
    }
}

/// Default category enum for basic operation categorization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DefaultCategory {
    /// Input/Output operations (database, network, file system)
    IO,
    /// Computational operations (calculations, data processing)
    Compute,
    /// Memory operations (allocation, deallocation)
    Memory,
    /// System operations (OS calls, threading)
    System,
    /// User interface operations
    UI,
    /// General uncategorized operations
    General,
}

impl Category for DefaultCategory {
    fn get_name(&self) -> &str {
        match self {
            DefaultCategory::IO => "I/O",
            DefaultCategory::Compute => "Compute",
            DefaultCategory::Memory => "Memory",
            DefaultCategory::System => "System",
            DefaultCategory::UI => "UI",
            DefaultCategory::General => "General",
        }
    }

    fn get_description(&self) -> &str {
        match self {
            DefaultCategory::IO => "Input/Output operations (database, network, file system)",
            DefaultCategory::Compute => "Computational operations (calculations, data processing)",
            DefaultCategory::Memory => "Memory operations (allocation, deallocation)",
            DefaultCategory::System => "System operations (OS calls, threading)",
            DefaultCategory::UI => "User interface operations",
            DefaultCategory::General => "General uncategorized operations",
        }
    }

    fn color_hint(&self) -> Option<&str> {
        Some(match self {
            DefaultCategory::IO => "#3498db",      // Blue
            DefaultCategory::Compute => "#e74c3c", // Red
            DefaultCategory::Memory => "#f39c12",  // Orange
            DefaultCategory::System => "#9b59b6",  // Purple
            DefaultCategory::UI => "#2ecc71",      // Green
            DefaultCategory::General => "#95a5a6", // Gray
        })
    }

    fn priority(&self) -> i32 {
        match self {
            DefaultCategory::IO => 1,
            DefaultCategory::Compute => 2,
            DefaultCategory::Memory => 3,
            DefaultCategory::System => 4,
            DefaultCategory::UI => 5,
            DefaultCategory::General => 999,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_category() {
        let no_cat = NoCategory;
        assert_eq!(no_cat.get_name(), "");
        assert_eq!(
            no_cat.get_description(),
            "Default category when none is specified"
        );
    }

    #[test]
    fn test_default_category() {
        assert_eq!(DefaultCategory::IO.get_name(), "I/O");
        assert_eq!(DefaultCategory::Compute.get_name(), "Compute");
        assert_eq!(DefaultCategory::Memory.get_name(), "Memory");
        assert_eq!(DefaultCategory::System.get_name(), "System");
        assert_eq!(DefaultCategory::UI.get_name(), "UI");
        assert_eq!(DefaultCategory::General.get_name(), "General");

        assert!(DefaultCategory::IO.color_hint().is_some());
        assert_eq!(DefaultCategory::IO.priority(), 1);
        assert_eq!(DefaultCategory::General.priority(), 999);
    }

    #[test]
    fn test_custom_category_implementation() {
        #[derive(Debug)]
        struct AppCategory;

        impl Category for AppCategory {
            fn get_name(&self) -> &str {
                "Application"
            }

            fn get_description(&self) -> &str {
                "Application-specific operations"
            }

            fn color_hint(&self) -> Option<&str> {
                Some("#9b59b6")
            }

            fn priority(&self) -> i32 {
                2
            }
        }

        let app = AppCategory;
        assert_eq!(app.get_name(), "Application");
        assert_eq!(app.get_description(), "Application-specific operations");
    }
}
