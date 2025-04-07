use super::present::{NotPresent, Present};
use super::resource_identifier::{IsResourceIdentifierWithoutLid, IsSingularResourceIdentifier};

use core::fmt::Debug;

super::macros::generate_markers! {
    IsResource: Debug: Option<T>, NotPresent;
    IsResourceWithoutLid: IsResource: Option<T>, NotPresent;
    IsResourceWithoutLidCollection: IsResourceWithoutLid: Option<T>, NotPresent;
}

super::macros::generate_object! {
    #[mark(IsResource)]
    #[unsafe_mark(Present)]
    Resource {
        #[flatten]
        identifier: Option<IDENTIFIER>: IsSingularResourceIdentifier + Present;
        attributes: Option<ATTRIBUTES>: Debug;
        relationships: Option<RELATIONSHIPS>: Debug;
        links: Option<LINKS>: Debug;
    }
}

super::macros::generate_alias! {
    #[mark(IsResource)]
    #[unsafe_mark(Present)]
    ResourceCollection:
    Vec<
        Resource<
            IDENTIFIER,
            ATTRIBUTES,
            RELATIONSHIPS,
            LINKS
        >
    >
    {
        IDENTIFIER: IsSingularResourceIdentifier + Present;
        ATTRIBUTES: Debug;
        RELATIONSHIPS: Debug;
        LINKS: Debug;
    }
}

super::macros::generate_alias! {
    #[mark(IsResourceWithoutLid)]
    ResourceWithoutLid:
    Resource<
        IDENTIFIER,
        ATTRIBUTES,
        RELATIONSHIPS,
        LINKS
    >
    {
        IDENTIFIER: IsSingularResourceIdentifier + IsResourceIdentifierWithoutLid + Present;
        ATTRIBUTES: Debug;
        RELATIONSHIPS: Debug;
        LINKS: Debug;
    }
}

super::macros::generate_alias! {
    #[mark(IsResourceWithoutLid, IsResourceWithoutLidCollection)]
    ResourceWithoutLidCollection:
    Vec<
        Resource<
            IDENTIFIER,
            ATTRIBUTES,
            RELATIONSHIPS,
            LINKS
        >,
    >
    {
        IDENTIFIER: IsSingularResourceIdentifier + IsResourceIdentifierWithoutLid + Present;
        ATTRIBUTES: Debug;
        RELATIONSHIPS: Debug;
        LINKS: Debug;
    }
}
