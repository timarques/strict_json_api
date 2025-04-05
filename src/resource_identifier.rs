use super::present::Present;
use super::relationship::markers::Data;
use super::resource::markers::Identifier;

use core::fmt::Debug;
use core::str::FromStr;

super::macros::generate_object! {
    #[markers(Data, Identifier)]
    #[unsafe_markers(Present)]
    ResourceIdentifier {
        TYPE: FromStr + Debug + Present: r#type, kind: TYPE;
        ID: FromStr + Debug: id: Option<ID>;
        LID: FromStr + Debug: lid: Option<LID>;
        #[rename(meta)]
        METADATA: Debug: metadata, meta: Option<METADATA>;
    }
}

super::macros::generate_wrapper_object! {
    #[markers(Data, Identifier)]
    #[unsafe_markers(Present)]
    ResourceIdentifierCollection: Vec<ResourceIdentifier<TYPE, ID, LID, METADATA>> {
        TYPE: FromStr + Debug + Present;
        ID: FromStr + Debug + Present;
        LID: FromStr + Debug + Present;
        METADATA: Debug;
    }
}
