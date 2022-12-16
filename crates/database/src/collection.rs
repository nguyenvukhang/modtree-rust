use futures::stream::StreamExt;
use mongodb::bson::{doc, to_document};
use mongodb::options::UpdateOptions;
use mongodb::results::{InsertManyResult, UpdateResult};
use prereqtree::PrereqTree;
use std::collections::{HashMap, HashSet};
use types::{Error, Module, Result};


/// Module Collection Inner
#[derive(Debug, Clone)]
struct MCI(mongodb::Collection<Module>);

#[derive(Debug, Clone)]
pub struct ModuleCollection(mongodb::Collection<Module>);

impl ModuleCollection {
    pub fn new(x: mongodb::Collection<Module>) -> Self {
        Self(x)
    }

    pub async fn count(&self) -> Result<u64> {
        Ok(self.0.count_documents(None, None).await?)
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
        let doc = to_document(module)?;
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
            let to_unlock = module.left_to_unlock(&done);
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
        let mut remain: HashSet<String> =
            HashSet::from_iter(module_codes.iter().map(|v| v.to_string()));
        let doc = doc! {
            "acad_year": acad_year,
            "module_code": { "$in": &module_codes }
        };
        let mut res = HashMap::new();
        let mut cursor = self.0.find(doc, None).await?;
        while let Some(module) = cursor.next().await {
            if let Ok(m) = module {
                remain.remove(m.code());
                res.insert(m.to_code(), Ok(m));
            }
        }
        res.extend(remain.into_iter().map(|c| {
            (c.to_owned(), Err(Error::ModuleNotFound(c, acad_year.into())))
        }));
        Ok(res)
    }

    pub async fn flatten_requirements(
        &self,
        codes: Vec<String>,
        acad_year: &str,
    ) -> Result<Vec<Module>> {
        let mut remain = codes;
        let mut result: HashSet<Module> = HashSet::new();
        let mut fetched: HashSet<String> = HashSet::new();
        while !remain.is_empty() {
            let response = self.find_many(&remain, &acad_year).await?;
            remain = vec![];
            for (code, module) in response {
                let prereqs = match module {
                    Ok(v) => {
                        fetched.insert(code.to_string());
                        let prereqs = v.prereqtree_flatten();
                        result.insert(v);
                        prereqs
                    }
                    Err(_) => {
                        fetched.insert(code);
                        continue;
                    }
                };
                remain.extend(
                    prereqs.into_iter().filter(|c| !fetched.contains(c)),
                );
            }
        }
        Ok(Vec::from_iter(result))
    }

    pub fn topological_sort(
        modules: Vec<(String, Module)>,
    ) -> Vec<(String, Module)> {
        let mut hash = HashMap::new();
        let mut sorter = vec![];
        for (code, module) in modules {
            sorter.push((module.to_code(), module.prereqtree()));
            hash.insert(code, module);
        }
        PrereqTree::topological_sort(sorter)
            .into_iter()
            .map(|(code, _)| {
                let module = std::mem::take(hash.get_mut(&code).unwrap());
                (code, module)
            })
            .collect()
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
        println!("tree->{tree:?}");
        tree.resolve("MA1512");
        tree.resolve("MA1511");
        println!("tree->{tree:?}");
        Ok(())
    }
}
