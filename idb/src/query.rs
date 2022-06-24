use idb_sys::Query as SysQuery;
use wasm_bindgen::JsValue;

use crate::KeyRange;

/// Specifies a query when fetching data from object store
#[derive(Debug, Clone, PartialEq)]
pub enum Query {
    /// Denotes a single key
    Key(JsValue),
    /// Denotes a key range
    KeyRange(KeyRange),
}

impl From<JsValue> for Query {
    fn from(value: JsValue) -> Self {
        Query::Key(value)
    }
}

impl From<KeyRange> for Query {
    fn from(value: KeyRange) -> Self {
        Query::KeyRange(value)
    }
}

impl From<Query> for SysQuery {
    fn from(query: Query) -> Self {
        match query {
            Query::Key(value) => SysQuery::Key(value),
            Query::KeyRange(value) => SysQuery::KeyRange(value.into()),
        }
    }
}

impl From<SysQuery> for Query {
    fn from(query: SysQuery) -> Self {
        match query {
            SysQuery::Key(value) => Query::Key(value),
            SysQuery::KeyRange(value) => Query::KeyRange(value.into()),
        }
    }
}

impl From<Query> for JsValue {
    fn from(value: Query) -> Self {
        match value {
            Query::Key(value) => value,
            Query::KeyRange(value) => value.into(),
        }
    }
}
