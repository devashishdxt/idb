use std::ops::Deref;

use wasm_bindgen::JsValue;
use web_sys::IdbIndexParameters;

/// Options when creating [`Index`](crate::Index).
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct IndexParams {
    inner: IdbIndexParameters,
}

impl IndexParams {
    /// Creates a new instance of [`IndexParams`]
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the `unique` flag.
    pub fn unique(&mut self, unique: bool) -> &mut Self {
        self.inner.unique(unique);
        self
    }

    /// Sets the `multi_entry` flag.
    pub fn multi_entry(&mut self, multi_entry: bool) -> &mut Self {
        self.inner.multi_entry(multi_entry);
        self
    }
}

impl Deref for IndexParams {
    type Target = IdbIndexParameters;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<IdbIndexParameters> for IndexParams {
    fn from(inner: IdbIndexParameters) -> Self {
        Self { inner }
    }
}

impl From<IndexParams> for IdbIndexParameters {
    fn from(params: IndexParams) -> Self {
        params.inner
    }
}

impl From<JsValue> for IndexParams {
    fn from(value: JsValue) -> Self {
        let inner = value.into();
        Self { inner }
    }
}

impl From<IndexParams> for JsValue {
    fn from(value: IndexParams) -> Self {
        value.inner.into()
    }
}
