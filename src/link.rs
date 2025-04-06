use super::present::{NotPresent, Present};

use core::fmt::Debug;
use core::str::FromStr;

super::macros::generate_markers! {
    HrefLanguage: Debug: String, &str, Vec<T>, Option<T>, NotPresent;
    Link: Debug: String, &str, Option<T>, NotPresent;
}

super::macros::generate_object! {
    #[mark(Link)]
    #[unsafe_mark(Present)]
    LinkObject {
        // this needs to be present
        href: HREF: FromStr + Debug + Present;
        r#type, kind: Option<TYPE>: FromStr + Debug;
        #[rename(rel)]
        relation, rel: Option<RELATION>: FromStr + Debug;
        #[rename(describedby)]
        described_by: Option<DESCRIBEDBY>: FromStr + Debug;
        title: Option<TITLE>: FromStr + Debug;
        #[rename(hreflang)]
        href_language: Option<HREFLANGUAGE>: HrefLanguage;
        #[rename(meta)]
        metadata, meta: Option<METADATA>: Debug;
    }
}
