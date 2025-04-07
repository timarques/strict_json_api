use super::link::IsLink;
use super::pagination_links::IsPaginationLinks;
use super::present::{NotPresent, Present};
use super::resource_identifier::IsResourceIdentifier;
use core::fmt::Debug;

super::macros::generate_markers! {
    IsRelationshipLinks: Debug: Option<T>, NotPresent;
}

super::macros::generate_object! {
    #[mark(IsRelationshipLinks)]
    #[unsafe_mark(Present)]
    RelationshipLinks {
        #[rename(self)]
        current, this: Option<CURRENT>: IsLink;
        related: Option<RELATED>: IsLink;
        article: Option<ARTICLE>: IsLink;
        #[flatten]
        pagination_links, pagination: Option<PAGINATION>: IsPaginationLinks;
    }
}

super::macros::generate_object! {
    RelationshipToOne {
        data, identifier: IDENTIFIER: IsResourceIdentifier + Present;
        links: Option<LINKS>: IsRelationshipLinks;
        metadata, meta: Option<METADATA>: Debug;
    }
}

super::macros::generate_alias! {
    RelationshipToMany:
    Vec<
        RelationshipToOne<
        IDENTIFIER,
        LINKS,
        METADATA
        >
    >
    {
        IDENTIFIER: IsResourceIdentifier + Present;
        LINKS: IsRelationshipLinks;
        METADATA: Debug;
    }
}
