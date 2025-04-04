use super::super::present::{NotPresent, Present};
use super::super::relationship::markers::Data;
use super::request_resource::RequestResource;

use core::fmt::Debug;
use core::str::FromStr;

super::super::macros::generate_wrapper_object! {
    #[unsafe_markers(Data, Present)]
    LinkageResource: RequestResource<TYPE, ID, NotPresent, NotPresent, NotPresent, NotPresent, METADATA> {
        TYPE: FromStr + Debug + Present;
        ID: FromStr + Debug + Present;
        METADATA: Debug;
    }
}

super::super::macros::generate_wrapper_object! {
    #[unsafe_markers(Data, Present)]
    LinkageResourceCollection: Vec<RequestResource<TYPE, ID, NotPresent, NotPresent, NotPresent, NotPresent, METADATA>> {
        TYPE: FromStr + Debug + Present;
        ID: FromStr + Debug + Present;
        METADATA: Debug;
    }
}
