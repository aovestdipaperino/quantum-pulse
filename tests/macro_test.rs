//! Integration tests for the Operation derive macro.
//!
//! These tests verify that the macro correctly generates category implementations
//! and handles various edge cases.

#![cfg(feature = "macros")]

use quantum_pulse::{Operation, ProfileOp};

#[test]
fn test_basic_derive() {
    #[derive(Debug, ProfileOp)]
    enum TestOp {
        #[category(name = "IO", description = "Input/Output operations")]
        ReadFile,

        #[category(name = "Network")]
        HttpRequest,

        ComputeHash,
    }

    let read_op = TestOp::ReadFile;
    assert_eq!(read_op.get_category().get_name(), "IO");
    assert_eq!(
        read_op.get_category().get_description(),
        "Input/Output operations"
    );

    let http_op = TestOp::HttpRequest;
    assert_eq!(http_op.get_category().get_name(), "Network");
    assert_eq!(http_op.get_category().get_description(), "Network");

    let compute_op = TestOp::ComputeHash;
    assert_eq!(compute_op.get_category().get_name(), "ComputeHash");
    assert_eq!(compute_op.get_category().get_description(), "ComputeHash");
}

#[test]
fn test_description_uniqueness() {
    // Test that only the first description for a category is used
    #[derive(Debug, ProfileOp)]
    enum TestOp {
        #[category(name = "IO", description = "First description")]
        Operation1,

        #[category(name = "IO", description = "Second description - should be ignored")]
        Operation2,

        #[category(name = "IO")]
        Operation3,
    }

    let op1 = TestOp::Operation1;
    let op2 = TestOp::Operation2;
    let op3 = TestOp::Operation3;

    // All should have the same category name
    assert_eq!(op1.get_category().get_name(), "IO");
    assert_eq!(op2.get_category().get_name(), "IO");
    assert_eq!(op3.get_category().get_name(), "IO");

    // All should have the first description
    assert_eq!(op1.get_category().get_description(), "First description");
    assert_eq!(op2.get_category().get_description(), "First description");
    assert_eq!(op3.get_category().get_description(), "First description");
}

#[test]
fn test_description_precedence() {
    // Test that a description provided later doesn't override if the first had none
    #[derive(Debug, ProfileOp)]
    enum TestOp {
        #[category(name = "Network")]
        Operation1,

        #[category(name = "Network", description = "Network operations")]
        Operation2,
    }

    let op1 = TestOp::Operation1;
    let op2 = TestOp::Operation2;

    // Both should have the same category name
    assert_eq!(op1.get_category().get_name(), "Network");
    assert_eq!(op2.get_category().get_name(), "Network");

    // Since the first variant didn't have a description, it defaulted to the name
    // The second description should update it
    assert_eq!(op1.get_category().get_description(), "Network operations");
    assert_eq!(op2.get_category().get_description(), "Network operations");
}

#[test]
fn test_enum_variants_with_data() {
    #[derive(Debug, ProfileOp)]
    enum TestOp {
        #[category(name = "Database", description = "Database operations")]
        Query(String),

        #[category(name = "Cache", description = "Cache operations")]
        CacheOp { key: String, ttl: u64 },

        #[category(name = "Compute")]
        Process(u32, u32),
    }

    let query_op = TestOp::Query("SELECT * FROM users".to_string());
    assert_eq!(query_op.get_category().get_name(), "Database");
    assert_eq!(
        query_op.get_category().get_description(),
        "Database operations"
    );

    let cache_op = TestOp::CacheOp {
        key: "user:123".to_string(),
        ttl: 3600,
    };
    assert_eq!(cache_op.get_category().get_name(), "Cache");
    assert_eq!(
        cache_op.get_category().get_description(),
        "Cache operations"
    );

    let process_op = TestOp::Process(10, 20);
    assert_eq!(process_op.get_category().get_name(), "Compute");
    assert_eq!(process_op.get_category().get_description(), "Compute");
}

