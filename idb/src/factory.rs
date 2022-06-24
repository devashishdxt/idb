use std::ops::Deref;

use idb_sys::Factory as SysFactory;
use wasm_bindgen::JsValue;

use crate::{utils::wait_request, Database, Error, VersionChangeEvent};

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
    pub async fn open<F>(
        &self,
        name: &str,
        version: u32,
        upgrade_handler: F,
    ) -> Result<Database, Error>
    where
        F: FnOnce(VersionChangeEvent) + 'static,
    {
        let mut request = self.inner.open(name, version)?;
        request.on_upgrade_needed(|event| upgrade_handler(event.into()));

        wait_request(request).await
    }

    /// Attempts to delete the named database. If the database already exists and there are open connections that don’t
    /// close in response to a `versionchange` event, the request will be blocked until they all close.
    pub async fn delete(&self, name: &str) -> Result<(), Error> {
        let request = self.inner.delete(name)?;
        let _: JsValue = wait_request(request).await?;
        Ok(())
    }
}

impl Deref for Factory {
    type Target = SysFactory;

    fn deref(&self) -> &Self::Target {
        &self.inner
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

impl From<JsValue> for Factory {
    fn from(value: JsValue) -> Self {
        let inner = value.into();
        Self { inner }
    }
}

impl From<Factory> for JsValue {
    fn from(value: Factory) -> Self {
        value.inner.into()
    }
}
