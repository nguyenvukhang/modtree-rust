use crate::collection::ModuleCollection;
use fetcher::Loader;
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

    pub async fn import_modules(&self, academic_year: &str) -> Result<()> {
        let module_collection = self.modules();
        module_collection.drop().await?;
        let loader = Loader::new(academic_year)?;
        println!("Loading modules from JSON...");
        let modules = loader.load_all_modules().await?;
        println!("Done loading all modules from JSON");
        println!("Inserting modules to mongo-db...");
        module_collection.insert_many_modules(&modules).await;
        println!("Done.");
        Ok(())
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
