use super::link::Link;
use super::present::{NotPresent, Present};
use core::fmt::Debug;

super::macros::generate_markers! {
    Pagination: Debug: Option<T>, NotPresent;
}

super::macros::generate_object! {
    #[mark(Pagination)]
    #[unsafe_mark(Present)]
    PaginationObject {
        first: Option<FIRST>: Link;
        last: Option<LAST>: Link;
        next: Option<NEXT>: Link;
        prev: Option<PREV>: Link;
    }
}
