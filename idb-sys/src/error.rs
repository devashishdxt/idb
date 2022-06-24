use js_sys::Error as JsError;
use thiserror::Error;
use wasm_bindgen::JsValue;

/// Error type for [`idb-sys`](crate) crate.
#[derive(Debug, Error)]
pub enum Error {
    /// Failed to add a value
    #[error("failed to add a value: {}", js_error_display(.0))]
    AddFailed(JsValue),

    /// Failed to clear object store
    #[error("failed to clear object store: {}", js_error_display(.0))]
    ClearFailed(JsValue),

    /// Failed to get count of records
    #[error("failed to get count of records: {}", js_error_display(.0))]
    CountFailed(JsValue),

    /// Failed to advance cursor
    #[error("failed to advance cursor: {}", js_error_display(.0))]
    CursorAdvanceFailed(JsValue),

    /// Failed to continue cursor
    #[error("failed to continue cursor: {}", js_error_display(.0))]
    CursorContinueFailed(JsValue),

    /// Failed to get cursor key
    #[error("failed to get cursor key: {}", js_error_display(.0))]
    CursorKeyNotFound(JsValue),

    /// Failed to get cursor primary key
    #[error("failed to get cursor primary key: {}", js_error_display(.0))]
    CursorPrimaryKeyNotFound(JsValue),

    /// Failed to get cursor value
    #[error("failed to get cursor value: {}", js_error_display(.0))]
    CursorValueNotFound(JsValue),

    /// Failed to delete a value
    #[error("failed to delete a value: {}", js_error_display(.0))]
    DeleteFailed(JsValue),

    /// Failed to get all values
    #[error("failed to get all values: {}", js_error_display(.0))]
    GetAllFailed(JsValue),

    /// Failed to get all keys
    #[error("failed to get all keys: {}", js_error_display(.0))]
    GetAllKeysFailed(JsValue),

    /// Failed to get a value
    #[error("failed to get a value: {}", js_error_display(.0))]
    GetFailed(JsValue),

    /// Failed to get a key
    #[error("failed to get a key: {}", js_error_display(.0))]
    GetKeyFailed(JsValue),

    /// Failed to create new index
    #[error("failed to create new index: {}", js_error_display(.0))]
    IndexCreateFailed(JsValue),

    /// Failed to delete index
    #[error("failed to delete index: {}", js_error_display(.0))]
    IndexDeleteFailed(JsValue),

    /// Failed to delete indexed db
    #[error("failed to delete indexed db: {}", js_error_display(.0))]
    IndexedDbDeleteFailed(JsValue),

    /// Indexed db not found
    #[error("indexed db not found")]
    IndexedDbNotFound(JsValue),

    /// Failed to open indexed db
    #[error("failed to open indexed db: {}", js_error_display(.0))]
    IndexedDbOpenFailed(JsValue),

    /// Failed to get index
    #[error("failed to get index: {}", js_error_display(.0))]
    IndexNotFound(JsValue),

    /// Invalid cursor direction
    #[error("invalid cursor direction")]
    InvalidCursorDirection,

    /// Invalid key path of an object store
    #[error("invalid key path of an object store")]
    InvalidKeyPath,

    /// Invalid request ready state
    #[error("invalid request ready state")]
    InvalidReqeustReadyState,

    /// Invalid storage type
    #[error("invalid storage type")]
    InvalidStorageType,

    /// Invalid transaction mode
    #[error("invalid transaction mode")]
    InvalidTransactionMode,

    /// Failed to get key path of an object store
    #[error("failed to get key path of an object store: {}", js_error_display(.0))]
    KeyPathNotFound(JsValue),

    /// Failed to get key range bound
    #[error("failed to get key range bound: {}", js_error_display(.0))]
    KeyRangeBoundNotFound(JsValue),

    /// Failed to create key range
    #[error("failed to create key range: {}", js_error_display(.0))]
    KeyRangeCreateFailed(JsValue),

    /// Failed to check if a value is included in key range
    #[error("failed to check if a value is included in key range: {}", js_error_display(.0))]
    KeyRangeIncludesFailed(JsValue),

    /// Number conversion error
    #[error("number conversion error")]
    NumberConversionError,

    /// Failed to convert a js value to a js object
    #[error("failed to convert a js value to a js object: {}", js_error_display(.0))]
    ObjectConversionError(JsValue),

    /// Failed to create new object store
    #[error("failed to create new object store: {}", js_error_display(.0))]
    ObjectStoreCreateFailed(JsValue),

    /// Failed to delete object store
    #[error("failed to delete object store: {}", js_error_display(.0))]
    ObjectStoreDeleteFailed(JsValue),

    /// Failed to get object store
    #[error("failed to get object store: {}", js_error_display(.0))]
    ObjectStoreNotFound(JsValue),

    /// Failed to open cursor
    #[error("failed to open cursor: {}", js_error_display(.0))]
    OpenCursorFailed(JsValue),

    /// Failed to open key cursor
    #[error("failed to open key cursor: {}", js_error_display(.0))]
    OpenKeyCursorFailed(JsValue),

    /// Failed to get request error
    #[error("failed to get request error: {}", js_error_display(.0))]
    RequestErrorNotFound(JsValue),

    /// Failed to get request result
    #[error("failed to get request source: {}", js_error_display(.0))]
    RequestResultNotFound(JsValue),

    /// Failed to get request result
    #[error("failed to get request source")]
    RequestSourceNotFound,

    /// Failed to abort transaction
    #[error("failed to abort transaction: {}", js_error_display(.0))]
    TransactionAbortError(JsValue),

    /// Failed to commit transaction
    #[error("failed to commit transaction: {}", js_error_display(.0))]
    TransactionCommitError(JsValue),

    /// Failed to get transaction mode
    #[error("failed to get transaction mode: {}", js_error_display(.0))]
    TransactionModeNotFound(JsValue),

    /// Failed to open new transaction
    #[error("failed to open new transaction: {}", js_error_display(.0))]
    TransactionOpenFailed(JsValue),

    /// Failed to update a value
    #[error("failed to update a value: {}", js_error_display(.0))]
    UpdateFailed(JsValue),
}

fn js_error_display(option: &JsValue) -> String {
    ToString::to_string(&JsError::from(option.clone()).to_string())
}
