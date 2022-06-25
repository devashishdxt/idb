use std::ops::Deref;

use idb_sys::{EventExt, Factory as SysFactory};
use wasm_bindgen::JsValue;
use web_sys::Event;

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
    pub async fn open<U, B, V, C>(
        &self,
        name: &str,
        version: u32,
        upgrade_handler: Option<U>,
        blocked_handler: Option<B>,
        version_change_handler: Option<V>,
        close_handler: Option<C>,
    ) -> Result<Database, Error>
    where
        U: FnOnce(VersionChangeEvent) + 'static,
        B: FnOnce(VersionChangeEvent) + 'static,
        V: FnOnce(Database) + 'static,
        C: FnOnce(Database) + 'static,
    {
        let mut request = self.inner.open(name, version)?;

        if let Some(upgrade_handler) = upgrade_handler {
            request.on_upgrade_needed(|event| upgrade_handler(event.into()));
        }

        if let Some(blocked_handler) = blocked_handler {
            request.on_blocked(|event| blocked_handler(event.into()));
        }

        let mut database: Database = wait_request(request).await?;

        if let Some(version_change_handler) = version_change_handler {
            database.inner.on_version_change(|event| {
                let database = get_database_from_event(event).expect("database");
                version_change_handler(database);
            })
        }

        if let Some(close_handler) = close_handler {
            database.inner.on_close(|event| {
                let database = get_database_from_event(event).expect("database");
                close_handler(database);
            })
        }

        Ok(database)
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

fn get_database_from_event(event: Event) -> Result<Database, Error> {
    event
        .request()?
        .database()
        .map(Into::into)
        .map_err(Into::into)
}
