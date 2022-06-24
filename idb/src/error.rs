use js_sys::Error as JsError;
use thiserror::Error;
use wasm_bindgen::JsValue;
use web_sys::DomException;

#[derive(Debug, Error)]
pub enum Error {
    /// DOM exception
    #[error("DOM exception: {}", js_error_display(.0))]
    DomException(DomException),

    /// DOM exception not found
    #[error("DOM exception not found")]
    DomExceptionNotFound,

    /// Event target not found
    #[error("event target not found")]
    EventTargetNotFound,

    /// Failed to receive object on oneshot channel
    #[error("failed to receive object on oneshot channel")]
    OneshotChannelReceiveError,

    /// [`idb_sys::Error`]
    #[error(transparent)]
    SysError(#[from] idb_sys::Error),

    /// Unexpected JS type
    #[error("unexpected JS type")]
    UnexpectedJsType,
}

fn js_error_display(option: &JsValue) -> String {
    ToString::to_string(&JsError::from(option.clone()).to_string())
}
