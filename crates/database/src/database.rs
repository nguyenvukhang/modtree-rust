#![allow(unused)]
use crate::collection::ModuleCollection;
use types::{Module, Result};

/// wrapper for the standard mongo-db database with project-specific tooling
pub struct Database(mongodb::Database);

impl Database {
    pub fn new(db: mongodb::Database) -> Self {
        Self(db)
    }

    pub fn modules(&self) -> ModuleCollection {
        ModuleCollection::new(self.0.collection::<Module>("modules"))
    }

    pub fn name<'a>(&'a self) -> &'a str {
        self.0.name()
    }

    pub async fn first_time_setup(&self) -> Result<()> {
        // list of all collections used.
        let collections = ["modules"];
        for name in collections {
            self.0.create_collection(name, None).await?;
        }
        Ok(())
    }
}
