use futures::stream::StreamExt;
use mongodb::bson::{doc, to_document};
use mongodb::options::UpdateOptions;
use mongodb::results::{InsertManyResult, UpdateResult};
use std::collections::{HashMap, HashSet};
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
        limit: Option<usize>,
    ) -> Result<()> {
        let loader = fetcher::Loader::new(academic_year)?;
        let modules = loader.load_all_modules(limit).await?;
        let delete_query = doc! { "academic_year": academic_year };
        self.0.delete_many(delete_query, None).await?;
        self.insert_many_unchecked(&modules).await.map(|_| ())
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
    ) -> Result<HashMap<u8, Vec<Module>>> {
        let mut cursor =
            self.0.find(doc! { "acad_year": acad_year }, None).await?;
        let mut result: HashMap<u8, Vec<Module>> = HashMap::new();
        while let Some(module) = cursor.next().await {
            let module = match module {
                Ok(v) => v,
                _ => continue,
            };
            if !module.prereqtree_has_one_of(&done) {
                continue;
            }
            let to_unlock = module.min_to_unlock(&done);
            if to_unlock <= max_distance {
                if result.contains_key(&to_unlock) {
                    result.entry(to_unlock).and_modify(|v| v.push(module));
                } else {
                    result.insert(to_unlock, vec![module]);
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
        result.ok_or(error!(ModuleNotFound, module_code.to_string()))
    }

    /// Gets a full path down to modules with no requirements.
    pub async fn min_path(
        &self,
        module_code: &str,
        acad_year: &str,
    ) -> Result<()> {
        let filter = doc! {
            "module_code": module_code,
            "acad_year": acad_year,
        };
        let root_module = self.0.find_one(filter, None).await?.unwrap();
        let mut tree = root_module.prereqtree();
        let loader = |code: String| async move {
            let a = self.find_one(&code, acad_year).await;
            a.ok().map(|m| m.prereqtree())
        };
        let globalist = tree.global_flatten(loader).await;
        println!("globalist->{globalist:?}");
        Ok(())
    }
}
