use std::ops::Deref;

use js_sys::Object;
use wasm_bindgen::JsValue;
use web_sys::IdbCursor;

use crate::{CursorDirection, Error, StoreRequest};

/// Represents a key cursor for traversing or iterating over multiple records (only keys) in a database.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyCursor {
    inner: IdbCursor,
}

impl KeyCursor {
    /// Returns the [`ObjectStore`](crate::ObjectStore) or [`Index`](crate::Index) the cursor was opened from.
    // TODO: make return type as enum: (IDBObjectStore or IDBIndex)
    pub fn source(&self) -> Object {
        self.inner.source()
    }

    /// Returns the direction of the cursor.
    pub fn direction(&self) -> Result<CursorDirection, Error> {
        self.inner.direction().try_into()
    }

    /// Returns the key of the cursor. Returns an [`Error`] if the cursor is advancing or is finished.
    pub fn key(&self) -> Result<JsValue, Error> {
        self.inner.key().map_err(Error::CursorKeyNotFound)
    }

    /// Returns the effective key of the cursor. Returns an [`Error`] if the cursor is advancing or is finished.
    pub fn primary_key(&self) -> Result<JsValue, Error> {
        self.inner
            .primary_key()
            .map_err(Error::CursorPrimaryKeyNotFound)
    }

    /// Returns the [`StoreRequest`] that was used to obtain this cursor.
    pub fn request(&self) -> StoreRequest {
        self.inner.request().into()
    }

    /// Advances the cursor through the next count records in range.
    pub fn advance(&self, count: u32) -> Result<(), Error> {
        self.inner
            .advance(count)
            .map_err(Error::CursorAdvanceFailed)
    }

    /// Advances the cursor to the next record in range matching or after key (if provided).
    pub fn next(&self, key: Option<&JsValue>) -> Result<(), Error> {
        match key {
            None => self.inner.continue_().map_err(Error::CursorContinueFailed),
            Some(key) => self
                .inner
                .continue_with_key(key)
                .map_err(Error::CursorContinueFailed),
        }
    }

    /// Advances the cursor to the next record in range matching or after key and primary key. Returns an [`Error`] if
    /// the source is not an [`Index`](crate::Index).
    pub fn next_primary_key(&self, key: &JsValue, primary_key: &JsValue) -> Result<(), Error> {
        self.inner
            .continue_primary_key(key, primary_key)
            .map_err(Error::CursorContinueFailed)
    }

    /// Updated the record pointed at by the cursor with a new value.
    pub fn update(&self, value: &JsValue) -> Result<StoreRequest, Error> {
        self.inner
            .update(value)
            .map(Into::into)
            .map_err(Error::UpdateFailed)
    }

    /// Delete the record pointed at by the cursor with a new value.
    pub fn delete(&self) -> Result<StoreRequest, Error> {
        self.inner
            .delete()
            .map(Into::into)
            .map_err(Error::DeleteFailed)
    }
}

impl Deref for KeyCursor {
    type Target = IdbCursor;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<IdbCursor> for KeyCursor {
    fn from(inner: IdbCursor) -> Self {
        Self { inner }
    }
}

impl From<KeyCursor> for IdbCursor {
    fn from(cursor: KeyCursor) -> Self {
        cursor.inner
    }
}

impl From<JsValue> for KeyCursor {
    fn from(value: JsValue) -> Self {
        let inner = value.into();
        Self { inner }
    }
}

impl From<KeyCursor> for JsValue {
    fn from(cursor: KeyCursor) -> Self {
        cursor.inner.into()
    }
}
