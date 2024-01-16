use std::ops::Deref;

use num_traits::ToPrimitive;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Event, IdbVersionChangeEvent};

use crate::Error;

/// Event triggered when the database version changes, as the result of an [`DatabaseRequest::on_upgrade_needed`](crate::DatabaseRequest::on_upgrade_needed) event
/// handler function.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionChangeEvent {
    inner: IdbVersionChangeEvent,
}

impl VersionChangeEvent {
    /// Returns previous version of database.
    pub fn old_version(&self) -> Result<u32, Error> {
        self.inner
            .old_version()
            .to_u32()
            .ok_or(Error::NumberConversionError)
    }

    /// Returns new version of database.
    pub fn new_version(&self) -> Result<Option<u32>, Error> {
        self.inner
            .new_version()
            .map(|new| new.to_u32().ok_or(Error::NumberConversionError))
            .transpose()
    }
}

impl Deref for VersionChangeEvent {
    type Target = Event;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl From<IdbVersionChangeEvent> for VersionChangeEvent {
    fn from(inner: IdbVersionChangeEvent) -> Self {
        Self { inner }
    }
}

impl From<VersionChangeEvent> for IdbVersionChangeEvent {
    fn from(event: VersionChangeEvent) -> Self {
        event.inner
    }
}

impl TryFrom<JsValue> for VersionChangeEvent {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        value
            .dyn_into::<IdbVersionChangeEvent>()
            .map(Into::into)
            .map_err(|value| Error::UnexpectedJsType("IdbVersionChangeEvent", value))
    }
}

impl From<VersionChangeEvent> for JsValue {
    fn from(value: VersionChangeEvent) -> Self {
        value.inner.into()
    }
}
