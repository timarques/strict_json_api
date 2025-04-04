use super::super::present::{NotPresent, Present};
use super::document::Document;
use super::markers::{Errors, JsonApi, Links};

use core::fmt::Debug;

super::super::macros::generate_wrapper_object! {
    ErrorDocument: Document<NotPresent, ERRORS, NotPresent, JSONAPI, LINKS, METADATA> {
        ERRORS: Errors + Present;
        JSONAPI: JsonApi;
        LINKS: Links;
        METADATA: Debug;
    }
}

impl<ERRORS, JSONAPI, LINKS, METADATA> core::error::Error
    for ErrorDocument<ERRORS, JSONAPI, LINKS, METADATA>
where
    ERRORS: Errors + Present,
    JSONAPI: JsonApi,
    LINKS: Links,
    METADATA: Debug,
{
}

impl<ERRORS, JSONAPI, LINKS, METADATA> core::fmt::Display
    for ErrorDocument<ERRORS, JSONAPI, LINKS, METADATA>
where
    ERRORS: Errors + Present,
    JSONAPI: JsonApi,
    LINKS: Links,
    METADATA: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.inner.errors().fmt(f)
    }
}
