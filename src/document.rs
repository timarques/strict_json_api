use super::error::IsErrorCollection;
use super::json_api::IsJsonApi;
use super::link::IsLink;
use super::pagination_links::IsPaginationLinks;
use super::present::{NotPresent, Present};
use super::resource::{IsResource, IsResourceResponseCollection};

use core::fmt::Debug;

super::macros::generate_markers! {
    IsDocumentLinks: Debug: Option<T>, NotPresent;
}

super::macros::generate_object! {
    #[mark(IsDocumentLinks)]
    #[unsafe_mark(Present)]
    DocumentLinks {
        #[flatten]
        pagination_links, pagination: Option<PAGINATION>: IsPaginationLinks;
        current, this: Option<CURRENT>: IsLink;
        related: Option<RELATED>: IsLink;
        #[rename(describedby)]
        described_by: Option<DESCRIBEDBY>: IsLink;
    }
}

super::macros::generate_object! {
    Document {
        data: Option<DATA>: IsResource;
        included: Option<INCLUDED>: IsResourceResponseCollection;
        errors: Option<ERRORS>: IsErrorCollection;
        json_api: Option<JSONAPI>: IsJsonApi;
        links: Option<LINKS>: IsDocumentLinks;
        #[rename(meta)]
        metadata, meta: Option<METADATA>: Debug;
    }
}

super::macros::generate_object! {
    DocumentRequest {
        data: DATA: IsResource + Present;
        json_api: Option<JSONAPI>: IsJsonApi;
        links: Option<LINKS>: IsDocumentLinks;
        #[rename(meta)]
        metadata, meta: Option<METADATA>: Debug;
    }
}

super::macros::generate_object! {
    DocumentSuccessResponse {
        data: DATA: IsResource + Present;
        included: Option<INCLUDED>: IsResourceResponseCollection;
        json_api: Option<JSONAPI>: IsJsonApi;
        links: Option<LINKS>: IsDocumentLinks;
        #[rename(meta)]
        metadata, meta: Option<METADATA>: Debug;
    }
}

super::macros::generate_object! {
    DocumentErrorResponse {
        errors: ERRORS: IsErrorCollection + Present;
        json_api: Option<JSONAPI>: IsJsonApi;
        links: Option<LINKS>: IsDocumentLinks;
        #[rename(meta)]
        metadata, meta: Option<METADATA>: Debug;
    }
}

impl<ERRORS, JSONAPI, LINKS, METADATA> core::error::Error
    for DocumentErrorResponse<ERRORS, JSONAPI, LINKS, METADATA>
where
    ERRORS: IsErrorCollection + Present,
    JSONAPI: IsJsonApi,
    LINKS: IsDocumentLinks,
    METADATA: Debug,
{
}

impl<ERRORS, JSONAPI, LINKS, METADATA> core::fmt::Display
    for DocumentErrorResponse<ERRORS, JSONAPI, LINKS, METADATA>
where
    ERRORS: IsErrorCollection + Present,
    JSONAPI: IsJsonApi,
    LINKS: IsDocumentLinks,
    METADATA: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.errors().fmt(f)
    }
}
