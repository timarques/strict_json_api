#[doc(hidden)]
mod data_document;
#[doc(hidden)]
mod document;
#[doc(hidden)]
mod error_document;

pub mod markers;

#[doc(inline)]
pub use data_document::DataDocument;
#[doc(inline)]
pub use document::Document;
#[doc(inline)]
pub use error_document::ErrorDocument;
