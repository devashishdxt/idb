use std::collections::HashSet;

use crate::{Database, DatabaseEvent, Error, Event, Factory};

use super::ObjectStoreBuilder;

/// Builder for databases.
#[derive(Debug)]
pub struct DatabaseBuilder {
    name: String,
    version: Option<u32>,
    object_stores: Vec<ObjectStoreBuilder>,
}

impl DatabaseBuilder {
    /// Creates a new instance of [`DatabaseBuilder`].
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            version: None,
            object_stores: Vec::new(),
        }
    }

    /// Sets the version of the database.
    pub fn version(mut self, version: u32) -> Self {
        self.version = Some(version);
        self
    }

    /// Adds an object store.
    pub fn add_object_store(mut self, object_store: ObjectStoreBuilder) -> Self {
        self.object_stores.push(object_store);
        self
    }

    /// Builds the database.
    pub async fn build(self) -> Result<Database, Error> {
        let factory = Factory::new()?;
        let mut request = factory.open(&self.name, self.version)?;

        request.on_upgrade_needed(move |event| {
            let request = event.target().expect("open database request");

            let mut store_names: HashSet<_> = self
                .object_stores
                .iter()
                .map(|store| store.name().to_owned())
                .collect();

            let database = event.database().expect("database");

            for object_store in self.object_stores {
                object_store
                    .apply(&database, &request)
                    .expect("object store creation");
            }

            let db_store_names = database.store_names();
            let mut stores_to_remove = Vec::new();

            for db_store_name in db_store_names {
                if !store_names.contains(&db_store_name) {
                    store_names.remove(&db_store_name);
                } else {
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
