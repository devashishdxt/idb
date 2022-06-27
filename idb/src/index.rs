use idb_sys::{CursorDirection, Index as SysIndex, KeyPath};
use js_sys::Array;
use wasm_bindgen::JsValue;

use crate::{
    utils::{array_to_vec, wait_request},
    Cursor, Error, KeyCursor, ObjectStore, Query,
};

/// Provides asynchronous access to an index in a database.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Index {
    inner: SysIndex,
}

impl Index {
    /// Returns the name of the index.
    pub fn name(&self) -> String {
        self.inner.name()
    }

    /// Updates the name of the index.
    pub fn set_name(&self, name: &str) {
        self.inner.set_name(name)
    }

    /// Returns the [`ObjectStore`] the index belongs to.
    pub fn object_store(&self) -> ObjectStore {
        self.inner.object_store().into()
    }

    /// Returns the key path of the index.
    pub fn key_path(&self) -> Result<Option<KeyPath>, Error> {
        self.inner.key_path().map_err(Into::into)
    }

    /// Returns true if the index’s `multi_entry` flag is true.
    pub fn multi_entry(&self) -> bool {
        self.inner.multi_entry()
    }

    /// Returns true if the index’s `unique` flag is true.
    pub fn unique(&self) -> bool {
        self.inner.unique()
    }

    /// Retrieves the value of the first record matching the given key or key range in query.
    pub async fn get(&self, query: impl Into<Query>) -> Result<JsValue, Error> {
        let request = self.inner.get(query.into())?;
        wait_request(request).await
    }

    /// Retrieves the key of the first record matching the given key or key range in query.
    pub async fn get_key(&self, query: impl Into<Query>) -> Result<JsValue, Error> {
        let request = self.inner.get_key(query.into())?;
        wait_request(request).await
    }

    /// Retrieves the values of the records matching the given key or key range in query (up to limit if given).
    pub async fn get_all(
        &self,
        query: Option<Query>,
        limit: Option<u32>,
    ) -> Result<Vec<JsValue>, Error> {
        let request = self.inner.get_all(query.map(Into::into), limit)?;
        let array = wait_request(request).await?;
        Ok(array_to_vec(array))
    }

    /// Retrieves the keys of records matching the given key or key range in query (up to limit if given).
    pub async fn get_all_keys(
        &self,
        query: Option<Query>,
        limit: Option<u32>,
    ) -> Result<Vec<JsValue>, Error> {
        let request = self.inner.get_all_keys(query.map(Into::into), limit)?;
        let array: Array = wait_request(request).await?;
        Ok(array_to_vec(array))
    }

    /// Retrieves the number of records matching the given key or key range in query.
    pub async fn count(&self, query: Option<Query>) -> Result<u32, Error> {
        let request = self.inner.count(query.map(Into::into))?;
        let value: JsValue = wait_request(request).await?;

        value
            .as_f64()
            .and_then(num_traits::cast)
            .ok_or(Error::UnexpectedJsType("u32", value))
    }

    /// Opens a [`Cursor`](crate::Cursor) over the records matching query, ordered by direction. If query is `None`, all
    /// records in index are matched.
    pub async fn open_cursor(
        &self,
        query: Option<Query>,
        cursor_direction: Option<CursorDirection>,
    ) -> Result<Cursor, Error> {
        let request = self
            .inner
            .open_cursor(query.map(Into::into), cursor_direction)?;
        wait_request(request).await
    }

    /// Opens a [`KeyCursor`](crate::KeyCursor) over the records matching query, ordered by direction. If query is
    /// `None`, all records in index are matched.
    pub async fn open_key_cursor(
        &self,
        query: Option<Query>,
        cursor_direction: Option<CursorDirection>,
    ) -> Result<KeyCursor, Error> {
        let request = self
            .inner
            .open_key_cursor(query.map(Into::into), cursor_direction)?;
        wait_request(request).await
    }
}

impl From<SysIndex> for Index {
    fn from(inner: SysIndex) -> Self {
        Self { inner }
    }
}

impl From<Index> for SysIndex {
    fn from(index: Index) -> Self {
        index.inner
    }
}

impl TryFrom<JsValue> for Index {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let inner = value.try_into()?;
        Ok(Self { inner })
    }
}

impl From<Index> for JsValue {
    fn from(value: Index) -> Self {
        value.inner.into()
    }
}
