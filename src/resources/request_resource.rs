use super::super::documents::markers::Data;
use super::super::present::Present;

use core::fmt::Debug;
use core::str::FromStr;

super::super::macros::generate_object! {
    #[markers(Data)]
    #[unsafe_markers(Present)]
    RequestResource {
        TYPE: FromStr + Debug + Present: r#type, kind: TYPE;
        ID: FromStr + Debug: id: Option<ID>;
        LID: FromStr + Debug: lid: Option<LID>;
        ATTRIBUTES: Debug: attributes: Option<ATTRIBUTES>;
        RELATIONSHIPS: Debug: relationships: Option<RELATIONSHIPS>;
        LINKS: Debug: links: Option<LINKS>;
        #[rename(meta)]
        METADATA: Debug: metadata, meta: Option<METADATA>;
    }
}

super::super::macros::generate_wrapper_object! {
    #[markers(Data)]
    #[unsafe_markers(Present)]
    RequestResourceCollection: Vec<RequestResource<TYPE, ID, LID, ATTRIBUTES, RELATIONSHIPS, LINKS, METADATA>> {
        TYPE: FromStr + Debug + Present;
        ID: FromStr + Debug;
        LID: FromStr + Debug;
        ATTRIBUTES: Debug;
        RELATIONSHIPS: Debug;
        LINKS: Debug;
        METADATA: Debug;
    }
}
