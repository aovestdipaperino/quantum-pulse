//! Procedural macros for the quantum-pulse profiling library.
//!
//! This crate provides derive macros to automatically implement profiling traits,
//! reducing boilerplate code and making it easier to integrate profiling into your applications.

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashMap;
use syn::{parse_macro_input, Data, DeriveInput};

/// Derives the `Operation` trait for enums, automatically generating category implementations.
///
/// This macro generates unique category structs for each distinct category name found in the
/// enum variants, and implements the `Operation` trait to return the appropriate category
/// for each variant.
///
/// # Attributes
///
/// The macro supports the `#[category(...)]` attribute on enum variants with the following parameters:
/// - `name`: The name of the category (optional, defaults to variant name)
/// - `description`: A description of the category (optional, defaults to category name)
///
/// # Important Behavior
///
/// When multiple variants use the same category name:
/// - Only one category struct is generated per unique category name
/// - The first `description` encountered for a category name is used
/// - Subsequent descriptions for the same category name are ignored
///
/// # Example
///
/// ```rust,ignore
/// use quantum_pulse::{ProfileOp, Operation};
///
/// #[derive(Debug, ProfileOp)]
/// enum MyOperation {
///     // Category with both name and description
///     #[category(name = "IO", description = "Input/Output operations")]
///     ReadFile,
///
///     // Same category, description is ignored (first one wins)
///     #[category(name = "IO", description = "This description is ignored")]
///     WriteFile,
///
///     // Category with only name (description defaults to name)
///     #[category(name = "Network")]
///     HttpRequest,
///
///     // No category attribute (uses variant name as category)
///     Compute,
///
///     // Supports enum variants with data
///     #[category(name = "Database")]
///     Query(String),
///
///     // Supports enum variants with named fields
///     #[category(name = "Cache")]
///     CacheOp { key: String, ttl: u64 },
/// }
/// ```
///
/// # Generated Code
///
/// For each unique category, the macro generates:
/// - A hidden struct implementing the `Category` trait
/// - An implementation of `Operation::get_category()` that returns the appropriate category
///
/// # Panics
///
/// - If applied to anything other than an enum
/// - If the category attribute parsing fails
#[proc_macro_derive(Operation, attributes(category))]
pub fn derive_operation(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    let data_enum = match &input.data {
        Data::Enum(data) => data,
        _ => panic!("Operation can only be derived for enums"),
    };

    // Track unique categories by name
    let mut categories: HashMap<String, CategoryInfo> = HashMap::new();
    let mut variant_categories: Vec<String> = Vec::new();

    // First pass: collect all categories and their info
    for variant in &data_enum.variants {
        let variant_ident = &variant.ident;
        let mut category_name = None;
        let mut category_description = None;

        // Parse the category attribute
        for attr in &variant.attrs {
            if attr.path().is_ident("category") {
                let nested = attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("name") {
                        let value = meta.value()?;
                        let s: syn::LitStr = value.parse()?;
                        category_name = Some(s.value());
                    } else if meta.path.is_ident("description") {
                        let value = meta.value()?;
                        let s: syn::LitStr = value.parse()?;
                        category_description = Some(s.value());
                    } else {
                        return Err(meta.error("unrecognized category attribute"));
                    }
                    Ok(())
                });

                if let Err(err) = nested {
                    panic!("Failed to parse category attribute: {}", err);
                }
            }
        }

        // Determine the category name (default to empty string if not specified)
        let final_category_name = category_name.unwrap_or_else(|| String::new());

        // Only update the category info if it hasn't been defined yet or if this one has a description
        if !categories.contains_key(&final_category_name) {
            categories.insert(
                final_category_name.clone(),
                CategoryInfo {
                    name: final_category_name.clone(),
                    description: category_description.unwrap_or_else(|| {
                        if final_category_name.is_empty() {
                            format!("{}", variant_ident)
                        } else {
                            final_category_name.clone()
                        }
                    }),
                },
            );
        } else if category_description.is_some() {
            // If this category already exists but this variant provides a description,
            // only update if the existing one doesn't have a custom description
            let existing = categories.get(&final_category_name).unwrap();
            if existing.description == final_category_name
                || (final_category_name.is_empty()
                    && existing.description == format!("{}", variant_ident))
            {
                categories.insert(
                    final_category_name.clone(),
                    CategoryInfo {
                        name: final_category_name.clone(),
                        description: category_description.unwrap(),
                    },
                );
            }
        }

        variant_categories.push(final_category_name);
    }

    // Generate category structs for unique categories
    let category_defs: Vec<_> = categories
        .values()
        .map(|cat_info| {
            let struct_name = format_ident!(
                "__Category_{}_{}",
                enum_name,
                sanitize_ident(&cat_info.name)
            );
            let cat_name = &cat_info.name;
            let cat_description = &cat_info.description;

            quote! {
                #[doc(hidden)]
                #[allow(non_camel_case_types)]
                #[derive(Debug)]
                struct #struct_name;

                impl quantum_pulse::Category for #struct_name {
                    fn get_name(&self) -> &str {
                        #cat_name
                    }

                    fn get_description(&self) -> &str {
                        #cat_description
                    }
                }
            }
        })
        .collect();

    // Generate match arms for the Operation implementation
    let match_arms: Vec<_> = data_enum
        .variants
        .iter()
        .zip(variant_categories.iter())
        .map(|(variant, category_name)| {
            let variant_ident = &variant.ident;
            let struct_name =
                format_ident!("__Category_{}_{}", enum_name, sanitize_ident(category_name));

            // Handle enum variants with fields
            let pattern = match &variant.fields {
                syn::Fields::Unit => quote! { #enum_name::#variant_ident },
                syn::Fields::Unnamed(_) => quote! { #enum_name::#variant_ident(..) },
                syn::Fields::Named(_) => quote! { #enum_name::#variant_ident{..} },
            };

            quote! {
                #pattern => &#struct_name as &dyn quantum_pulse::Category,
            }
        })
        .collect();

    // Handle empty enums specially
    let operation_impl = if data_enum.variants.is_empty() {
        quote! {
            impl quantum_pulse::Operation for #enum_name {
                fn get_category(&self) -> &dyn quantum_pulse::Category {
                    match *self {}
                }
            }
        }
    } else {
        quote! {
            impl quantum_pulse::Operation for #enum_name {
                fn get_category(&self) -> &dyn quantum_pulse::Category {
                    match self {
                        #(#match_arms)*
                    }
                }
            }
        }
    };

    let expanded = quote! {
        #(#category_defs)*

        #operation_impl
    };

    TokenStream::from(expanded)
}

/// Information about a category collected from enum variant attributes.
///
/// This struct holds the parsed category information that will be used
/// to generate the category implementation.
struct CategoryInfo {
    /// The name of the category as specified in the attribute or derived from variant name
    name: String,
    /// The description of the category, defaults to the name if not specified
    description: String,
}

/// Sanitizes a string to be a valid Rust identifier.
///
/// Replaces any non-alphanumeric characters (except underscore) with underscores
/// to ensure the resulting string can be used as part of a struct name.
///
/// # Arguments
///
/// * `s` - The string to sanitize
///
/// # Returns
///
/// A string safe to use as a Rust identifier component
///
/// # Example
///
/// ```ignore
/// assert_eq!(sanitize_ident("My-Category"), "My_Category");
/// assert_eq!(sanitize_ident("IO/Network"), "IO_Network");
/// ```
fn sanitize_ident(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}
