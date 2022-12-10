use mongodb::bson::{doc, to_document};
use mongodb::options::UpdateOptions;
use mongodb::results::UpdateResult;
use types::{Module, Result};

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
        let modules = loader.load_all_modules().await?;
        println!("Done loading all modules from JSON");
        println!("Inserting modules to mongo-db...");
        self.insert_many_modules(&modules).await;
        println!("Done.");
        Ok(())
    }

    pub async fn insert_one_module(
        &self,
        module: &Module,
    ) -> Result<UpdateResult> {
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

    pub async fn insert_many_modules(
        &self,
        modules: &Vec<Module>,
    ) -> Vec<Result<UpdateResult>> {
        let handles = modules
            .iter()
            .map(|module| async move { self.insert_one_module(module).await });
        futures::future::join_all(handles).await
    }

    pub async fn drop(&self) -> Result<()> {
        Ok(self.0.drop(None).await?)
    }

    pub async fn count(&self) -> Result<u64> {
        Ok(self.0.count_documents(None, None).await?)
    }

    pub async fn find_one_module(
        &self,
        module_code: &str,
        acad_year: &str,
    ) -> Result<Option<Module>> {
        let filter = doc! {
            "module_code": module_code,
            "acad_year": acad_year,
        };
        Ok(self.0.find_one(filter, None).await?)
    }
}
