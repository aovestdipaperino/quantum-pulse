//! # Category Module
//!
//! Provides traits and types for categorizing profiling operations.
//! Users can define their own categories by implementing the `Category` trait.

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
        "NoCategory"
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_category() {
        let no_cat = NoCategory;
        assert_eq!(no_cat.get_name(), "NoCategory");
        assert_eq!(
            no_cat.get_description(),
            "Default category when none is specified"
        );
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
