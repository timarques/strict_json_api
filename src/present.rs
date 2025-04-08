use core::fmt::Debug;
use core::str::FromStr;
use serde::de::IgnoredAny;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy)]
pub struct NotPresent {}

impl<'de> Deserialize<'de> for NotPresent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let _ = deserializer.deserialize_ignored_any(IgnoredAny);
        Ok(Self::default())
    }
}

impl Serialize for NotPresent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_none()
    }
}

impl FromStr for NotPresent {
    type Err = ();
    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(Self::default())
    }
}

#[allow(clippy::missing_safety_doc)]
pub unsafe trait IsPresent {}

unsafe impl IsPresent for String {}
unsafe impl IsPresent for u16 {}
unsafe impl IsPresent for u32 {}
unsafe impl IsPresent for usize {}
unsafe impl IsPresent for f32 {}
unsafe impl<T> IsPresent for Option<T> {}
unsafe impl<T> IsPresent for Vec<T> where T: IsPresent {}
