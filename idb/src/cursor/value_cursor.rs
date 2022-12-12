use idb_sys::Cursor as SysCursor;
use js_sys::Object;
use wasm_bindgen::JsValue;

use crate::{utils::wait_request, CursorDirection, Error};

/// Represents a cursor for traversing or iterating over multiple records in a database.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cursor {
    inner: SysCursor,
    finished: bool,
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

    /// Returns the cursor's current value. Returns an [`Error`] if the cursor is advancing or is finished.
    pub fn value(&self) -> Result<JsValue, Error> {
        if self.finished {
            Ok(JsValue::null())
        } else {
            self.inner.value().map_err(Into::into)
        }
    }

    /// Advances the cursor through the next count records in range.
    pub async fn advance(&mut self, count: u32) -> Result<(), Error> {
        if self.finished {
            return Err(Error::CursorFinished);
        }

        let request = self.inner.request();

        self.inner.advance(count)?;
        let cursor: Option<JsValue> = wait_request(request).await?;

        match cursor {
            None => self.finished = true,
            Some(cursor) => {
                let inner = SysCursor::try_from(cursor)?;
                self.inner = inner;
            }
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
        let cursor: Option<JsValue> = wait_request(request).await?;

        match cursor {
            None => self.finished = true,
            Some(cursor) => {
                let inner = SysCursor::try_from(cursor)?;
                self.inner = inner;
            }
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
        let cursor: Option<JsValue> = wait_request(request).await?;

        match cursor {
            None => self.finished = true,
            Some(cursor) => {
                let inner = SysCursor::try_from(cursor)?;
                self.inner = inner;
            }
        }

        Ok(())
    }

    /// Updated the record pointed at by the cursor with a new value.
    pub async fn update(&self, value: &JsValue) -> Result<JsValue, Error> {
        if self.finished {
            return Err(Error::CursorFinished);
        }

        let request = self.inner.update(value)?;
        wait_request(request).await?.ok_or(Error::UnexpectedJsValue(
            "value after update",
            JsValue::NULL,
        ))
    }

    /// Delete the record pointed at by the cursor with a new value.
    pub async fn delete(&self) -> Result<(), Error> {
        if self.finished {
            return Err(Error::CursorFinished);
        }

        let request = self.inner.delete()?;
        let _: Option<JsValue> = wait_request(request).await?;
        Ok(())
    }
}

impl From<SysCursor> for Cursor {
    fn from(inner: SysCursor) -> Self {
        Self {
            inner,
            finished: false,
        }
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
        Ok(Self {
            inner,
            finished: false,
        })
    }
}

impl From<Cursor> for JsValue {
    fn from(cursor: Cursor) -> Self {
        cursor.inner.into()
    }
}
