use futures::stream::StreamExt;
use mongodb::bson::{doc, to_document};
use mongodb::options::UpdateOptions;
use mongodb::results::UpdateResult;
use std::collections::HashSet;
use types::{error, Error, Module, Result};

#[derive(Debug)]
pub struct ModuleCollection(mongodb::Collection<Module>);

impl ModuleCollection {
    pub fn new(x: mongodb::Collection<Module>) -> Self {
        Self(x)
    }

    pub async fn import_academic_year(
        &self,
        academic_year: &str,
    ) -> Result<()> {
        use fetcher::Loader;
        let loader = Loader::new(academic_year)?;
        println!("Loading modules from JSON...");
        let modules = loader.load_all_modules(None).await?;
        println!("Done loading all modules from JSON");
        println!("Inserting modules to mongo-db...");
        self.insert_many(&modules).await;
        println!("Done.");
        Ok(())
    }

    pub async fn import_partial(
        &self,
        academic_year: &str,
        count: usize,
    ) -> Result<()> {
        use fetcher::Loader;
        let loader = Loader::new(academic_year)?;
        println!("Loading modules from JSON...");
        let modules = loader.load_all_modules(Some(count)).await?;
        println!("Done loading all modules from JSON");
        println!("Inserting modules to mongo-db...");
        self.insert_many(&modules).await;
        println!("Done.");
        Ok(())
    }

    pub async fn import_one(
        &self,
        academic_year: &str,
        module_code: &str,
    ) -> Result<()> {
        use fetcher::Loader;
        let loader = Loader::new(academic_year)?;
        println!("Loading {module_code} from JSON...");
        let module = loader.load_module(module_code).await?;
        println!("Done loading {module_code} from JSON");
        println!("Inserting modules to mongo-db...");
        self.insert_one(&module).await?;
        println!("Done.");
        Ok(())
    }

    pub async fn insert_one(&self, module: &Module) -> Result<UpdateResult> {
        let mut doc = to_document(module)?;
        // let mongo-db automatically generate an id
        doc.remove("_id");
        let query = doc! {
            "module_code": module.code(),
            "acad_year": module.academic_year(),
        };
        let opts = UpdateOptions::builder().upsert(true).build();
        let result =
            self.0.update_one(query, doc! { "$set": doc }, opts).await?;
        Ok(result)
    }

    pub async fn insert_many(
        &self,
        modules: &Vec<Module>,
    ) -> Vec<Result<UpdateResult>> {
        let handles = modules
            .iter()
            .map(|module| async move { self.insert_one(module).await });
        futures::future::join_all(handles).await
    }

    pub async fn drop(&self) -> Result<()> {
        Ok(self.0.drop(None).await?)
    }

    /// Lists all modules in database. Can get heavy.
    pub async fn list_all(&self) -> Result<Vec<Module>> {
        let cursor = self.0.find(None, None).await?;
        let v: Vec<_> = cursor.collect().await;
        let valids = v.into_iter().filter_map(|v| v.ok());
        Ok(valids.collect())
    }

    /// Finds modules which satisfy the following:
    /// 1. is in the specified academic year
    /// 2. has a pre-requisite module that is in the `done` set
    /// 3. only needs at most `max_distance` more modules to unlock
    pub async fn radar(
        &self,
        acad_year: &str,
        done: HashSet<String>,
        max_distance: u8,
    ) -> Result<Vec<Module>> {
        let mut cursor =
            self.0.find(doc! { "acad_year": acad_year }, None).await?;
        let mut result = vec![];
        while let Some(module) = cursor.next().await {
            if let Ok(module) = module {
                if module.prereqtree_has_one_of(&done)
                    && module.prereqtree_min_to_unlock(&done) <= max_distance
                {
                    result.push(module)
                }
            }
        }
        Ok(result)
    }

    pub async fn count(&self) -> Result<u64> {
        Ok(self.0.count_documents(None, None).await?)
    }

    pub async fn find_one(
        &self,
        module_code: &str,
        acad_year: &str,
    ) -> Result<Module> {
        let filter = doc! {
            "module_code": module_code,
            "acad_year": acad_year,
        };
        let result = self.0.find_one(filter, None).await?;
        result.ok_or(error!(NotFound))
    }
}
