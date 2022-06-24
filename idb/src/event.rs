use std::ops::Deref;

use idb_sys::VersionChangeEvent as SysVersionChangeEvent;
use wasm_bindgen::JsValue;

use crate::Error;

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

impl From<JsValue> for VersionChangeEvent {
    fn from(value: JsValue) -> Self {
        let inner = value.into();
        Self { inner }
    }
}

impl From<VersionChangeEvent> for JsValue {
    fn from(value: VersionChangeEvent) -> Self {
        value.inner.into()
    }
}
