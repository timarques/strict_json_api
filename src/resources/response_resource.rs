use super::super::documents::markers::{Data, Included};
use super::super::present::Present;

use core::fmt::Debug;
use core::str::FromStr;

super::super::macros::generate_object! {
    #[unsafe_markers(Data, Present)]
    ResponseResource {
        TYPE: FromStr + Debug + Present: r#type, kind: TYPE;
        ID: FromStr + Debug + Present: id: ID;
        ATTRIBUTES: Debug: attributes: Option<ATTRIBUTES>;
        RELATIONSHIPS: Debug: relationships: Option<RELATIONSHIPS>;
        LINKS: Debug: links: Option<LINKS>;
        #[rename(meta)]
        METADATA: Debug: metadata, meta: Option<METADATA>;
    }
}

super::super::macros::generate_wrapper_object! {
    #[unsafe_markers(Data, Included, Present)]
    ResponseResourceCollection: Vec<ResponseResource<TYPE, ID, ATTRIBUTES, RELATIONSHIPS, LINKS, METADATA>> {
        TYPE: FromStr + Debug + Present;
        ID: FromStr + Debug + Present;
        ATTRIBUTES: Debug;
        RELATIONSHIPS: Debug;
        LINKS: Debug;
        METADATA: Debug;
    }
}
