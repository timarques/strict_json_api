use super::link::Link;
use super::present::{NotPresent, Present};
use core::fmt::Debug;

use core::str::FromStr;

super::macros::generate_markers! {
    JsonApi: Debug: Option<T>, NotPresent;
}

super::macros::generate_object! {
    #[mark(JsonApi)]
    #[unsafe_mark(Present)]
    JsonApiObject {
        version: Option<VERSION>: FromStr + Debug;
        #[rename(ext)]
        extensions: Vec<EXTENSION>: Link;
        #[rename(profile)]
        profiles: Vec<PROFILE>: Link;
        #[rename(meta)]
        metadata, meta: Option<METADATA>: Debug;
    }
}
