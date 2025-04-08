//! # Strict JSON:API (`strict_json_api`)
//!
//! This crate provides strongly-typed Rust definitions for representing
//! [JSON:API v1.0/v1.1](https://jsonapi.org/format/) structures. It leverages Rust's
//! type system and `serde` for compile-time validation of structure, increased clarity,
//! and efficient serialization/deserialization.
//!
//! ## Core Philosophy: Explicit & Strict Types
//!
//! Instead of relying on generic maps (like `serde_json::Value` or `HashMap`) which
//! require runtime checks, this crate mandates defining explicit Rust types for
//! every part of your JSON:API documents, resources, errors, links, etc.
//!
//! **Benefits:**
//!
//! * **Compile-Time Safety:** Catch JSON structural errors during compilation.
//! * **Clarity:** Explicit types make the expected data structure obvious.
//! * **Reduced Runtime Checks:** Minimize validation logic in your application code.
//! * **Potential Performance Gains:** Direct field access and targeted optimizations.
//! * **`serde` Integration:** Seamlessly integrates with `serde` for JSON handling.
//!
//! ## Key Concepts & Mechanisms
//!
//! 1. **Generic Structures:** Core JSON:API constructs are defined as generic Rust structs.
//!    You provide specific types for placeholders like `ID`, `TYPE`, `ATTRIBUTES`,
//!    `RELATIONSHIPS`, `META`, `LINKS`. These structs typically derive `Debug`, `Serialize`,
//!    `Deserialize`, and `Clone` if their underlying type parameters also support these traits.
//!
//! 2. **Marker Traits:** Various traits (often `unsafe`) are used as markers or
//!    constraints on generic parameters. They categorize types or signal that a type
//!    satisfies the structural rules for a specific JSON:API role. The `unsafe` nature
//!    signifies that the implementor is responsible for upholding the implied guarantees.
//!
//! 3. **`FromStr` Constraint:** JSON:API requires certain members be strings in JSON.
//!    This crate uses `T: FromStr + Debug` trait bounds to allow both `String` and custom
//!    types that implement these traits. This enhances type safety while ensuring proper
//!    JSON representation. The constraint is compile-time only; `serde` handles the actual
//!    deserialization.
//!
//! 4. **Handling Presence:** JSON:API defines many optional fields. This crate provides:
//!    * **`Option<T>`:** Standard way to represent maybe-present fields. **Recommended default**.
//!    * **`present::NotPresent`:** Marker for guaranteed absence, allowing optimization.
//!    * **`present::IsPresent`:** Unsafe trait for guaranteed presence, enabling optimizations.
//!    * Use `Option<T>` for uncertain presence, `NotPresent` for known-absent fields,
//!      and `IsPresent` only when guaranteed presence is needed for optimization.

#![doc = include_str!(concat!(env!("OUT_DIR"), "/examples.md"))]

pub mod document;
pub mod error;
pub mod json_api;
pub mod link;
pub mod pagination_links;
pub mod present;
pub mod relationship;
pub mod resource;
pub mod resource_identifier;

mod macros;
