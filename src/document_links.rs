use super::document::markers;
use super::link::markers::Link;
use super::present::Present;
use core::fmt::Debug;
use serde::{Deserialize, Serialize};

super::macros::generate_object! {
    #[markers(markers::Links)]
    #[unsafe_markers(Present)]
    DocumentLinks {
        FIRST: Link: first: Option<FIRST>;
        LAST: Link: last: Option<LAST>;
        NEXT: Link: next: Option<NEXT>;
        PREV: Link: prev: Option<PREV>;
        #[rename(self)]
        CURRENT: Link: current, this: Option<CURRENT>;
        RELATED: Link: related: Option<RELATED>;
        #[rename(describedby)]
        DESCRIBEDBY: Link: described_by: Option<DESCRIBEDBY>;
    }
}
