use std::{
    future::{Future, IntoFuture},
    pin::Pin,
    task::{Context, Poll},
};

use futures::channel::oneshot;
use idb_sys::{DatabaseRequest, Request};
use wasm_bindgen::JsValue;
use web_sys::EventTarget;

use crate::{
    utils::{error_callback, success_callback},
    Database, Error, VersionChangeEvent,
};

/// Request returned by [`Factory`](crate::Factory) when opening a database.
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

pub struct OpenRequestFuture {
    _inner: DatabaseRequest,
    error_receiver: oneshot::Receiver<<Self as Future>::Output>,
    success_receiver: oneshot::Receiver<<Self as Future>::Output>,
}

impl Future for OpenRequestFuture {
    type Output = Result<Database, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        if let Poll::Ready(result) = Pin::new(&mut this.error_receiver).poll(cx) {
            return match result {
                Ok(result) => Poll::Ready(result),
                Err(_) => Poll::Ready(Err(Error::OneshotChannelReceiveError)),
            };
        }

        if let Poll::Ready(result) = Pin::new(&mut this.success_receiver).poll(cx) {
            return match result {
                Ok(result) => Poll::Ready(result),
                Err(_) => Poll::Ready(Err(Error::OneshotChannelReceiveError)),
            };
        }

        Poll::Pending
    }
}

impl IntoFuture for OpenRequest {
    type Output = <OpenRequestFuture as Future>::Output;

    type IntoFuture = OpenRequestFuture;

    fn into_future(mut self) -> Self::IntoFuture {
        let (error_sender, error_receiver) = oneshot::channel::<Self::Output>();
        let (success_sender, success_receiver) = oneshot::channel::<Self::Output>();

        self.inner.on_error(move |event| {
            let res = error_callback(event);
            let _ = error_sender.send(res);
        });
        self.inner.on_success(move |event| {
            let res = success_callback(event);
            let _ = success_sender.send(res);
        });

        OpenRequestFuture {
            _inner: self.inner,
            error_receiver,
            success_receiver,
        }
    }
}
