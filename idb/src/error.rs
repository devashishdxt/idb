use std::convert::Infallible;

use js_sys::Object;
use thiserror::Error;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::DomException;

/// Error type for [`idb`](crate) crate.
#[derive(Debug, Error, PartialEq)]
pub enum Error {
    /// Cursor is finished.
    #[error("cursor is finished")]
    CursorFinished,

    /// DOM exception
    #[error("DOM exception: {}", js_object_display(.0))]
    DomException(DomException),

    /// DOM exception not found
    #[error("DOM exception not found")]
    DomExceptionNotFound,

    /// Event target not found
    #[error("event target not found")]
    EventTargetNotFound,

    /// Cannot fail
    #[error("infalliable")]
    Infalliable(#[from] Infallible),

    /// Failed to receive object on oneshot channel
    #[error("failed to receive object on oneshot channel")]
    OneshotChannelReceiveError,

    /// [`idb_sys::Error`]
    #[error(transparent)]
    SysError(#[from] idb_sys::Error),

    /// Unexpected JS type
    #[error("unexpected JS type. expected: {}, found: {}", .0, js_object_display(.1))]
    UnexpectedJsType(&'static str, JsValue),
}

fn js_object_display(option: &JsValue) -> String {
    let object: &Object = option.unchecked_ref();
    ToString::to_string(&object.to_string())
}
