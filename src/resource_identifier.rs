use super::present::{NotPresent, Present};
use core::fmt::Debug;
use core::str::FromStr;

super::macros::generate_markers! {
    IsResourceIdentifier: Debug + Present;
    IsSingleResourceIdentifier: IsResourceIdentifier;
    IsResourceResponseIdentifier: IsResourceIdentifier;
}

super::macros::generate_object! {
    #[mark(IsResourceIdentifier, IsSingleResourceIdentifier)]
    #[unsafe_mark(Present)]
    ResourceIdentifier {
        r#type, kind: TYPE: FromStr + Debug + Present;
        id: Option<ID>: FromStr + Debug;
        lid: Option<LID>: FromStr + Debug;
        #[rename(meta)]
        metadata, meta: Option<METADATA>: Debug;
    }
}

super::macros::generate_alias! {
    #[mark(IsResourceIdentifier)]
    #[unsafe_mark(Present)]
    ResourceIdentifierCollection:
    Vec<
        ResourceIdentifier<TYPE, ID, LID, METADATA>
    >
    {
        TYPE: FromStr + Debug + Present;
        ID: FromStr + Debug;
        LID: FromStr + Debug;
        METADATA: Debug;
    }
}

super::macros::generate_alias! {
    #[mark(IsResourceResponseIdentifier)]
    ResourceResponseIdentifier:
    ResourceIdentifier<TYPE, ID, NotPresent, METADATA>
    {
        TYPE: FromStr + Debug + Present;
        ID: FromStr + Debug + Present;
        METADATA: Debug;
    }
}
