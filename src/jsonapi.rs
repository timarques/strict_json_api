use super::documents::markers::JsonApi as JsonApiMarker;
use super::link::markers::Link;
use super::present::Present;
use core::fmt::Debug;
use serde::{Deserialize, Serialize};

use core::str::FromStr;

super::macros::generate_object! {
    #[markers(JsonApiMarker)]
    #[unsafe_markers(Present)]
    JsonApi {
        VERSION: FromStr + Debug: version: Option<VERSION>;
        #[rename(ext)]
        EXTENSION: Link: extensions: Vec<EXTENSION>;
        #[rename(profile)]
        PROFILE: Link: profiles: Vec<PROFILE>;
        #[rename(meta)]
        METADATA: Debug: metadata, meta: Option<METADATA>;
    }
}
