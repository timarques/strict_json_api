use super::present::{NotPresent, Present};
use super::resource_identifier::{IsResourceIdentifierResponse, IsResourceIdentifierSingle};

use core::fmt::Debug;

super::macros::generate_markers! {
    IsResource: Debug: Option<T>, NotPresent;
    IsResourceResponse: IsResource: Option<T>, NotPresent;
    IsResourceResponseCollection: IsResourceResponse: Option<T>, NotPresent;
}

super::macros::generate_object! {
    #[mark(IsResource)]
    #[unsafe_mark(Present)]
    Resource {
        #[flatten]
        identifier: Option<IDENTIFIER>: IsResourceIdentifierSingle;
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
        IDENTIFIER: IsResourceIdentifierSingle;
        ATTRIBUTES: Debug;
        RELATIONSHIPS: Debug;
        LINKS: Debug;
    }
}

super::macros::generate_alias! {
    #[mark(IsResourceResponse)]
    ResourceResponse:
    Resource<
        IDENTIFIER,
        ATTRIBUTES,
        RELATIONSHIPS,
        LINKS
    >
    {
        IDENTIFIER: IsResourceIdentifierSingle + IsResourceIdentifierResponse;
        ATTRIBUTES: Debug;
        RELATIONSHIPS: Debug;
        LINKS: Debug;
    }
}

super::macros::generate_alias! {
    #[mark(IsResourceResponse, IsResourceResponseCollection)]
    ResourceResponseCollection:
    Vec<
        ResourceResponse<
            IDENTIFIER,
            ATTRIBUTES,
            RELATIONSHIPS,
            LINKS
        >,
    >
    {
        IDENTIFIER: IsResourceIdentifierSingle + IsResourceIdentifierResponse;
        ATTRIBUTES: Debug;
        RELATIONSHIPS: Debug;
        LINKS: Debug;
    }
}
