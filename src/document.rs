use super::error::IsErrorCollection;
use super::json_api::IsJsonApi;
use super::link::IsLink;
use super::pagination_links::IsPaginationLinks;
use super::present::{NotPresent, Present};
use super::resource::{IsResource, IsResourceResponse, IsResourceResponseCollection};

use core::fmt::Debug;

super::macros::generate_markers! {
    IsDocumentLinks: Debug {
        #[wrap]
        Option;
        NotPresent;
    }
    IsDocumentPrimaryData: Debug {
        #[dyn]
        IsResource;
    }
    IsDocumentPrimaryDataResponse: Debug {
        #[dyn]
        IsResourceResponse;
    }
    IsDocumentIncluded: Debug {
        #[dyn]
        IsResourceResponseCollection;
    }
}

super::macros::generate_object! {
    #[mark(IsDocumentLinks)]
    #[unsafe_mark(Present)]
    DocumentLinks {
        #[flatten]
        pagination: Option<PAGINATION>: IsPaginationLinks;
        current, this: Option<CURRENT>: IsLink;
        related: Option<RELATED>: IsLink;
        #[rename(describedby)]
        described_by: Option<DESCRIBEDBY>: IsLink;
    }
}

super::macros::generate_object! {
    Document {
        data: Option<DATA>: IsDocumentPrimaryData;
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
        data: DATA: IsDocumentPrimaryData + Present;
        json_api: Option<JSONAPI>: IsJsonApi;
        links: Option<LINKS>: IsDocumentLinks;
        #[rename(meta)]
        metadata, meta: Option<METADATA>: Debug;
    }
}

super::macros::generate_object! {
    DocumentSuccessResponse {
        data: DATA: IsDocumentPrimaryDataResponse + Present;
        included: Option<INCLUDED>: IsDocumentIncluded;
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
