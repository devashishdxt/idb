use std::ops::Deref;

use idb_sys::{DatabaseRequest, Request, VersionChangeEvent as SysVersionChangeEvent};
use wasm_bindgen::JsValue;
use web_sys::Event;

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
        let target = self.target().ok_or(Error::EventTargetNotFound)?;
        let request: DatabaseRequest = DatabaseRequest::try_from(target)?;

        request.database().map(Into::into).map_err(Into::into)
    }

    /// Returns the transaction that the event was triggered within.
    pub fn transaction(&self) -> Result<Option<Transaction>, Error> {
        let target = self.target().ok_or(Error::EventTargetNotFound)?;
        let request: DatabaseRequest = DatabaseRequest::try_from(target)?;

        Ok(request.transaction().map(Into::into))
    }
}

impl Deref for VersionChangeEvent {
    type Target = Event;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
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
