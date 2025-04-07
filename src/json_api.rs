use super::link::IsLink;
use super::present::{NotPresent, Present};
use core::fmt::Debug;

use core::str::FromStr;

super::macros::generate_markers! {
    IsJsonApi: Debug {
        #[wrap]
        Option;
        NotPresent;
    }
}

super::macros::generate_object! {
    #[mark(IsJsonApi)]
    #[unsafe_mark(Present)]
    JsonApi {
        version: Option<VERSION>: FromStr + Debug;
        #[rename(ext)]
        extensions: Vec<EXTENSION>: IsLink;
        #[rename(profile)]
        profiles: Vec<PROFILE>: IsLink;
        #[rename(meta)]
        metadata: Option<METADATA>: Debug;
    }
}
