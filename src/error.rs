use super::link::Link;
use super::present::{NotPresent, Present};
use core::fmt::Debug;
use core::str::FromStr;

super::macros::generate_markers! {
    Source: Debug: Option<T>, NotPresent;
    Errors: Debug: Option<T>, NotPresent, Vec<T>;
}

super::macros::generate_object! {
    #[unsafe_mark(Present)]
    Links {
        #[rename(self)]
        current, this: Option<CURRENT>: Link;
        about: Option<ABOUT>: Link;
    }
}

super::macros::generate_object! {
    #[mark(Source)]
    #[unsafe_mark(Present)]
    SourceObject {
        pointer: Option<POINTER>: FromStr + Debug;
        parameter: Option<PARAMETER>: FromStr + Debug;
        header: Option<HEADER>: FromStr + Debug;
    }
}

super::macros::generate_object! {
    #[unsafe_mark(Present)]
    Error {
        id: Option<ID>: FromStr + Debug;
        code: Option<CODE>: FromStr + Debug;
        status: Option<STATUS>: FromStr + Debug;
        detail: Option<DETAIL>: FromStr + Debug;
        title: Option<TITLE>: FromStr + Debug;
        source: Option<SOURCE>: Source;
        links: Option<LINKS>: Debug;
        #[rename(meta)]
        metadata, meta: Option<METADATA>: Debug;
    }
}

super::macros::generate_wrapper_object! {
    #[mark(Errors)]
    #[unsafe_mark(Present)]
    #[wrap]
    ErrorCollection:
    Vec<
        Error<
            ID,
            CODE,
            STATUS,
            DETAIL,
            TITLE,
            SOURCE,
            LINKS,
            METADATA
        >
    >
    {
        ID: FromStr + Debug;
        CODE: FromStr + Debug;
        STATUS: FromStr + Debug;
        DETAIL: FromStr + Debug;
        TITLE: FromStr + Debug;
        SOURCE: Source;
        LINKS: Debug;
        METADATA: Debug;
    }
}
