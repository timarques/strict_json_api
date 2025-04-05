pub mod markers {
    use super::super::present::NotPresent;
    use core::fmt::Debug;
    use core::str::FromStr;

    #[allow(clippy::missing_safety_doc)]
    pub trait HrefLanguage: Debug {}

    impl HrefLanguage for String {}
    impl HrefLanguage for &str {}
    impl HrefLanguage for NotPresent {}
    impl<T> HrefLanguage for Vec<T> where T: FromStr + Debug {}
    impl<T> HrefLanguage for Option<T> where T: HrefLanguage {}

    #[allow(clippy::missing_safety_doc)]
    pub trait Link: Debug {}

    impl Link for String {}
    impl Link for &str {}
    impl Link for NotPresent {}
    impl<L> Link for Option<L> where L: Link {}
}

use super::present::Present;

use core::fmt::Debug;
use core::str::FromStr;

super::macros::generate_object! {
    #[markers(markers::Link)]
    #[unsafe_markers(Present)]
    Link {
        TYPE: FromStr + Debug: r#type, kind: Option<TYPE>;
        HREF: FromStr + Debug + Present: href: HREF;
        #[rename(rel)]
        RELATION: FromStr + Debug: relation, rel: RELATION;
        #[rename(describedby)]
        DESCRIBEDBY: FromStr + Debug: described_by: DESCRIBEDBY;
        TITLE: FromStr + Debug: title: Option<TITLE>;
        #[rename(hreflang)]
        HREFLANGUAGE: markers::HrefLanguage: href_language: Option<HREFLANGUAGE>;
        META: Debug: template: Option<META>;
    }
}
