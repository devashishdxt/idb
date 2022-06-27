use idb_sys::DatabaseRequest;
use wasm_bindgen::JsValue;
use web_sys::EventTarget;

use crate::{utils::wait_request, Database, Error, VersionChangeEvent};

#[derive(Debug)]
pub struct OpenRequest {
    inner: DatabaseRequest,
}

impl OpenRequest {
    /// Adds an event handler for `blocked` event.
    pub fn on_blocked<F>(&mut self, callback: F)
    where
        F: FnOnce(VersionChangeEvent) + 'static,
    {
        self.inner.on_blocked(|event| callback(event.into()))
    }

    /// Adds an event handler for `upgradeneeded` event.
    pub fn on_upgrade_needed<F>(&mut self, callback: F)
    where
        F: FnOnce(VersionChangeEvent) + 'static,
    {
        self.inner.on_upgrade_needed(|event| callback(event.into()))
    }

    /// Executes and waits for the database to open
    pub async fn execute(self) -> Result<Database, Error> {
        wait_request(self.inner).await
    }
}

impl TryFrom<EventTarget> for OpenRequest {
    type Error = Error;

    fn try_from(target: EventTarget) -> Result<Self, Self::Error> {
        let inner = target.try_into()?;
        Ok(Self { inner })
    }
}

impl From<DatabaseRequest> for OpenRequest {
    fn from(inner: DatabaseRequest) -> Self {
        Self { inner }
    }
}

impl From<OpenRequest> for DatabaseRequest {
    fn from(open_request: OpenRequest) -> Self {
        open_request.inner
    }
}

impl TryFrom<JsValue> for OpenRequest {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let inner = DatabaseRequest::try_from(value)?;
        Ok(Self { inner })
    }
}

impl From<OpenRequest> for JsValue {
    fn from(open_request: OpenRequest) -> Self {
        open_request.inner.into()
    }
}
