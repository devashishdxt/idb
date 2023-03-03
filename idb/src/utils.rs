use idb_sys::{Request, StoreRequest, Transaction as SysTransaction};
use js_sys::Array;
use tokio::{select, sync::oneshot};
use wasm_bindgen::JsValue;
use web_sys::Event;

use crate::{Error, Transaction};

pub async fn wait_request<T, E>(mut request: impl Request) -> Result<T, Error>
where
    T: TryFrom<JsValue, Error = E> + 'static,
    E: Into<Error>,
{
    let (error_sender, error_receiver) = oneshot::channel::<Result<T, Error>>();
    let (success_sender, success_receiver) = oneshot::channel::<Result<T, Error>>();

    request.on_error(move |event| {
        let res = error_callback(event);
        let _ = error_sender.send(res);
    });
    request.on_success(move |event| {
        let res = success_callback(event);
        let _ = success_sender.send(res);
    });

    select! {
        res = error_receiver => res.map_err(|_| Error::OneshotChannelReceiveError)?,
        res = success_receiver => res.map_err(|_| Error::OneshotChannelReceiveError)?,
    }
}

pub async fn wait_transaction(transaction: &mut Transaction) -> Result<(), Error> {
    let (error_sender, error_receiver) = oneshot::channel::<Result<(), Error>>();
    let (success_sender, success_receiver) = oneshot::channel::<Result<(), Error>>();

    transaction.inner.on_error(move |event| {
        let res = transaction_error_callback(event);
        let _ = error_sender.send(res);
    });
    transaction.inner.on_complete(move |_| {
        let _ = success_sender.send(Ok(()));
    });

    select! {
        res = error_receiver => res.map_err(|_| Error::OneshotChannelReceiveError)?,
        res = success_receiver => res.map_err(|_| Error::OneshotChannelReceiveError)?,
    }
}

pub async fn wait_transaction_commit(transaction: &mut Transaction) -> Result<(), Error> {
    let (error_sender, error_receiver) = oneshot::channel::<Result<(), Error>>();
    let (success_sender, success_receiver) = oneshot::channel::<Result<(), Error>>();

    transaction.inner.on_error(move |event| {
        let res = transaction_error_callback(event);
        let _ = error_sender.send(res);
    });
    transaction.inner.on_complete(move |_| {
        let _ = success_sender.send(Ok(()));
    });

    transaction.inner.commit()?;

    select! {
        res = error_receiver => res.map_err(|_| Error::OneshotChannelReceiveError)?,
        res = success_receiver => res.map_err(|_| Error::OneshotChannelReceiveError)?,
    }
}

pub async fn wait_transaction_abort(transaction: &mut Transaction) -> Result<(), Error> {
    let (sender, receiver) = oneshot::channel::<()>();

    transaction.inner.on_abort(move |_| {
        let _ = sender.send(());
    });

    transaction.inner.abort()?;

    receiver
        .await
        .map_err(|_| Error::OneshotChannelReceiveError)
}

pub fn success_callback<T, E>(event: Event) -> Result<T, Error>
where
    T: TryFrom<JsValue, Error = E>,
    E: Into<Error>,
{
    let request = StoreRequest::try_from(event.target().ok_or(Error::EventTargetNotFound)?)?;

    let js_value = request.result()?;

    TryInto::try_into(js_value).map_err(Into::into)
}

pub fn error_callback<T>(event: Event) -> Result<T, Error> {
    let request = StoreRequest::try_from(event.target().ok_or(Error::EventTargetNotFound)?)?;

    let error = request.error()?;

    match error {
        None => Err(Error::DomExceptionNotFound),
        Some(error) => Err(Error::DomException(error)),
    }
}

fn transaction_error_callback<T>(event: Event) -> Result<T, Error> {
    let transaction = SysTransaction::try_from(event.target().ok_or(Error::EventTargetNotFound)?)?;

    let error = transaction.error();

    match error {
        None => Err(Error::DomExceptionNotFound),
        Some(error) => Err(Error::DomException(error)),
    }
}

pub fn array_to_vec(array: Array) -> Vec<JsValue> {
    let mut vec = Vec::new();
    for i in 0..array.length() {
        vec.push(array.get(i));
    }
    vec
}
