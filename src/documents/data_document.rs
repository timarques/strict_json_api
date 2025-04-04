use super::markers::{Data, Included, JsonApi, Links};

use core::fmt::Debug;

super::super::macros::generate_object! {
    DataDocument {
        DATA: Data: data: Option<DATA>;
        INCLUDED: Included: included: Option<INCLUDED>;
        JSONAPI: JsonApi: jsonapi: Option<JSONAPI>;
        LINKS: Links: links: Option<LINKS>;
        #[rename(meta)]
        METADATA: Debug: metadata, meta: Option<METADATA>;
    }
}
