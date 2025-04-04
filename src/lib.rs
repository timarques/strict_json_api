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
//!    * **`present::Present`:** Unsafe trait for guaranteed presence, enabling optimizations.
//!    * Use `Option<T>` for uncertain presence, `NotPresent` for known-absent fields,
//!      and `Present` only when guaranteed presence is needed for optimization.
//!
//! ## Module Overview
//!
//! * **`documents`:** Top-level JSON:API document structures and related markers.
//! * **`error`:** Structures for JSON:API error objects and source information.
//! * **`jsonapi`:** Structure for the top-level `jsonapi` member.
//! * **`link`:** Structures for link objects and markers.
//! * **`document_links`:** Common link structures for document's top level.
//! * **`present`:** Contains the `Present` unsafe marker trait and the `NotPresent` type.
//! * **`relationship`:** Structures for relationship objects and linkage.
//! * **`resources`:** Structures for resource objects, resource identifier objects, and markers.
//!
//! ## Usage Example
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//! use strict_json_api::{
//!     documents::DataDocument,
//!     present::NotPresent,
//!     resources::ResponseResource,
//! };
//! use core::str::FromStr;
//! use core::fmt::Debug;
//!
//! // Define your specific attribute structure
//! #[derive(Serialize, Deserialize, Debug, Clone)]
//! pub struct ArticleAttributes {
//!     pub title: String,
//!     pub word_count: u32,
//!     pub author_email: Option<String>, // Optional attribute
//! }
//!
//! // --- Define Concrete Types ---
//!
//! // Using String for both Type and ID (implements required traits)
//! type ArticleType = String;
//! type ArticleId = String;
//!
//! // Define the concrete resource type
//! type ArticleResource = ResponseResource<
//!     ArticleType,               // TYPE
//!     ArticleId,                 // ID
//!     Option<ArticleAttributes>, // ATTRIBUTES: Optional
//!     NotPresent,                // RELATIONSHIPS
//!     NotPresent,                // LINKS
//!     NotPresent,                // METADATA
//! >;
//!
//! // Define the top-level document
//! type ArticleDocument = DataDocument<
//!     ArticleResource, // DATA
//!     NotPresent,      // INCLUDED
//!     NotPresent,      // JSONAPI
//!     NotPresent,      // LINKS
//!     NotPresent,      // METADATA
//! >;
//!
//! // --- Deserialization ---
//! let json_string = r#"{
//!   "data": {
//!     "type": "articles",
//!     "id": "123",
//!     "attributes": {
//!       "title": "JSON:API Explained",
//!       "word_count": 1500
//!     }
//!   }
//! }"#;
//!
//! let doc: ArticleDocument = serde_json::from_str(json_string).expect("Failed to deserialize");
//!
//! // --- Accessing Data ---
//! let resource = doc.data();
//!
//! // Access fields marked Present
//! let resource_type = resource.r#type(); // or resource.kind()
//! let resource_id = resource.id();
//! println!("Type: {}, ID: {}", resource_type, resource_id);
//!
//! // Access optional fields safely
//! if let Some(attributes) = resource.attributes() {
//!     println!("Title: {}", attributes.title);
//!     println!("Word Count: {}", attributes.word_count);
//!     if let Some(email) = &attributes.author_email {
//!         println!("Author Email: {}", email);
//!     } else {
//!         println!("Author Email: Not provided");
//!     }
//! } else {
//!     println!("No attributes present.");
//! }
//! ```
//!

pub mod document_links;
pub mod documents;
pub mod error;
pub mod jsonapi;
pub mod link;
pub mod present;
pub mod relationship;
pub mod resources;

#[doc(hidden)]
mod macros;
