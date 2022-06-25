#![deny(missing_docs, unsafe_code)]
#![cfg_attr(feature = "future", into_future)]
//! A callback based crate for interacting with IndexedDB on browsers using webassembly.
mod cursor;
mod database;
mod error;
mod event;
mod factory;
mod index;
mod key_range;
mod object_store;
mod query;
mod request;
mod transaction;
mod utils;

pub use self::{
    cursor::{Cursor, CursorDirection, KeyCursor},
    database::Database,
    error::Error,
    event::{FromEventTarget, VersionChangeEvent},
    factory::Factory,
    index::{Index, IndexParams},
    key_range::KeyRange,
    object_store::{KeyPath, ObjectStore, ObjectStoreParams},
    query::Query,
    request::{DatabaseRequest, Request, RequestReadyState, StoreRequest},
    transaction::{Transaction, TransactionMode},
};
