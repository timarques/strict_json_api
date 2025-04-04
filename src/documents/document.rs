use super::markers::{Data, Errors, Included, JsonApi, Links};

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

super::super::macros::generate_object! {
    Document {
        DATA: Data: data: Option<DATA>;
        ERRORS: Errors: errors: Option<ERRORS>;
        INCLUDED: Included: included: Option<INCLUDED>;
        JSONAPI: JsonApi: jsonapi: Option<JSONAPI>;
        LINKS: Links: links: Option<LINKS>;
        #[rename(meta)]
        METADATA: Debug: metadata, meta: Option<METADATA>;
    }
}
