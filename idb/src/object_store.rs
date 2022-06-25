use std::ops::Deref;

use idb_sys::{KeyPath, ObjectStore as SysObjectStore};
use js_sys::Array;
use wasm_bindgen::JsValue;

use crate::{
    utils::{array_to_vec, wait_request},
    Cursor, CursorDirection, Error, Index, IndexParams, KeyCursor, Query, Transaction,
};

/// Represents an object store in a database.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectStore {
    inner: SysObjectStore,
}

impl ObjectStore {
    /// Returns the name of the store.
    pub fn name(&self) -> String {
        self.inner.name()
    }

    /// Updates the name of the store to newName. Returns and [`Error`] if not called within an upgrade transaction.
    pub fn set_name(&self, name: &str) {
        self.inner.set_name(name)
    }

    /// Returns the key path of the store.
    pub fn key_path(&self) -> Result<Option<KeyPath>, Error> {
        self.inner.key_path().map_err(Into::into)
    }

    /// Returns a list of the names of indexes in the store.
    pub fn index_names(&self) -> Vec<String> {
        self.inner.index_names()
    }

    /// Returns the associated [`Transaction`].
    pub fn transaction(&self) -> Transaction {
        self.inner.transaction().into()
    }

    /// Returns `true` if the store has a key generator, and `false` otherwise.
    pub fn auto_increment(&self) -> bool {
        self.inner.auto_increment()
    }

    /// Adds or updates a record in store with the given value and key.
    pub async fn put(&self, value: &JsValue, key: Option<&JsValue>) -> Result<JsValue, Error> {
        let request = self.inner.put(value, key)?;
        wait_request(request).await
    }

    /// Adds a record in store with the given value and key.
    pub async fn add(&self, value: &JsValue, key: Option<&JsValue>) -> Result<JsValue, Error> {
        let request = self.inner.add(value, key)?;
        wait_request(request).await
    }

    /// Deletes records in store with the given key or in the given key range in query.
    pub async fn delete(&self, query: impl Into<Query>) -> Result<(), Error> {
        let request = self.inner.delete(query.into())?;
        let _: JsValue = wait_request(request).await?;
        Ok(())
    }

    /// Deletes all records in store.
    pub async fn clear(&self) -> Result<(), Error> {
        let request = self.inner.clear()?;
        let _: JsValue = wait_request(request).await?;
        Ok(())
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
        query: Option<impl Into<Query>>,
        limit: Option<u32>,
    ) -> Result<Vec<JsValue>, Error> {
        let request = self.inner.get_all(query.map(Into::into), limit)?;
        let array: Array = wait_request(request).await?;
        Ok(array_to_vec(array))
    }

    /// Retrieves the keys of records matching the given key or key range in query (up to limit if given).
    pub async fn get_all_keys(
        &self,
        query: Option<impl Into<Query>>,
        limit: Option<u32>,
    ) -> Result<Vec<JsValue>, Error> {
        let request = self.inner.get_all_keys(query.map(Into::into), limit)?;
        let array: Array = wait_request(request).await?;
        Ok(array_to_vec(array))
    }

    /// Retrieves the number of records matching the given key or key range in query.
    pub async fn count(&self, query: Option<impl Into<Query>>) -> Result<u32, Error> {
        let request = self.inner.count(query.map(Into::into))?;
        let value: JsValue = wait_request(request).await?;

        value
            .as_f64()
            .and_then(num_traits::cast)
            .ok_or(Error::UnexpectedJsType("u32", value))
    }

    /// Opens a [`Cursor`](crate::Cursor) over the records matching query, ordered by direction. If query is `None`,
    /// all records in store are matched.
    pub async fn open_cursor(
        &self,
        query: Option<impl Into<Query>>,
        cursor_direction: Option<CursorDirection>,
    ) -> Result<Cursor, Error> {
        let request = self
            .inner
            .open_cursor(query.map(Into::into), cursor_direction)?;
        wait_request(request).await
    }

    /// Opens a [`KeyCursor`](crate::KeyCursor) over the records matching query, ordered by direction. If query is
    /// `None`, all records in store are matched.
    pub async fn open_key_cursor(
        &self,
        query: Option<impl Into<Query>>,
        cursor_direction: Option<CursorDirection>,
    ) -> Result<KeyCursor, Error> {
        let request = self
            .inner
            .open_key_cursor(query.map(Into::into), cursor_direction)?;
        wait_request(request).await
    }

    /// Returns an [`Index`] for the index named name in store.
    pub fn index(&self, name: &str) -> Result<Index, Error> {
        self.inner.index(name).map(Into::into).map_err(Into::into)
    }

    /// Creates a new index in store with the given name, key path and options and returns a new [`Index`]. Returns an
    /// [`Error`] if not called within an upgrade transaction.
    pub fn create_index(
        &self,
        name: &str,
        key_path: KeyPath,
        params: Option<IndexParams>,
    ) -> Result<Index, Error> {
        self.inner
            .create_index(name, key_path, params)
            .map(Into::into)
            .map_err(Into::into)
    }

    /// Deletes the index in store with the given name. Returns an [`Error`] if not called within an upgrade
    /// transaction.
    pub fn delete_index(&self, name: &str) -> Result<(), Error> {
        self.inner.delete_index(name).map_err(Into::into)
    }
}

impl Deref for ObjectStore {
    type Target = SysObjectStore;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<SysObjectStore> for ObjectStore {
    fn from(inner: SysObjectStore) -> Self {
        Self { inner }
    }
}

impl From<ObjectStore> for SysObjectStore {
    fn from(object_store: ObjectStore) -> Self {
        object_store.inner
    }
}

impl TryFrom<JsValue> for ObjectStore {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let inner = value.try_into()?;
        Ok(Self { inner })
    }
}

impl From<ObjectStore> for JsValue {
    fn from(value: ObjectStore) -> Self {
        value.inner.into()
    }
}
