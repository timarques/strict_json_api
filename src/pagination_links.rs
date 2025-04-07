use super::link::IsLink;
use super::present::{NotPresent, Present};
use core::fmt::Debug;

super::macros::generate_markers! {
    IsPaginationLinks: Debug: Option<T>, NotPresent;
}

super::macros::generate_object! {
    #[mark(IsPaginationLinks)]
    #[unsafe_mark(Present)]
    PaginationLinks {
        first: Option<FIRST>: IsLink;
        last: Option<LAST>: IsLink;
        next: Option<NEXT>: IsLink;
        #[rename(prev)]
        previous: Option<PREVIOUS>: IsLink;
    }
}
