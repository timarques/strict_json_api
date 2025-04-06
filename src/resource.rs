use super::present::{NotPresent, Present};
use super::resource_identifier::{
    ResourceIdentifier, ResourceIdentifierCollection, ResourceIdentifierObject,
};

use core::fmt::Debug;
use core::str::FromStr;

super::macros::generate_markers! {
    Resource: Debug: Option<T>, NotPresent, Vec<T>;
    ResponseResource: Debug: Option<T>, NotPresent, Vec<T>;
    IncludedResources: Debug: Option<T>, NotPresent;
}

super::macros::generate_object! {
    #[mark(Resource)]
    #[unsafe_mark(Present)]
    ResourceObject {
        #[flatten]
        identifier: Option<IDENTIFIER>: ResourceIdentifier;
        attributes: Option<ATTRIBUTES>: Debug;
        relationships: Option<RELATIONSHIPS>: Debug;
        links: Option<LINKS>: Debug;
    }
}

super::macros::generate_wrapper_object! {
    #[mark(Resource)]
    #[unsafe_mark(Present)]
    #[wrap]
    ResourceCollection:
    Vec<
        ResourceObject<
            IDENTIFIER,
            ATTRIBUTES,
            RELATIONSHIPS,
            LINKS
        >
    >
    {
        IDENTIFIER: ResourceIdentifier;
        ATTRIBUTES: Debug;
        RELATIONSHIPS: Debug;
        LINKS: Debug;
    }
}

super::macros::generate_wrapper_object! {
    #[mark(ResponseResource, Resource)]
    #[unsafe_mark(Present)]
    #[wrap]
    ResponseResourceObject:
    ResourceObject<
        ResourceIdentifierObject<
            ID,
            TYPE,
            NotPresent,
            METADATA
        >,
        ATTRIBUTES,
        RELATIONSHIPS,
        LINKS
    >
    {
        ID: FromStr + Debug + Present;
        TYPE: FromStr + Debug + Present;
        METADATA: Debug;
        ATTRIBUTES: Debug;
        RELATIONSHIPS: Debug;
        LINKS: Debug;
    }
}

super::macros::generate_wrapper_object! {
    #[mark(ResponseResource, IncludedResources, Resource)]
    #[unsafe_mark(Present)]
    #[wrap]
    ResponseResourceCollection:
        ResourceObject<
            ResourceIdentifierCollection<
                ID,
                TYPE,
                NotPresent,
                METADATA
            >,
            ATTRIBUTES,
            RELATIONSHIPS,
            LINKS
        > {
            ID: FromStr + Debug + Present;
            TYPE: FromStr + Debug + Present;
            METADATA: Debug;
            ATTRIBUTES: Debug;
            RELATIONSHIPS: Debug;
            LINKS: Debug;
        }
}
