use futures::stream::StreamExt;
use mongodb::bson::{doc, to_document, Document};
use mongodb::options::UpdateOptions;
use mongodb::results::{DeleteResult, InsertManyResult, UpdateResult};
use mongodb::Cursor;
use std::collections::{HashMap, HashSet};
use types::{Error, Module, Result};

#[derive(Debug, Clone)]
pub struct ModuleCollection(mongodb::Collection<Module>);

impl ModuleCollection {
    pub fn new(x: mongodb::Collection<Module>) -> Self {
        Self(x)
    }

    /// Gets a count of all modules in the collection.
    pub async fn count(&self) -> Result<u64> {
        Ok(self.0.count_documents(None, None).await?)
    }

    /// Inserts one module that is uniquely identified by module code and
    /// academic year.
    pub async fn insert_one(&self, module: &Module) -> Result<UpdateResult> {
        let doc = to_document(module)?;
        let query = doc! {
            "module_code": module.code(),
            "acad_year": module.acad_year(),
        };
        let opts = UpdateOptions::builder().upsert(true).build();
        Ok(self.0.update_one(query, doc! { "$set": doc }, opts).await?)
    }

    /// Inserts modules which do not already exist in database. A module is
    /// uniquely identified by its module code and academic year.
    pub async fn insert_many(
        &self,
        modules: &Vec<Module>,
    ) -> Vec<Result<UpdateResult>> {
        let handles = modules
            .iter()
            .map(|module| async move { self.insert_one(module).await });
        futures::future::join_all(handles).await
    }

    /// Inserts many modules without checking for duplicate
    /// module code/academic year
    pub async fn insert_many_unchecked(
        &self,
        modules: &Vec<Module>,
    ) -> Result<InsertManyResult> {
        Ok(self.0.insert_many(modules, None).await?)
    }

    /// Drops collection and deletes all modules.
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

    /// Finds one module by academic year and module code
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
        result.ok_or(Error::ModuleNotFound(
            module_code.to_string(),
            acad_year.to_string(),
        ))
    }

    /// Finds many modules, and returns a same-sized result.
    pub async fn find_many(
        &self,
        module_codes: &Vec<String>,
        acad_year: &str,
    ) -> Result<HashMap<String, Result<Module>>> {
        let mut task: HashSet<String> =
            HashSet::from_iter(module_codes.iter().map(|v| v.to_string()));
        let doc = doc! {
            "acad_year": acad_year,
            "module_code": { "$in": &module_codes }
        };
        let mut res = HashMap::new();
        let mut cursor = self.0.find(doc, None).await?;
        while let Some(module) = cursor.next().await {
            if let Ok(m) = module {
                task.remove(m.code());
                res.insert(m.to_code(), Ok(m));
            }
        }
        res.extend(task.into_iter().map(|c| {
            (c.to_owned(), Err(Error::ModuleNotFound(c, acad_year.into())))
        }));
        Ok(res)
    }

    /// Finds many modules, and returns a same-sized result.
    pub async fn find_many_by_year(
        &self,
        acad_year: &str,
    ) -> Result<Cursor<Module>> {
        Ok(self.0.find(doc! { "acad_year": acad_year }, None).await?)
    }

    /// Deletes many modules
    pub async fn delete_many(&self, query: Document) -> Result<DeleteResult> {
        self.0.delete_many(query, None).await.map_err(|e| e.into())
    }

    /// For loading a new academic year into the database.
    pub async fn import_academic_year(
        &self,
        academic_year: &str,
        limit: Option<usize>,
    ) -> Result<()> {
        let loader = fetcher::Loader::new(academic_year)?;
        let modules = loader.load_all_modules(limit).await?;
        self.delete_many(doc! { "academic_year": academic_year }).await?;
        self.insert_many_unchecked(&modules).await.map(|_| ())
    }
}
