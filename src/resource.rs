use super::present::{IsPresent, NotPresent};
use super::resource_identifier::{IsResourceIdentifierSingle, IsResourceIdentifierWithoutLid};

use core::fmt::Debug;

super::macros::generate_markers! {
    IsResource: Debug {
        #[wrap]
        Option;
        NotPresent;
    }
    IsResourceResponse: IsResource {
        #[wrap]
        Option;
        NotPresent;
    }
    IsResourceResponseCollection: IsResourceResponse {
        #[wrap]
        Option;
        NotPresent;
    }
}

super::macros::generate_object! {
    #[mark(IsResource)]
    #[unsafe_mark(IsPresent)]
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
        IDENTIFIER: IsResourceIdentifierSingle + IsResourceIdentifierWithoutLid;
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
        IDENTIFIER: IsResourceIdentifierSingle + IsResourceIdentifierWithoutLid;
        ATTRIBUTES: Debug;
        RELATIONSHIPS: Debug;
        LINKS: Debug;
    }
}
