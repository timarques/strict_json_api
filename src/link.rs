pub mod markers {
    use super::super::present::NotPresent;
    use core::fmt::Debug;
    use core::str::FromStr;

    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait HrefLanguage: Debug {}

    unsafe impl HrefLanguage for String {}
    unsafe impl HrefLanguage for &str {}
    unsafe impl HrefLanguage for NotPresent {}
    unsafe impl<T> HrefLanguage for Vec<T> where T: FromStr + Debug {}
    unsafe impl<T> HrefLanguage for Option<T> where T: HrefLanguage {}

    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait Link: Debug {}

    unsafe impl Link for String {}
    unsafe impl Link for &str {}
    unsafe impl Link for NotPresent {}
    unsafe impl<L> Link for Option<L> where L: Link {}
}

use super::present::Present;

use core::fmt::Debug;
use core::str::FromStr;

super::macros::generate_object! {
    #[unsafe_markers(markers::Link, Present)]
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
