use super::super::present::NotPresent;
use super::document::Document;
use super::markers::{Data, Included, JsonApi, Links};

use core::fmt::Debug;

super::super::macros::generate_wrapper_object! {
    DataDocument: Document<DATA, NotPresent, INCLUDED, JSONAPI, LINKS, METADATA> {
        DATA: Data;
        INCLUDED: Included;
        JSONAPI: JsonApi;
        LINKS: Links;
        METADATA: Debug;
    }
}
