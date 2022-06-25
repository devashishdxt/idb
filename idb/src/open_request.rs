use std::ops::Deref;

use idb_sys::DatabaseRequest;
use wasm_bindgen::JsValue;

use crate::{utils::wait_request, Database, Error, VersionChangeEvent};

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

impl Deref for OpenRequest {
    type Target = DatabaseRequest;

    fn deref(&self) -> &Self::Target {
        &self.inner
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
