pub mod markers {
    use super::super::present::NotPresent;
    use core::fmt::Debug;

    // ERRORS
    pub trait Errors: Debug {}

    impl Errors for NotPresent {}
    impl<T> Errors for Option<T> where T: Errors {}

    // INCLUDED
    pub trait Included: Debug {}

    impl Included for NotPresent {}
    impl<T> Included for Option<T> where T: Included {}

    // DATA
    pub trait Data: Debug {}

    impl Data for NotPresent {}
    impl<T> Data for Option<T> where T: Data {}

    // JSON_API
    pub trait JsonApi: Debug {}

    impl JsonApi for NotPresent {}
    impl<T> JsonApi for Option<T> where T: JsonApi {}

    // LINKS
    pub trait Links: Debug {}

    impl Links for NotPresent {}
    impl<T> Links for Option<T> where T: Links {}
}

use self::markers::{Data, Errors, Included, JsonApi, Links};
use super::present::Present;

use core::fmt::Debug;

super::macros::generate_object! {
    Document {
        DATA: Data: data: Option<DATA>;
        INCLUDED: Included: included: Option<INCLUDED>;
        JSONAPI: JsonApi: jsonapi: Option<JSONAPI>;
        LINKS: Links: links: Option<LINKS>;
        #[rename(meta)]
        METADATA: Debug: metadata, meta: Option<METADATA>;
    }
}

super::macros::generate_object! {
    ErrorDocument {
        ERRORS: Errors: errors: Option<ERRORS>;
        JSONAPI: JsonApi: jsonapi: Option<JSONAPI>;
        LINKS: Links: links: Option<LINKS>;
        #[rename(meta)]
        METADATA: Debug: metadata, meta: Option<METADATA>;
    }
}

impl<ERRORS, JSONAPI, LINKS, METADATA> core::error::Error
    for ErrorDocument<ERRORS, JSONAPI, LINKS, METADATA>
where
    ERRORS: Errors + Present,
    JSONAPI: JsonApi,
    LINKS: Links,
    METADATA: Debug,
{
}

impl<ERRORS, JSONAPI, LINKS, METADATA> core::fmt::Display
    for ErrorDocument<ERRORS, JSONAPI, LINKS, METADATA>
where
    ERRORS: Errors + Present,
    JSONAPI: JsonApi,
    LINKS: Links,
    METADATA: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.errors().fmt(f)
    }
}
