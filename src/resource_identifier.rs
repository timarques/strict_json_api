use super::present::{NotPresent, Present};

use core::fmt::Debug;
use core::str::FromStr;

super::macros::generate_markers! {
    ResourceIdentifier: Debug: Option<T>, NotPresent, Vec<T>;
}

super::macros::generate_object! {
    #[mark(ResourceIdentifier)]
    #[unsafe_mark(Present)]
    ResourceIdentifierObject {
        r#type, kind: TYPE: FromStr + Debug + Present;
        id: Option<ID>: FromStr + Debug;
        lid: Option<LID>: FromStr + Debug;
        #[rename(meta)]
        metadata, meta: Option<METADATA>: Debug;
    }
}

super::macros::generate_wrapper_object! {
    #[mark(ResourceIdentifier)]
    #[unsafe_mark(Present)]
    #[wrap]
    ResourceIdentifierCollection: Vec<ResourceIdentifierObject<TYPE, ID, LID, METADATA>> {
        TYPE: FromStr + Debug + Present;
        ID: FromStr + Debug + Present;
        LID: FromStr + Debug;
        METADATA: Debug;
    }
}
