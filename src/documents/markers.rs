#[doc(hidden)]
pub mod errors {
    use super::super::super::present::NotPresent;
    use core::fmt::Debug;

    #[allow(clippy::missing_safety_doc)]
    pub trait Errors: Debug {}

    impl Errors for NotPresent {}
    impl<T> Errors for Option<T> where T: Errors {}
}

#[doc(hidden)]
pub mod included {
    use super::super::super::present::NotPresent;

    use core::fmt::Debug;

    #[allow(clippy::missing_safety_doc)]
    pub trait Included: Debug {}

    impl Included for NotPresent {}
    impl<T> Included for Option<T> where T: Included {}
}

#[doc(hidden)]
pub mod data {
    use super::super::super::present::NotPresent;
    use core::fmt::Debug;

    #[allow(clippy::missing_safety_doc)]
    pub trait Data: Debug {}

    impl Data for NotPresent {}
    impl<T> Data for Option<T> where T: Data {}
}

#[doc(hidden)]
pub mod json_api {
    use super::super::super::present::NotPresent;
    use core::fmt::Debug;

    #[allow(clippy::missing_safety_doc)]
    pub trait JsonApi: Debug {}

    impl JsonApi for NotPresent {}
    impl<T> JsonApi for Option<T> where T: JsonApi {}
}

#[doc(hidden)]
pub mod links {
    use super::super::super::present::NotPresent;
    use core::fmt::Debug;

    #[allow(clippy::missing_safety_doc)]
    pub trait Links: Debug {}

    impl Links for NotPresent {}
    impl<T> Links for Option<T> where T: Links {}
}

#[doc(inline)]
pub use data::Data;
#[doc(inline)]
pub use errors::Errors;
#[doc(inline)]
pub use included::Included;
#[doc(inline)]
pub use json_api::JsonApi;
#[doc(inline)]
pub use links::Links;
