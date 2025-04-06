use super::error::Errors;
use super::jsonapi::JsonApi;
use super::link::Link;
use super::pagination::Pagination;
use super::present::{NotPresent, Present};
use super::resource::{IncludedResources, Resource};

use core::fmt::Debug;

super::macros::generate_markers! {
    Links: Debug: Option<T>, NotPresent;
}

super::macros::generate_object! {
    #[mark(Links)]
    #[unsafe_mark(Present)]
    DocumentLinksObject {
        #[flatten]
        pagination: Option<PAGINATION>: Pagination;
        current, this: Option<CURRENT>: Link;
        related: Option<RELATED>: Link;
        #[rename(describedby)]
        described_by: Option<DESCRIBEDBY>: Link;
    }
}

super::macros::generate_object! {
    DocumentObject {
        data: Option<DATA>: Resource;
        included: Option<INCLUDED>: IncludedResources;
        errors: Option<ERRORS>: Errors;
        json_api: Option<JSONAPI>: JsonApi;
        links: Option<LINKS>: Links;
        #[rename(meta)]
        metadata, meta: Option<METADATA>: Debug;
    }
}

super::macros::generate_wrapper_object! {
    #[wrap]
    DataDocumentObject: DocumentObject<DATA, INCLUDED, NotPresent, JSONAPI, LINKS, METADATA> {
        DATA: Resource + Present;
        INCLUDED: IncludedResources;
        JSONAPI: JsonApi;
        LINKS: Links;
        METADATA: Debug;
    }
}

super::macros::generate_wrapper_object! {
    #[wrap]
    ErrorDocumentObject: DocumentObject<NotPresent, NotPresent, ERRORS, JSONAPI, LINKS, METADATA> {
        ERRORS: Errors + Present;
        JSONAPI: JsonApi;
        LINKS: Links;
        METADATA: Debug;
    }
}

impl<ERRORS, JSONAPI, LINKS, METADATA> core::error::Error
    for ErrorDocumentObject<ERRORS, JSONAPI, LINKS, METADATA>
where
    ERRORS: Errors + Present,
    JSONAPI: JsonApi,
    LINKS: Links,
    METADATA: Debug,
{
}

impl<ERRORS, JSONAPI, LINKS, METADATA> core::fmt::Display
    for ErrorDocumentObject<ERRORS, JSONAPI, LINKS, METADATA>
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
