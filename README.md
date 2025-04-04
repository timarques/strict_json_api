# strict_json_api

> Strongly-typed Rust definitions for JSON:API v1.0/v1.1 structures

- [Crate on Crates.io](https://crates.io/crates/strict-json-api)
- [API Documentation](https://docs.rs/strict-json-api)
- [License: MIT/Apache-2.0](./LICENSE)

## Overview

`strict_json_api` leverages Rust's type system to provide compile-time validation of [JSON:API](https://jsonapi.org/format/) structures. Instead of using generic maps that require runtime checks, this crate enforces explicit types for every part of your JSON:API documents.

## Benefits

- **Compile-Time Safety**: Catch structure errors during compilation
- **Type Clarity**: Explicit types make data structures obvious
- **Performance**: Direct field access and minimized runtime checks
- **`serde` Integration**: Seamless JSON serialization/deserialization

## Key Concepts

1. **Generic Structures**: Core JSON:API constructs use generic Rust structs where you provide specific types
2. **Marker Traits**: Traits ensure types satisfy JSON:API structural requirements
3. **String Representation**: Uses `FromStr` constraint to allow both `String` and custom types
4. **Presence Handling**: Provides `Option<T>`, `NotPresent`, and `Present` mechanisms for optional fields

## Installation

```toml
[dependencies]
strict_json_api = "0.1.0"
```

## Usage Example

```rust
use serde::{Deserialize, Serialize};
use strict_json_api::{
    documents::DataDocument,
    present::NotPresent,
    resources::ResponseResource,
};
use core::str::FromStr;
use core::fmt::Debug;

// Define your specific attribute structure
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArticleAttributes {
    pub title: String,
    pub word_count: u32,
    pub author_email: Option<String>,
}

// Define concrete types
type ArticleType = String;
type ArticleId = String;

// Define the resource type
type ArticleResource = ResponseResource<
    ArticleType,               // TYPE
    ArticleId,                 // ID
    Option<ArticleAttributes>, // ATTRIBUTES
    NotPresent,                // RELATIONSHIPS
    NotPresent,                // LINKS
    NotPresent,                // METADATA
>;

// Define the document
type ArticleDocument = DataDocument<
    ArticleResource, // DATA
    NotPresent,      // INCLUDED
    NotPresent,      // JSONAPI
    NotPresent,      // LINKS
    NotPresent,      // METADATA
>;

// Deserialize JSON:API document
let json_string = r#"{
  "data": {
    "type": "articles",
    "id": "123",
    "attributes": {
      "title": "JSON:API Explained",
      "word_count": 1500
    }
  }
}"#;

let doc: ArticleDocument = serde_json::from_str(json_string).expect("Failed to deserialize");

// Access data
let resource = doc.data();
let resource_type = resource.r#type();
let resource_id = resource.id();

// Access optional fields safely
if let Some(attributes) = resource.attributes() {
    println!("Title: {}", attributes.title);
    println!("Word Count: {}", attributes.word_count);
}
```