use std::ops::Deref;

use idb_sys::Database as SysDatabase;
use wasm_bindgen::JsValue;
use web_sys::{Event, EventTarget};

use crate::{Error, ObjectStore, ObjectStoreParams, Transaction, TransactionMode};

/// [`Database`] provides a connection to a database; you can use an [`Database`] object to open a transaction on your
/// database then create, manipulate, and delete objects (data) in that database. The object provides the only way to
/// get and manage versions of the database.
#[derive(Debug)]
pub struct Database {
    pub(crate) inner: SysDatabase,
}

impl Database {
    /// Returns the name of the database.
    pub fn name(&self) -> String {
        self.inner.name()
    }

    /// Returns the version of the database.
    pub fn version(&self) -> Result<u32, Error> {
        self.inner.version().map_err(Into::into)
    }

    /// Returns a list of the names of [`ObjectStore`]s in the database.
    pub fn store_names(&self) -> Vec<String> {
        self.inner.store_names()
    }

    /// Returns a new transaction with the given scope (which can be a single object store name or an array of names),
    /// mode ([`TransactionMode::ReadOnly`] or [`TransactionMode::ReadWrite`]).
    pub fn transaction<T>(
        &self,
        store_names: &[T],
        mode: TransactionMode,
    ) -> Result<Transaction, Error>
    where
        T: AsRef<str>,
    {
        self.inner
            .transaction(store_names, mode)
            .map(Into::into)
            .map_err(Into::into)
    }

    /// Closes the connection once all running transactions have finished.
    pub fn close(&self) {
        self.inner.close()
    }

    /// Creates a new object store with the given name and options and returns a new [`ObjectStore`]. Returns an
    /// [`Error`] if not called within an upgrade transaction.
    pub fn create_object_store(
        &self,
        name: &str,
        params: &ObjectStoreParams,
    ) -> Result<ObjectStore, Error> {
        self.inner
            .create_object_store(name, params)
            .map(Into::into)
            .map_err(Into::into)
    }

    /// Deletes the object store with the given name. Returns an [`Error`] if not called within an upgrade transaction.
    pub fn delete_object_store(&self, name: &str) -> Result<(), Error> {
        self.inner.delete_object_store(name).map_err(Into::into)
    }

    /// Adds an event handler for `close` event.
    pub fn on_close<F>(&mut self, callback: F)
    where
        F: FnOnce(Self) + 'static,
    {
        self.inner.on_close(move |event| {
            let database = get_database_from_event(event).expect("database");
            callback(database);
        })
    }

    /// Adds an event handler for `versionchange` event.
    pub fn on_version_change<F>(&mut self, callback: F)
    where
        F: FnOnce(Database) + 'static,
    {
        self.inner.on_version_change(move |event| {
            let database = get_database_from_event(event).expect("database");
            callback(database);
        })
    }
}

impl TryFrom<EventTarget> for Database {
    type Error = Error;

    fn try_from(target: EventTarget) -> Result<Self, Self::Error> {
        let inner = target.try_into()?;
        Ok(Self { inner })
    }
}

impl Deref for Database {
    type Target = SysDatabase;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<SysDatabase> for Database {
    fn from(inner: SysDatabase) -> Self {
        Self { inner }
    }
}

impl From<Database> for SysDatabase {
    fn from(database: Database) -> Self {
        database.inner
    }
}

impl TryFrom<JsValue> for Database {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let inner = value.try_into()?;
        Ok(Self { inner })
    }
}

impl From<Database> for JsValue {
    fn from(value: Database) -> Self {
        value.inner.into()
    }
}

fn get_database_from_event(event: Event) -> Result<Database, Error> {
    let target = event.target().ok_or(Error::EventTargetNotFound)?;
    Database::try_from(target).map_err(Into::into)
}
