use super::link::IsLink;
use super::pagination_links::IsPaginationLinks;
use super::present::IsPresent;
use super::resource_identifier::{
    IsResourceIdentifier, IsResourceIdentifierCollecion, IsResourceIdentifierSingle,
};
use core::fmt::Debug;

super::macros::generate_markers! {
    IsRelationshipLinks: Debug {
        #[dyn]
        IsPaginationLinks;
    }
    IsRelationshipData: Debug {
        #[dyn]
        IsResourceIdentifier;
    }
    IsRelationshipToOneData: IsRelationshipData {
        #[dyn]
        IsResourceIdentifierSingle;
    }
    IsRelationshipToManyData: IsRelationshipData {
        #[dyn]
        IsResourceIdentifierCollecion;
    }
}

super::macros::generate_object! {
    #[mark(IsRelationshipLinks)]
    #[unsafe_mark(IsPresent)]
    RelationshipLinks {
        #[rename(self)]
        current, this: Option<CURRENT>: IsLink;
        related: Option<RELATED>: IsLink;
        article: Option<ARTICLE>: IsLink;
        #[flatten]
        pagination: Option<PAGINATION>: IsPaginationLinks;
    }
}

super::macros::generate_object! {
    Relationship {
        data: DATA: IsRelationshipData;
        links: Option<LINKS>: IsRelationshipLinks;
        metadata, meta: Option<METADATA>: Debug;
    }
}

super::macros::generate_object! {
    RelationshipToOne {
        data: DATA: IsRelationshipToOneData;
        links: Option<LINKS>: IsRelationshipLinks;
        metadata, meta: Option<METADATA>: Debug;
    }
}

super::macros::generate_object! {
    RelationshipToMany {
        data: DATA: IsRelationshipToManyData;
        links: Option<LINKS>: IsRelationshipLinks;
        metadata, meta: Option<METADATA>: Debug;
    }
}
