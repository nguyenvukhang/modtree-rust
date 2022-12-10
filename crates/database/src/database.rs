use mongodb::Collection;
use types::{Module, Result};

/// wrapper for the standard mongo-db database with project-specific tooling
pub struct Database(mongodb::Database);

impl Database {
    pub fn new(db: mongodb::Database) -> Self {
        Self(db)
    }

    pub fn modules(&self) -> Collection<Module> {
        self.0.collection::<Module>("modules")
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
