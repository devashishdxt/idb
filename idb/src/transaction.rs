use idb_sys::{Transaction as SysTransaction, TransactionMode};
use wasm_bindgen::JsValue;
use web_sys::{DomException, EventTarget};

use crate::{
    utils::{wait_transaction, wait_transaction_abort, wait_transaction_commit},
    Database, Error, ObjectStore,
};

/// Provides a static, asynchronous transaction on a database. All reading and writing of data is done within
/// transactions.
#[derive(Debug)]
pub struct Transaction {
    pub(crate) inner: SysTransaction,
}

impl Transaction {
    /// Returns a list of the names of object stores in the transaction’s scope. For an upgrade transaction this is all
    /// object stores in the database.
    pub fn store_names(&self) -> Vec<String> {
        self.inner.store_names()
    }

    /// Returns the mode the transaction was created with ("readonly" or "readwrite"), or "versionchange" for an upgrade
    /// transaction.
    pub fn mode(&self) -> Result<TransactionMode, Error> {
        self.inner.mode().map_err(Into::into)
    }

    /// Returns the transaction’s connection.
    pub fn database(&self) -> Database {
        self.inner.database().into()
    }

    /// If the transaction was aborted, returns the error (a `DOMException`) providing the reason.
    pub fn error(&self) -> Option<DomException> {
        self.inner.error()
    }

    /// Returns an [`ObjectStore`] in the transaction's scope.
    pub fn object_store(&self, name: &str) -> Result<ObjectStore, Error> {
        self.inner
            .object_store(name)
            .map(Into::into)
            .map_err(Into::into)
    }

    /// Waits for the transaction to complete.
    pub async fn done(mut self) -> Result<(), Error> {
        wait_transaction(&mut self).await
    }

    /// Attempts to commit the transaction. All pending requests will be allowed to complete, but no new requests will
    /// be accepted. This can be used to force a transaction to quickly finish, without waiting for pending requests to
    /// fire success events before attempting to commit normally.
    pub async fn commit(mut self) -> Result<(), Error> {
        wait_transaction_commit(&mut self).await
    }

    /// Aborts the transaction. All pending requests will fail and all changes made to the database will be reverted.
    pub async fn abort(mut self) -> Result<(), Error> {
        wait_transaction_abort(&mut self).await
    }
}

impl TryFrom<EventTarget> for Transaction {
    type Error = Error;

    fn try_from(target: EventTarget) -> Result<Self, Self::Error> {
        let inner = target.try_into()?;
        Ok(Self { inner })
    }
}

impl From<SysTransaction> for Transaction {
    fn from(inner: SysTransaction) -> Self {
        Self { inner }
    }
}

impl From<Transaction> for SysTransaction {
    fn from(transaction: Transaction) -> Self {
        transaction.inner
    }
}

impl TryFrom<JsValue> for Transaction {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let inner = value.try_into()?;
        Ok(Self { inner })
    }
}

impl From<Transaction> for JsValue {
    fn from(value: Transaction) -> Self {
        value.inner.into()
    }
}
