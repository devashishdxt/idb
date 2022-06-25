use std::ops::Deref;

use idb_sys::Cursor as SysCursor;
use js_sys::Object;
use wasm_bindgen::JsValue;

use crate::{utils::wait_request, CursorDirection, Error};

/// Represents a cursor for traversing or iterating over multiple records in a database.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cursor {
    inner: SysCursor,
}

impl Cursor {
    /// Returns the [`ObjectStore`](crate::ObjectStore) or [`Index`](crate::Index) the cursor was opened from.
    // TODO: make return type as enum: (IDBObjectStore or IDBIndex)
    pub fn source(&self) -> Object {
        self.inner.source()
    }

    /// Returns the direction of the cursor.
    pub fn direction(&self) -> Result<CursorDirection, Error> {
        self.inner.direction().map_err(Into::into)
    }

    /// Returns the key of the cursor. Returns an [`Error`] if the cursor is advancing or is finished.
    pub fn key(&self) -> Result<JsValue, Error> {
        self.inner.key().map_err(Into::into)
    }

    /// Returns the effective key of the cursor. Returns an [`Error`] if the cursor is advancing or is finished.
    pub fn primary_key(&self) -> Result<JsValue, Error> {
        self.inner.primary_key().map_err(Into::into)
    }

    /// Returns the cursor's current value. Returns an [`Error`] if the cursor is advancing or is finished.
    pub fn value(&self) -> Result<JsValue, Error> {
        self.inner.value().map_err(Into::into)
    }

    /// Advances the cursor through the next count records in range.
    pub fn advance(&self, count: u32) -> Result<(), Error> {
        self.inner.advance(count).map_err(Into::into)
    }

    /// Advances the cursor to the next record in range matching or after key (if provided).
    pub fn next(&self, key: Option<&JsValue>) -> Result<(), Error> {
        self.inner.next(key).map_err(Into::into)
    }

    /// Advances the cursor to the next record in range matching or after key and primary key. Returns an [`Error`] if
    /// the source is not an [`Index`](crate::Index).
    pub fn next_primary_key(&self, key: &JsValue, primary_key: &JsValue) -> Result<(), Error> {
        self.inner
            .next_primary_key(key, primary_key)
            .map_err(Into::into)
    }

    /// Updated the record pointed at by the cursor with a new value.
    pub async fn update(&self, value: &JsValue) -> Result<JsValue, Error> {
        let request = self.inner.update(value)?;
        wait_request(request).await
    }

    /// Delete the record pointed at by the cursor with a new value.
    pub async fn delete(&self) -> Result<(), Error> {
        let request = self.inner.delete()?;
        let _: JsValue = wait_request(request).await?;
        Ok(())
    }
}

impl Deref for Cursor {
    type Target = SysCursor;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<SysCursor> for Cursor {
    fn from(inner: SysCursor) -> Self {
        Self { inner }
    }
}

impl From<Cursor> for SysCursor {
    fn from(cursor: Cursor) -> Self {
        cursor.inner
    }
}

impl TryFrom<JsValue> for Cursor {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let inner = value.try_into()?;
        Ok(Self { inner })
    }
}

impl From<Cursor> for JsValue {
    fn from(cursor: Cursor) -> Self {
        cursor.inner.into()
    }
}
