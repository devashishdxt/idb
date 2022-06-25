use std::ops::Deref;

use idb_sys::{EventExt, Request, VersionChangeEvent as SysVersionChangeEvent};
use wasm_bindgen::JsValue;

use crate::{Database, Error, Transaction};

/// Event triggered when the database version changes, as the result of an `upgrade_handler` function.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionChangeEvent {
    inner: SysVersionChangeEvent,
}

impl VersionChangeEvent {
    /// Returns previous version of database.
    pub fn old_version(&self) -> Result<u32, Error> {
        self.inner.old_version().map_err(Into::into)
    }

    /// Returns new version of database.
    pub fn new_version(&self) -> Result<Option<u32>, Error> {
        self.inner.new_version().map_err(Into::into)
    }

    /// Returns the database that triggered the event.
    pub fn database(&self) -> Result<Database, Error> {
        self.inner
            .request()?
            .database()
            .map(Into::into)
            .map_err(Into::into)
    }

    /// Returns the transaction that the event was triggered within.
    pub fn transaction(&self) -> Result<Option<Transaction>, Error> {
        let reqeust = self.inner.request()?;
        Ok(reqeust.transaction().map(Into::into))
    }
}

impl Deref for VersionChangeEvent {
    type Target = SysVersionChangeEvent;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<SysVersionChangeEvent> for VersionChangeEvent {
    fn from(inner: SysVersionChangeEvent) -> Self {
        Self { inner }
    }
}

impl From<VersionChangeEvent> for SysVersionChangeEvent {
    fn from(event: VersionChangeEvent) -> Self {
        event.inner
    }
}

impl TryFrom<JsValue> for VersionChangeEvent {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let inner = value.try_into()?;
        Ok(Self { inner })
    }
}

impl From<VersionChangeEvent> for JsValue {
    fn from(value: VersionChangeEvent) -> Self {
        value.inner.into()
    }
}
