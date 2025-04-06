use core::fmt::Debug;
use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

pub struct Wrapper<T> {
    inner: T,
}

impl<T> Wrapper<T> {
    pub const fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> Deref for Wrapper<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Wrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> AsRef<T> for Wrapper<T> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<T> AsMut<T> for Wrapper<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: Clone> Clone for Wrapper<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: Debug> Debug for Wrapper<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T: Serialize> Serialize for Wrapper<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Wrapper<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let inner = serde::Deserialize::deserialize(deserializer)?;

        Ok(Self { inner })
    }
}
