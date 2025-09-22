//! # Operation Module
//!
//! Trait for defining categorizable profiling operations.

use crate::category::{Category, NoCategory};
use std::fmt::Debug;

/// Trait for defining profiling operations
///
/// Implement this trait for types that represent operations you want to profile.
/// The trait requires Debug and provides default implementations for categorization
/// and string conversion.
///
/// # Example
/// ```rust
/// use quantum_pulse::{Operation, Category};
///
/// #[derive(Debug)]
/// struct DatabaseQuery {
///     table: String,
/// }
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
///         "Database operations"
///     }
/// }
///
/// impl Operation for DatabaseQuery {
///     fn get_category(&self) -> &dyn Category {
///         &DatabaseCategory
///     }
/// }
/// ```
pub trait Operation: Debug + Send + Sync {
    /// Get the category for this operation
    ///
    /// By default, returns `NoCategory`. Override this method to provide
    /// a specific category for your operation.
    fn get_category(&self) -> &dyn Category {
        &NoCategory
    }

    /// Convert the operation to a string representation
    ///
    /// By default, uses the Debug formatting. Override this method to provide
    /// a custom string representation.
    fn to_str(&self) -> String {
        format!("{:?}", self)
    }
}

/// A simple operation implementation for basic profiling
///
/// This is a convenience type for when you don't need custom categories
/// or complex operation types.

/// A categorized operation implementation
///
/// This is a convenience type for when you want to specify both
/// an operation name and a category without defining custom types.
pub struct CategorizedOperation {
    pub name: String,
    pub category: Box<dyn Category>,
}

impl CategorizedOperation {
    /// Create a new categorized operation
    pub fn new(name: impl Into<String>, category: Box<dyn Category>) -> Self {
        Self {
            name: name.into(),
            category,
        }
    }
}

impl Debug for CategorizedOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CategorizedOperation({})", self.name)
    }
}

impl Operation for CategorizedOperation {
    fn get_category(&self) -> &dyn Category {
        self.category.as_ref()
    }

    fn to_str(&self) -> String {
        self.name.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestOperation {
        name: String,
    }

    #[derive(Debug)]
    struct TestCategory;

    impl Category for TestCategory {
        fn get_name(&self) -> &str {
            "Test"
        }

        fn get_description(&self) -> &str {
            "Test category"
        }
    }

    impl Operation for TestOperation {
        fn get_category(&self) -> &dyn Category {
            &TestCategory
        }

        fn to_str(&self) -> String {
            format!("test_{}", self.name)
        }
    }

    #[test]
    fn test_operation_default_category() {
        #[derive(Debug)]
        struct DefaultOp;
        impl Operation for DefaultOp {}

        let op = DefaultOp;
        let category = op.get_category();
        assert_eq!(category.get_name(), "");
    }

    #[test]
    fn test_operation_default_to_str() {
        #[derive(Debug)]
        struct DefaultOp;
        impl Operation for DefaultOp {}

        let op = DefaultOp;
        assert_eq!(op.to_str(), "DefaultOp");
    }

    #[test]
    fn test_custom_operation() {
        let op = TestOperation {
            name: "my_test".to_string(),
        };

        assert_eq!(op.get_category().get_name(), "Test");
        assert_eq!(op.to_str(), "test_my_test");
    }

    #[test]
    fn test_categorized_operation_creation() {
        let op = CategorizedOperation::new("simple_test", Box::new(NoCategory));
        assert_eq!(op.to_str(), "simple_test");
        assert_eq!(op.get_category().get_name(), "");
    }

    #[test]
    fn test_categorized_operation() {
        let op = CategorizedOperation::new("categorized_test", Box::new(TestCategory));
        assert_eq!(op.to_str(), "categorized_test");
        assert_eq!(op.get_category().get_name(), "Test");
        assert_eq!(
            format!("{:?}", op),
            "CategorizedOperation(categorized_test)"
        );
    }
}
