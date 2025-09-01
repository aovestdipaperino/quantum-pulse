# quantum-pulse-macros

Procedural macros for the [quantum-pulse](https://github.com/aovestdipaperino/quantum-pulse) profiling library.

## Overview

This crate provides derive macros to automatically implement profiling traits from the quantum-pulse library, reducing boilerplate code and making it easier to integrate profiling into your applications.

## Features

- **`#[derive(Operation)]`** - Automatically implements the `Operation` trait for enums
- **Zero-cost abstractions** - Generated code is optimized away when profiling is disabled
- **Flexible categorization** - Define custom categories with descriptions
- **Type-safe** - All category definitions are checked at compile time

## Installation

This crate is typically used as a dependency of `quantum-pulse`. Add to your `Cargo.toml`:

```toml
[dependencies]
quantum-pulse = { version = "0.1", features = ["macros"] }
```

## Usage

### Basic Example

```rust
use quantum_pulse::{ProfileOp, Operation, profile};

#[derive(Debug, ProfileOp)]
enum MyOperation {
    // Category with both name and description
    #[category(name = "IO", description = "Input/Output operations")]
    ReadFile,
    
    // Same category, description is reused
    #[category(name = "IO")]
    WriteFile,
    
    // Category with only name (description defaults to name)
    #[category(name = "Network")]
    HttpRequest,
    
    // No category attribute (uses variant name as category)
    ComputeHash,
}

fn main() {
    let op = MyOperation::ReadFile;
    let result = profile!(op, {
        // Your code here
        "file content"
    });
}
```

### Category Attributes

The `#[category(...)]` attribute supports the following parameters:

- **`name`** - The name of the category (optional, defaults to variant name)
- **`description`** - A description of the category (optional, defaults to category name)

### Important Behavior

When multiple enum variants use the same category name:

1. **Only one category struct is generated** per unique category name
2. **The first description wins** - The first `description` encountered for a category name is used
3. **Subsequent descriptions are ignored** - If a category already has a description, later ones are ignored
4. **Description upgrade** - If the first variant with a category name has no description (defaulting to the name), and a later variant provides one, the description is updated

### Example with Shared Categories

```rust
#[derive(Debug, ProfileOp)]
enum DatabaseOps {
    #[category(name = "Query", description = "Database read operations")]
    SelectUsers,
    
    #[category(name = "Query")]  // Reuses "Database read operations" description
    SelectPosts,
    
    #[category(name = "Mutation", description = "Database write operations")]
    InsertUser,
    
    #[category(name = "Mutation")]  // Reuses "Database write operations" description
    UpdateUser,
}
```

### Enum Variants with Data

The macro supports all enum variant types:

```rust
#[derive(Debug, ProfileOp)]
enum ComplexOperation {
    // Unit variant
    #[category(name = "Simple")]
    Basic,
    
    // Tuple variant
    #[category(name = "Database")]
    Query(String),
    
    // Struct variant
    #[category(name = "Cache")]
    CacheOp { key: String, ttl: u64 },
}
```

### Special Characters in Category Names

Category names can contain special characters, which are automatically sanitized for internal use:

```rust
#[derive(Debug, ProfileOp)]
enum SpecialOps {
    #[category(name = "I/O Operations")]
    FileAccess,
    
    #[category(name = "CPU-Intensive")]
    HeavyCompute,
    
    #[category(name = "Memory & Cache")]
    MemoryOp,
}
```

## How It Works

The `#[derive(Operation)]` macro:

1. **Parses** the enum and its `#[category]` attributes
2. **Collects** unique categories and their descriptions
3. **Generates** a hidden struct for each unique category that implements the `Category` trait
4. **Implements** the `Operation` trait for the enum with a `get_category()` method that returns the appropriate category

Generated structs are prefixed with the enum name to avoid naming conflicts when multiple enums use the same category names.

## Performance

The generated code has minimal runtime overhead:

- Category structs are zero-sized types (ZSTs)
- The `get_category()` method is a simple match expression
- All generated code can be inlined by the compiler
- When profiling is disabled (default), everything compiles to nothing

## License

This crate is part of the quantum-pulse project and is dual-licensed under MIT OR Apache-2.0.