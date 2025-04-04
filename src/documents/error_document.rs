use super::super::present::Present;
use super::markers::{Errors, JsonApi, Links};

use core::fmt::Debug;

super::super::macros::generate_object! {
    ErrorDocument {
        ERRORS: Errors: errors: Option<ERRORS>;
        JSONAPI: JsonApi: jsonapi: Option<JSONAPI>;
        LINKS: Links: links: Option<LINKS>;
        #[rename(meta)]
        METADATA: Debug: metadata, meta: Option<METADATA>;
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
        self.errors().fmt(f)
    }
}
