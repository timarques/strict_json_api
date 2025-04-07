use super::present::{NotPresent, Present};

use core::fmt::Debug;
use core::str::FromStr;

super::macros::generate_markers! {
    IsResourceIdentifier: Debug: Option<T>, NotPresent;
    IsSingularResourceIdentifier: IsResourceIdentifier: Option<T>, NotPresent;
    IsResourceIdentifierWithoutLid: IsResourceIdentifier: Option<T>, NotPresent;
}

super::macros::generate_object! {
    #[mark(IsResourceIdentifier, IsSingularResourceIdentifier)]
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
    #[mark(IsResourceIdentifierWithoutLid)]
    ResourceIdentifierWithoutLid:
    ResourceIdentifier<TYPE, ID, NotPresent, METADATA>
    {
        TYPE: FromStr + Debug + Present;
        ID: FromStr + Debug;
        METADATA: Debug;
    }
}

super::macros::generate_alias! {
    #[mark(IsResourceIdentifierWithoutLid)]
    ResourceIdentifierWithoutLidCollection:
    Vec<
        ResourceIdentifierWithoutLid<
            TYPE,
            ID,
            METADATA
        >
    >
    {
        TYPE: FromStr + Debug + Present;
        ID: FromStr + Debug;
        METADATA: Debug;
    }
}
