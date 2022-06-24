use std::ops::Deref;

use idb_sys::KeyRange as SysKeyRange;
use wasm_bindgen::JsValue;

use crate::Error;

/// Represents a continuous interval over some data type that is used for keys.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyRange {
    inner: SysKeyRange,
}

impl KeyRange {
    /// Returns a new [`KeyRange`] spanning only key.
    pub fn only(value: &JsValue) -> Result<Self, Error> {
        let inner = SysKeyRange::only(value)?;

        Ok(Self { inner })
    }

    /// Returns a new [`KeyRange`] spanning from lower to upper. If `lower_open` is true, `lower` is not included in the
    /// range. If `upper_open` is true, `upper` is not included in the range.
    pub fn bound(
        lower: &JsValue,
        upper: &JsValue,
        lower_open: Option<bool>,
        upper_open: Option<bool>,
    ) -> Result<Self, Error> {
        let inner = SysKeyRange::bound(lower, upper, lower_open, upper_open)?;

        Ok(Self { inner })
    }

    /// Returns a new [`KeyRange`] starting at key with no upper bound. If `lower_open` is true, key is not included in
    /// the range.
    pub fn lower_bound(lower: &JsValue, lower_open: Option<bool>) -> Result<Self, Error> {
        let inner = SysKeyRange::lower_bound(lower, lower_open)?;

        Ok(Self { inner })
    }

    /// Returns a new [`KeyRange`] with no lower bound and ending at key. If `upper_open` is true, key is not included
    /// in the range.
    pub fn upper_bound(upper: &JsValue, upper_open: Option<bool>) -> Result<Self, Error> {
        let inner = SysKeyRange::upper_bound(upper, upper_open)?;

        Ok(Self { inner })
    }

    /// Returns the range’s lower bound, or undefined if none.
    pub fn lower(&self) -> Result<JsValue, Error> {
        self.inner.lower().map_err(Into::into)
    }

    /// Returns the range’s upper bound, or undefined if none.
    pub fn upper(&self) -> Result<JsValue, Error> {
        self.inner.upper().map_err(Into::into)
    }

    /// Returns the range’s lower open flag.
    pub fn lower_open(&self) -> bool {
        self.inner.lower_open()
    }

    /// Returns the range’s upper open flag.
    pub fn upper_open(&self) -> bool {
        self.inner.upper_open()
    }

    /// Returns true if key is included in the range, and false otherwise.
    pub fn includes(&self, value: &JsValue) -> Result<bool, Error> {
        self.inner.includes(value).map_err(Into::into)
    }
}

impl Deref for KeyRange {
    type Target = SysKeyRange;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<SysKeyRange> for KeyRange {
    fn from(inner: SysKeyRange) -> Self {
        KeyRange { inner }
    }
}

impl From<KeyRange> for SysKeyRange {
    fn from(key_range: KeyRange) -> Self {
        key_range.inner
    }
}

impl From<JsValue> for KeyRange {
    fn from(value: JsValue) -> Self {
        let inner: SysKeyRange = value.into();
        inner.into()
    }
}

impl From<KeyRange> for JsValue {
    fn from(value: KeyRange) -> Self {
        value.inner.into()
    }
}
