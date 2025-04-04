pub mod markers {
    use super::super::present::NotPresent;

    use core::fmt::Debug;

    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait Data: Debug {}

    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait Links: Debug {}

    unsafe impl Links for NotPresent {}
    unsafe impl<L> Links for Option<L> where L: Links {}
}

use super::link::markers::Link;
use super::present::Present;
use core::fmt::Debug;

super::macros::generate_object! {
    #[unsafe_markers(markers::Links, Present)]
    Links {
        #[rename(self)]
        CURRENT: Link: current, this: Option<CURRENT>;
        RELATED: Link: related: Option<RELATED>;
    }
}

super::macros::generate_object! {
    #[unsafe_markers(Present)]
    Relationship {
        DATA: markers::Data + Present: data: DATA;
        LINKS: markers::Links: links: Option<LINKS>;
        #[rename(meta)]
        METADATA: Debug: metadata, meta: Option<METADATA>;
}
}
