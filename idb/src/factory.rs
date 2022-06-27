use idb_sys::Factory as SysFactory;
use wasm_bindgen::JsValue;

use crate::{utils::wait_request, Error, OpenRequest};

/// Lets applications asynchronously access the indexed databases.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Factory {
    inner: SysFactory,
}

impl Factory {
    /// Gets an instance of [Factory] from `global` scope.
    pub fn new() -> Result<Factory, Error> {
        let inner = SysFactory::new()?;
        Ok(Self { inner })
    }

    /// Attempts to open a connection to the named database with the specified version. If the database already exists
    /// with a lower version and there are open connections that don’t close in response to a `versionchange` event, the
    /// request will be blocked until they all close, then an upgrade will occur. If the database already exists with a
    /// higher version the request will fail.
    pub fn open(&self, name: &str, version: u32) -> Result<OpenRequest, Error> {
        self.inner
            .open(name, version)
            .map(Into::into)
            .map_err(Into::into)
    }

    /// Attempts to delete the named database. If the database already exists and there are open connections that don’t
    /// close in response to a `versionchange` event, the request will be blocked until they all close.
    pub async fn delete(&self, name: &str) -> Result<(), Error> {
        let request = self.inner.delete(name)?;
        let _: JsValue = wait_request(request).await?;
        Ok(())
    }
}

impl From<SysFactory> for Factory {
    fn from(inner: SysFactory) -> Self {
        Self { inner }
    }
}

impl From<Factory> for SysFactory {
    fn from(factory: Factory) -> Self {
        factory.inner
    }
}

impl TryFrom<JsValue> for Factory {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let inner = value.try_into()?;
        Ok(Self { inner })
    }
}

impl From<Factory> for JsValue {
    fn from(value: Factory) -> Self {
        value.inner.into()
    }
}
