use super::link::Link;
use super::pagination::Pagination;
use super::present::{NotPresent, Present};
use super::resource_identifier::{
    ResourceIdentifier, ResourceIdentifierCollection, ResourceIdentifierObject,
};
use core::fmt::Debug;
use core::str::FromStr;

super::macros::generate_markers! {
    RelationshipLinks: Debug: Option<T>, NotPresent;
}

super::macros::generate_object! {
    #[mark(RelationshipLinks)]
    #[unsafe_mark(Present)]
    RelationshipLinksObject {
        #[rename(self)]
        current, this: Option<CURRENT>: Link;
        related: Option<RELATED>: Link;
        article: Option<ARTICLE>: Link;
        #[flatten]
        pagination: Option<PAGINATION>: Pagination;
    }
}

super::macros::generate_object! {
    #[unsafe_mark(Present)]
    RelationshipObject {
        data, identifier: Option<DATA>: ResourceIdentifier;
        links: Option<LINKS>: RelationshipLinks;
        metadata, meta: Option<METADATA>: Debug;
    }
}

super::macros::generate_wrapper_object! {
    #[unsafe_mark(Present)]
    #[wrap]
    ResponseRelationshipObject: RelationshipObject<
        ResourceIdentifierObject<ID, TYPE, NotPresent, DATA_METADATA>,
        LINKS,
        METADATA,
    >
    {
        ID: FromStr + Debug + Present;
        TYPE: FromStr + Debug + Present;
        DATA_METADATA: Debug;
        LINKS: RelationshipLinks;
        METADATA: Debug;
    }
}

super::macros::generate_wrapper_object! {
    #[unsafe_mark(Present)]
    #[wrap]
    ResponseRelationshipCollection: RelationshipObject<
        ResourceIdentifierCollection<ID, TYPE, NotPresent, DATA_METADATA>,
        LINKS,
        METADATA,
    >
    {
        ID: FromStr + Debug + Present;
        TYPE: FromStr + Debug + Present;
        DATA_METADATA: Debug;
        LINKS: RelationshipLinks;
        METADATA: Debug;
    }
}
