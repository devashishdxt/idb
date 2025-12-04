use std::collections::HashSet;

use indexmap::IndexMap;

use crate::{Database, DatabaseEvent, Error, Event, Factory};

use super::ObjectStoreBuilder;

/// Builder for databases.
#[derive(Debug)]
pub struct DatabaseBuilder {
    name: String,
    version: Option<u32>,
    object_stores: IndexMap<String, ObjectStoreBuilder>,
}

impl DatabaseBuilder {
    /// Creates a new instance of [`DatabaseBuilder`].
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            version: None,
            object_stores: Default::default(),
        }
    }

    /// Sets the version of the database.
    pub fn version(mut self, version: u32) -> Self {
        self.version = Some(version);
        self
    }

    /// Adds an object store.
    pub fn add_object_store(mut self, object_store: ObjectStoreBuilder) -> Self {
        let name = object_store.name().to_owned();
        let _previous_object_store_for_name = self.object_stores.insert(name, object_store);
        debug_assert!(
            _previous_object_store_for_name.is_none(),
            "we probably don't want to be overwriting object stores at any point"
        );
        self
    }

    /// Builds the database.
    pub async fn build(self) -> Result<Database, Error> {
        let factory = Factory::new()?;
        let mut request = factory.open(&self.name, self.version)?;

        request.on_upgrade_needed(move |event| {
            let request = event.target().expect("open database request");

            let mut store_names = self.object_stores.keys().cloned().collect::<HashSet<_>>();

            let database = event.database().expect("database");

            for object_store in self.object_stores.into_values() {
                object_store
                    .apply(&database, &request)
                    .expect("object store creation");
            }

            let db_store_names = database.store_names();
            let mut stores_to_remove = Vec::new();

            for db_store_name in db_store_names {
                if !store_names.remove(&db_store_name) {
                    stores_to_remove.push(db_store_name);
                }
            }

            for store_name in stores_to_remove {
                database
                    .delete_object_store(&store_name)
                    .expect("object store deletion");
            }
        });

        let mut database = request.await?;

        database.on_version_change(|event| {
            let database = event.database().expect("database");
            database.close();
        });

        Ok(database)
    }
}
