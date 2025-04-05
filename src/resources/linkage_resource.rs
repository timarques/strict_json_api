use super::super::present::Present;
use super::super::relationship::markers::Data;

use core::fmt::Debug;
use core::str::FromStr;

super::super::macros::generate_object! {
    #[markers(Data)]
    #[unsafe_markers(Present)]
    LinkageResource {
        TYPE: FromStr + Debug + Present: r#type, kind: TYPE;
        ID: FromStr + Debug: id: Option<ID>;
        #[rename(meta)]
        METADATA: Debug: metadata, meta: Option<METADATA>;
    }
}

super::super::macros::generate_wrapper_object! {
    #[markers(Data)]
    #[unsafe_markers(Present)]
    LinkageResourceCollection: Vec<LinkageResource<TYPE, ID, METADATA>> {
        TYPE: FromStr + Debug + Present;
        ID: FromStr + Debug + Present;
        METADATA: Debug;
    }
}