#[test]
fn test_special_characters_in_category_names() {
    #[derive(Debug, ProfileOp)]
    enum TestOp {
        #[category(name = "IO/Network", description = "Combined I/O and Network")]
        Hybrid,

        #[category(name = "CPU-Intensive")]
        Heavy,

        #[category(name = "Memory Allocation")]
        Allocate,
    }

    let hybrid_op = TestOp::Hybrid;
    assert_eq!(hybrid_op.get_category().get_name(), "IO/Network");
    assert_eq!(
        hybrid_op.get_category().get_description(),
        "Combined I/O and Network"
    );

    let heavy_op = TestOp::Heavy;
    assert_eq!(heavy_op.get_category().get_name(), "CPU-Intensive");
    assert_eq!(heavy_op.get_category().get_description(), "CPU-Intensive");

    let alloc_op = TestOp::Allocate;
    assert_eq!(alloc_op.get_category().get_name(), "Memory Allocation");
    assert_eq!(
        alloc_op.get_category().get_description(),
        "Memory Allocation"
    );
}

#[test]
fn test_mixed_category_definitions() {
    #[derive(Debug, ProfileOp)]
    enum TestOp {
        // With description
        #[category(name = "IO", description = "Input/Output")]
        FileRead,

        // Same category, no description
        #[category(name = "IO")]
        FileWrite,

        // No category attribute at all
        Compute,

        // Different category
        #[category(name = "Network", description = "Network operations")]
        HttpRequest,

        // Back to IO category
        #[category(name = "IO")]
        FileDelete,
    }

    let read = TestOp::FileRead;
    let write = TestOp::FileWrite;
    let compute = TestOp::Compute;
    let http = TestOp::HttpRequest;
    let delete = TestOp::FileDelete;

    assert_eq!(read.get_category().get_name(), "IO");
    assert_eq!(read.get_category().get_description(), "Input/Output");

    assert_eq!(write.get_category().get_name(), "IO");
    assert_eq!(write.get_category().get_description(), "Input/Output");

    assert_eq!(compute.get_category().get_name(), "Compute");
    assert_eq!(compute.get_category().get_description(), "Compute");

    assert_eq!(http.get_category().get_name(), "Network");
    assert_eq!(http.get_category().get_description(), "Network operations");

    assert_eq!(delete.get_category().get_name(), "IO");
    assert_eq!(delete.get_category().get_description(), "Input/Output");
}

#[test]
fn test_operation_trait_implementation() {
    use quantum_pulse::profile;
    use quantum_pulse::ProfileCollector;

    #[derive(Debug, ProfileOp)]
    enum TestOp {
        #[category(name = "Test", description = "Test operations")]
        TestOperation,
    }

    ProfileCollector::clear_all();

    let op = TestOp::TestOperation;

    // Verify it implements Operation trait
    let category = op.get_category();
    assert_eq!(category.get_name(), "Test");

    // Verify it can be used with the profile macro
    let result = profile!(op, { 42 });

    assert_eq!(result, 42);
}

#[test]
fn test_multiple_enums_same_category_names() {
    // Test that multiple enums can use the same category names without conflict
    #[derive(Debug, ProfileOp)]
    enum AppOp1 {
        #[category(name = "IO", description = "App1 IO")]
        Read,
    }

    #[derive(Debug, ProfileOp)]
    enum AppOp2 {
        #[category(name = "IO", description = "App2 IO")]
        Write,
    }

    let op1 = AppOp1::Read;
    let op2 = AppOp2::Write;

    // Each enum should have its own category implementation
    assert_eq!(op1.get_category().get_name(), "IO");
    assert_eq!(op1.get_category().get_description(), "App1 IO");

    assert_eq!(op2.get_category().get_name(), "IO");
    assert_eq!(op2.get_category().get_description(), "App2 IO");
}

#[test]
fn test_empty_enum() {
    // Edge case: empty enum (though not very useful in practice)
    #[derive(Debug, ProfileOp)]
    enum EmptyOp {}

    // This should compile successfully even though it can never be instantiated
    // The match in get_category() will have no arms but that's valid for an empty enum
}

#[test]
fn test_single_variant_enum() {
    quantum_pulse::ProfileCollector::clear_all();
    #[derive(Debug, ProfileOp)]
    enum SingleOp {
        #[category(name = "Single", description = "Single operation")]
        OnlyOne,
    }

    let op = SingleOp::OnlyOne;
    assert_eq!(op.get_category().get_name(), "Single");
    assert_eq!(op.get_category().get_description(), "Single operation");
}
