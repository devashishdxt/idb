use crate::{Error, IndexParams, KeyPath, ObjectStore};

/// Builder for object store indexes.
#[derive(Debug)]
pub struct IndexBuilder {
    name: String,
    key_path: KeyPath,
    unique: Option<bool>,
    multi_entry: Option<bool>,
}

impl IndexBuilder {
    /// Creates a new instance of [`IndexBuilder`].
    pub fn new(name: String, key_path: KeyPath) -> Self {
        Self {
            name,
            key_path,
            unique: None,
            multi_entry: None,
        }
    }

    /// Returns the name of the index.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the `unique` flag.
    pub fn unique(mut self, unique: bool) -> Self {
        self.unique = Some(unique);
        self
    }

    /// Sets the `multi_entry` flag.
    pub fn multi_entry(mut self, multi_entry: bool) -> Self {
        self.multi_entry = Some(multi_entry);
        self
    }

    /// Applies the index to the given object store.
    pub(crate) fn apply(self, object_store: &ObjectStore) -> Result<(), Error> {
        let mut params = IndexParams::new();

        if let Some(unique) = self.unique {
            params.unique(unique);
        }

        if let Some(multi_entry) = self.multi_entry {
            params.multi_entry(multi_entry);
        }

        object_store.create_index(&self.name, self.key_path, Some(params))?;

        Ok(())
    }
}
