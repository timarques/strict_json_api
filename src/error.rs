use super::link::IsLink;
use super::present::{IsPresent, NotPresent};
use core::fmt::Debug;
use core::str::FromStr;

super::macros::generate_markers! {
    IsErrorSource: Debug {
        #[wrap]
        Option;
        NotPresent;
    }
    IsErrorCollection: Debug {
        NotPresent;
        #[wrap]
        Option;
        #[wrap]
        Vec;
    }
}

// spec refers this as may, therefore no trait
super::macros::generate_object! {
    #[unsafe_mark(IsPresent)]
    ErrorLinks {
        #[rename(self)]
        current: Option<CURRENT>: IsLink;
        about: Option<ABOUT>: IsLink;
        #[flatten]
        others: Option<OTHERS>: Debug;
    }
}

super::macros::generate_object! {
    #[mark(IsErrorSource)]
    #[unsafe_mark(IsPresent)]
    ErrorSource {
        pointer: Option<POINTER>: FromStr + Debug;
        parameter: Option<PARAMETER>: FromStr + Debug;
        header: Option<HEADER>: FromStr + Debug;
    }
}

super::macros::generate_object! {
    #[unsafe_mark(IsPresent)]
    Error {
        id: Option<ID>: FromStr + Debug;
        code: Option<CODE>: FromStr + Debug;
        status: Option<STATUS>: FromStr + Debug;
        detail: Option<DETAIL>: FromStr + Debug;
        title: Option<TITLE>: FromStr + Debug;
        source: Option<SOURCE>: IsErrorSource;
        links: Option<LINKS>: Debug;
        #[rename(meta)]
        metadata: Option<METADATA>: Debug;
    }
}

super::macros::generate_alias! {
    #[mark(IsErrorCollection)]
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
        SOURCE: IsErrorSource;
        LINKS: Debug;
        METADATA: Debug;
    }
}
