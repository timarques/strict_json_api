use super::present::{IsPresent, NotPresent};
use core::fmt::Debug;
use core::str::FromStr;

super::macros::generate_markers! {
    IsResourceIdentifier: Debug + IsPresent {}
    IsResourceIdentifierSingle: IsResourceIdentifier {}
    IsResourceIdentifierCollecion: IsResourceIdentifier {}
    IsResourceIdentifierWithoutLid: IsResourceIdentifier {}
}

super::macros::generate_object! {
    #[mark(IsResourceIdentifier, IsResourceIdentifierSingle)]
    #[unsafe_mark(IsPresent)]
    ResourceIdentifier {
        #[rename(r#type)]
        resource_type: TYPE: FromStr + Debug + IsPresent;
        id: Option<ID>: FromStr + Debug;
        lid: Option<LID>: FromStr + Debug;
        #[rename(meta)]
        metadata, meta: Option<METADATA>: Debug;
    }
}

super::macros::generate_alias! {
    #[mark(IsResourceIdentifier, IsResourceIdentifierCollecion)]
    ResourceIdentifierCollection:
    Vec<
        ResourceIdentifier<TYPE, ID, LID, METADATA>
    >
    {
        TYPE: FromStr + Debug + IsPresent;
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
        TYPE: FromStr + Debug + IsPresent;
        ID: FromStr + Debug + IsPresent;
        METADATA: Debug;
    }
}
