#[doc(hidden)]
pub mod errors {
    use super::super::super::present::NotPresent;
    use core::fmt::Debug;

    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait Errors: Debug {}

    unsafe impl Errors for NotPresent {}
    unsafe impl<T> Errors for Option<T> where T: Errors {}
}

#[doc(hidden)]
pub mod included {
    use super::super::super::present::NotPresent;

    use core::fmt::Debug;

    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait Included: Debug {}

    unsafe impl Included for NotPresent {}
    unsafe impl<T> Included for Option<T> where T: Included {}
}

#[doc(hidden)]
pub mod data {
    use super::super::super::present::NotPresent;
    use core::fmt::Debug;

    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait Data: Debug {}

    unsafe impl Data for NotPresent {}
    unsafe impl<T> Data for Option<T> where T: Data {}
}

#[doc(hidden)]
pub mod json_api {
    use super::super::super::present::NotPresent;
    use core::fmt::Debug;

    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait JsonApi: Debug {}

    unsafe impl JsonApi for NotPresent {}
    unsafe impl<T> JsonApi for Option<T> where T: JsonApi {}
}

#[doc(hidden)]
pub mod links {
    use super::super::super::present::NotPresent;
    use core::fmt::Debug;

    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait Links: Debug {}

    unsafe impl Links for NotPresent {}
    unsafe impl<T> Links for Option<T> where T: Links {}
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
