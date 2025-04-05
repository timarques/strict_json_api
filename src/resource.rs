pub mod markers {
    use super::super::present::Present;
    use core::fmt::Debug;
    pub trait Identifier: Debug + Present {}
}

use super::document::markers::{Data, Included};
use super::present::Present;

use core::fmt::Debug;

super::macros::generate_object! {
    #[markers(Data)]
    #[unsafe_markers(Present)]
    Resource {
        #[flatten]
        IDENTIFIER: markers::Identifier: identifier: IDENTIFIER;
        ATTRIBUTES: Debug: attributes: Option<ATTRIBUTES>;
        RELATIONSHIPS: Debug: relationships: Option<RELATIONSHIPS>;
        LINKS: Debug: links: Option<LINKS>;
    }
}

super::macros::generate_wrapper_object! {
    #[markers(Data, Included)]
    #[unsafe_markers(Present)]
    ResourceCollection: Vec<Resource<IDENTIFIER, ATTRIBUTES, RELATIONSHIPS, LINKS>> {
        IDENTIFIER: markers::Identifier;
        ATTRIBUTES: Debug;
        RELATIONSHIPS: Debug;
        LINKS: Debug;
    }
}
