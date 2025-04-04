use super::super::documents::markers::{Data, Included};
use super::super::present::{NotPresent, Present};
use super::request_resource::RequestResource;

use core::fmt::Debug;
use core::str::FromStr;

super::super::macros::generate_wrapper_object! {
    #[unsafe_markers(Data, Present)]
    ResponseResource: RequestResource<TYPE, ID, NotPresent, ATTRIBUTES, RELATIONSHIPS, LINKS, METADATA> {
        TYPE: FromStr + Debug + Present;
        ID: FromStr + Debug + Present;
        ATTRIBUTES: Debug;
        RELATIONSHIPS: Debug;
        LINKS: Debug;
        METADATA: Debug;
    }
}

super::super::macros::generate_wrapper_object! {
    #[unsafe_markers(Data, Included, Present)]
    ResponseResourceCollection: Vec<RequestResource<TYPE, ID, NotPresent, ATTRIBUTES, RELATIONSHIPS, LINKS, METADATA>> {
        TYPE: FromStr + Debug + Present;
        ID: FromStr + Debug + Present;
        ATTRIBUTES: Debug;
        RELATIONSHIPS: Debug;
        LINKS: Debug;
        METADATA: Debug;
    }
}
