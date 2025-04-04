#[doc(hidden)]
mod linkage_resource;
#[doc(hidden)]
mod request_resource;
#[doc(hidden)]
mod response_resource;

#[doc(inline)]
pub use linkage_resource::{LinkageResource, LinkageResourceCollection};
#[doc(inline)]
pub use request_resource::{RequestResource, RequestResourceCollection};
#[doc(inline)]
pub use response_resource::{ResponseResource, ResponseResourceCollection};
