pub mod markers {
    use super::super::present::NotPresent;
    use core::fmt::Debug;

    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait Source: Debug {}

    unsafe impl Source for NotPresent {}
    unsafe impl<T> Source for Option<T> where T: Source {}
}

use super::documents::markers::Errors;
use super::link::markers::Link;
use super::present::Present;
use core::fmt::Debug;
use core::str::FromStr;

super::macros::generate_object! {
    #[unsafe_markers(Present)]
    Links {
        #[rename(self)]
        CURRENT: Link: current, this: Option<CURRENT>;
        ABOUT: Link: about: Option<ABOUT>;
    }
}

super::macros::generate_object! {
    #[unsafe_markers(markers::Source, Present)]
    Source {
        POINTER: FromStr + Debug: pointer: Option<POINTER>;
        PARAMETER: FromStr + Debug: parameter: Option<PARAMETER>;
        HEADER: FromStr + Debug: header: Option<HEADER>;
    }
}

super::macros::generate_object! {
    #[unsafe_markers(Present)]
    Error {
        ID: FromStr + Debug: id: Option<ID>;
        CODE: FromStr + Debug: code: Option<CODE>;
        STATUS: FromStr + Debug: status: Option<STATUS>;
        DETAIL: FromStr + Debug: detail: Option<DETAIL>;
        TITLE: FromStr + Debug: title: Option<TITLE>;
        SOURCE: markers::Source: source: Option<SOURCE>;
        LINKS: Debug: links: Option<LINKS>;
        #[rename(meta)]
        METADATA: Debug: metadata, meta: Option<METADATA>;
    }
}

super::macros::generate_wrapper_object! {
    #[unsafe_markers(Errors, Present)]
    ErrorCollection: Vec<Error<ID, CODE, STATUS, DETAIL, TITLE, SOURCE, LINKS, METADATA>> {
        ID: FromStr + Debug;
        CODE: FromStr + Debug;
        STATUS: FromStr + Debug;
        DETAIL: FromStr + Debug;
        TITLE: FromStr + Debug;
        SOURCE: markers::Source;
        LINKS: Debug;
        METADATA: Debug;
    }
}
