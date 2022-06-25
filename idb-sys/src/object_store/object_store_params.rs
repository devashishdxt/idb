use std::ops::Deref;

use wasm_bindgen::{JsCast, JsValue};
use web_sys::IdbObjectStoreParameters;

use crate::{Error, KeyPath};

/// Options when creating an [`ObjectStore`](crate::ObjectStore).
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ObjectStoreParams {
    inner: IdbObjectStoreParameters,
}

impl ObjectStoreParams {
    /// Creates an new instance of [`ObjectStoreParams`].
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the `auto_increment` flag.
    pub fn auto_increment(&mut self, auto_increment: bool) -> &mut Self {
        self.inner.auto_increment(auto_increment);
        self
    }

    /// Sets the key path.
    pub fn key_path(&mut self, key_path: Option<KeyPath>) -> &mut Self {
        self.inner.key_path(key_path.map(Into::into).as_ref());
        self
    }
}

impl Deref for ObjectStoreParams {
    type Target = IdbObjectStoreParameters;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<IdbObjectStoreParameters> for ObjectStoreParams {
    fn from(inner: IdbObjectStoreParameters) -> Self {
        Self { inner }
    }
}

impl From<ObjectStoreParams> for IdbObjectStoreParameters {
    fn from(params: ObjectStoreParams) -> Self {
        params.inner
    }
}

impl TryFrom<JsValue> for ObjectStoreParams {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        value
            .dyn_into::<IdbObjectStoreParameters>()
            .map(Into::into)
            .map_err(|value| Error::UnexpectedJsType("IdbObjectStoreParameters", value))
    }
}

impl From<ObjectStoreParams> for JsValue {
    fn from(value: ObjectStoreParams) -> Self {
        value.inner.into()
    }
}
