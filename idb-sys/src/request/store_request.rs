use js_sys::Object;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{DomException, Event, EventTarget, IdbRequest};

use crate::{Error, Request, RequestReadyState, Transaction};

/// Request returned when performing operations on an [`ObjectStore`](crate::ObjectStore).
#[derive(Debug)]
pub struct StoreRequest {
    inner: IdbRequest,
    success_callback: Option<Closure<dyn FnMut(Event)>>,
    error_callback: Option<Closure<dyn FnMut(Event)>>,
}

impl Request for StoreRequest {
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

impl TryFrom<EventTarget> for StoreRequest {
    type Error = Error;

    fn try_from(target: EventTarget) -> Result<Self, Self::Error> {
        let target: JsValue = target.into();
        target
            .dyn_into::<IdbRequest>()
            .map(Into::into)
            .map_err(|value| Error::UnexpectedJsType("IdbRequest", value))
    }
}

impl From<IdbRequest> for StoreRequest {
    fn from(inner: IdbRequest) -> Self {
        Self {
            inner,
            success_callback: None,
            error_callback: None,
        }
    }
}

impl From<StoreRequest> for IdbRequest {
    fn from(request: StoreRequest) -> Self {
        request.inner
    }
}

impl TryFrom<JsValue> for StoreRequest {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        value
            .dyn_into::<IdbRequest>()
            .map(Into::into)
            .map_err(|value| Error::UnexpectedJsType("IdbRequest", value))
    }
}

impl From<StoreRequest> for JsValue {
    fn from(value: StoreRequest) -> Self {
        value.inner.into()
    }
}
