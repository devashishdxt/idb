use std::ops::Deref;

use js_sys::Object;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{DomException, Event, IdbOpenDbRequest, IdbVersionChangeEvent};

use crate::{Error, Request, RequestReadyState, Transaction, VersionChangeEvent};

/// Request returned by [`Factory`](crate::Factory) when opening or deleting a database.
#[derive(Debug)]
pub struct DatabaseRequest {
    inner: IdbOpenDbRequest,
    success_callback: Option<Closure<dyn FnMut(Event)>>,
    error_callback: Option<Closure<dyn FnMut(Event)>>,
    blocked_callback: Option<Closure<dyn FnMut(IdbVersionChangeEvent)>>,
    upgrade_needed_callback: Option<Closure<dyn FnMut(IdbVersionChangeEvent)>>,
}

impl DatabaseRequest {
    /// Adds an event handler for `blocked` event.
    pub fn on_blocked<F>(&mut self, callback: F)
    where
        F: FnOnce(VersionChangeEvent) + 'static,
    {
        let f = move |event: IdbVersionChangeEvent| {
            let event = VersionChangeEvent::from(event);
            callback(event);
        };

        let closure = Closure::once(f);

        self.inner
            .set_onblocked(Some(closure.as_ref().unchecked_ref()));
        self.blocked_callback = Some(closure);
    }

    /// Adds an event handler for `upgradeneeded` event.
    pub fn on_upgrade_needed<F>(&mut self, callback: F)
    where
        F: FnOnce(VersionChangeEvent) + 'static,
    {
        let f = move |event: IdbVersionChangeEvent| {
            let event = VersionChangeEvent::from(event);
            callback(event);
        };

        let closure = Closure::once(f);

        self.inner
            .set_onupgradeneeded(Some(closure.as_ref().unchecked_ref()));
        self.upgrade_needed_callback = Some(closure);
    }
}

impl Request for DatabaseRequest {
    fn result(&self) -> Result<JsValue, Error> {
        self.inner.result().map_err(Error::RequestResultNotFound)
    }

    fn error(&self) -> Result<Option<DomException>, Error> {
        self.inner.error().map_err(Error::RequestErrorNotFound)
    }

    fn source(&self) -> Result<Object, Error> {
        self.inner.source().ok_or(Error::RequestSourceNotFound)
    }

    fn transaction(&self) -> Option<Transaction> {
        self.inner.transaction().map(Into::into)
    }

    fn ready_state(&self) -> Result<RequestReadyState, Error> {
        self.inner.ready_state().try_into()
    }

    fn on_success<F>(&mut self, callback: F)
    where
        F: FnOnce(Event) + 'static,
    {
        let closure = Closure::once(callback);
        self.inner
            .set_onsuccess(Some(closure.as_ref().unchecked_ref()));
        self.success_callback = Some(closure);
    }

    fn on_error<F>(&mut self, callback: F)
    where
        F: FnOnce(Event) + 'static,
    {
        let closure = Closure::once(callback);
        self.inner
            .set_onerror(Some(closure.as_ref().unchecked_ref()));
        self.error_callback = Some(closure);
    }
}

impl Deref for DatabaseRequest {
    type Target = IdbOpenDbRequest;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<IdbOpenDbRequest> for DatabaseRequest {
    fn from(inner: IdbOpenDbRequest) -> Self {
        Self {
            inner,
            success_callback: None,
            error_callback: None,
            blocked_callback: None,
            upgrade_needed_callback: None,
        }
    }
}

impl From<DatabaseRequest> for IdbOpenDbRequest {
    fn from(request: DatabaseRequest) -> Self {
        request.inner
    }
}

impl From<JsValue> for DatabaseRequest {
    fn from(value: JsValue) -> Self {
        let inner: IdbOpenDbRequest = value.into();
        inner.into()
    }
}

impl From<DatabaseRequest> for JsValue {
    fn from(value: DatabaseRequest) -> Self {
        value.inner.into()
    }
}
