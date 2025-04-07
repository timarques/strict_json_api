use super::present::{NotPresent, Present};

use core::fmt::Debug;
use core::str::FromStr;

super::macros::generate_markers! {
    IsHrefLanguage: Debug {
        String;
        &str;
        #[wrap]
        Vec;
        #[wrap]
        Option;
        NotPresent;
    }
}

super::macros::generate_markers! {
    IsLink: Debug {
        String;
        &str;
        #[wrap]
        Option;
        NotPresent;
    }
}

super::macros::generate_object! {
    #[mark(IsLink)]
    #[unsafe_mark(Present)]
    Link {
        // this needs to be present
        href: HREF: FromStr + Debug + Present;
        #[rename(r#type)]
        resource_type: Option<TYPE>: FromStr + Debug;
        #[rename(rel)]
        relation, rel: Option<RELATION>: FromStr + Debug;
        #[rename(describedby)]
        described_by: Option<DESCRIBEDBY>: FromStr + Debug;
        title: Option<TITLE>: FromStr + Debug;
        #[rename(hreflang)]
        href_language: Option<HREFLANGUAGE>: IsHrefLanguage;
        #[rename(meta)]
        metadata, meta: Option<METADATA>: Debug;
    }
}
