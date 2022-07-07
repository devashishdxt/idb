use idb_sys::KeyCursor as SysKeyCursor;
use js_sys::Object;
use wasm_bindgen::JsValue;

use crate::{utils::wait_request, CursorDirection, Error};

/// Represents a key cursor for traversing or iterating over multiple records (only keys) in a database.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyCursor {
    inner: SysKeyCursor,
    finished: bool,
}

impl KeyCursor {
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
        if self.finished {
            Ok(JsValue::null())
        } else {
            self.inner.key().map_err(Into::into)
        }
    }

    /// Returns the effective key of the cursor. Returns an [`Error`] if the cursor is advancing or is finished.
    pub fn primary_key(&self) -> Result<JsValue, Error> {
        if self.finished {
            Ok(JsValue::null())
        } else {
            self.inner.primary_key().map_err(Into::into)
        }
    }

    /// Advances the cursor through the next count records in range.
    pub async fn advance(&mut self, count: u32) -> Result<(), Error> {
        if self.finished {
            return Err(Error::CursorFinished);
        }

        let request = self.inner.request();

        self.inner.advance(count)?;
        let cursor: JsValue = wait_request(request).await?;

        if cursor.is_null() {
            self.finished = true;
        } else {
            let inner = SysKeyCursor::try_from(cursor)?;
            self.inner = inner;
        }

        Ok(())
    }

    /// Advances the cursor to the next record in range matching or after key (if provided).
    pub async fn next(&mut self, key: Option<&JsValue>) -> Result<(), Error> {
        if self.finished {
            return Err(Error::CursorFinished);
        }

        let request = self.inner.request();

        self.inner.next(key)?;
        let cursor: JsValue = wait_request(request).await?;

        if cursor.is_null() {
            self.finished = true;
        } else {
            let inner = SysKeyCursor::try_from(cursor)?;
            self.inner = inner;
        }

        Ok(())
    }

    /// Advances the cursor to the next record in range matching or after key and primary key. Returns an [`Error`] if
    /// the source is not an [`Index`](crate::Index).
    pub async fn next_primary_key(
        &mut self,
        key: &JsValue,
        primary_key: &JsValue,
    ) -> Result<(), Error> {
        if self.finished {
            return Err(Error::CursorFinished);
        }

        let request = self.inner.request();

        self.inner.next_primary_key(key, primary_key)?;
        let cursor: JsValue = wait_request(request).await?;

        if cursor.is_null() {
            self.finished = true;
        } else {
            let inner = SysKeyCursor::try_from(cursor)?;
            self.inner = inner;
        }

        Ok(())
    }

    /// Updated the record pointed at by the cursor with a new value.
    pub async fn update(&self, value: &JsValue) -> Result<JsValue, Error> {
        if self.finished {
            return Err(Error::CursorFinished);
        }

        let request = self.inner.update(value)?;
        wait_request(request).await
    }

    /// Delete the record pointed at by the cursor with a new value.
    pub async fn delete(&self) -> Result<(), Error> {
        if self.finished {
            return Err(Error::CursorFinished);
        }

        let request = self.inner.delete()?;
        let _: JsValue = wait_request(request).await?;
        Ok(())
    }
}

impl From<SysKeyCursor> for KeyCursor {
    fn from(inner: SysKeyCursor) -> Self {
        Self {
            inner,
            finished: false,
        }
    }
}

impl From<KeyCursor> for SysKeyCursor {
    fn from(cursor: KeyCursor) -> Self {
        cursor.inner
    }
}

impl TryFrom<JsValue> for KeyCursor {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let inner = value.try_into()?;
        Ok(Self {
            inner,
            finished: false,
        })
    }
}

impl From<KeyCursor> for JsValue {
    fn from(cursor: KeyCursor) -> Self {
        cursor.inner.into()
    }
}
