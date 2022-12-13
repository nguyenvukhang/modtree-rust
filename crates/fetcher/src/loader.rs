use crate::core::FileParser;
use futures::prelude::*;
use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use types::{Error, Module, NusmodsModuleShort, Result};

#[derive(Debug)]
pub struct Loader(FileParser);

/// The spirit of this loader is to always do a two-step fetch:
///   1. from local cache
///   2. from remote data (only when step 1 fails)
impl Loader {
    pub fn new(academic_year: &str) -> Result<Self> {
        Ok(Self(FileParser::new(academic_year)?))
    }

    /// Loads one module and all of its information.
    pub async fn load_module(&self, code: &str) -> Result<Module> {
        Ok(self.0.load(&format!("modules/{code}.json")).await?)
    }

    /// Load list of modules from NUSMods. This pulls an extremely minimal list of modules that
    /// only contains module code, title, and semesters offered.
    pub async fn load_module_list(&self) -> Result<Vec<NusmodsModuleShort>> {
        Ok(self.0.load("moduleList.json").await?)
    }

    /// Loads all full-info modules.
    pub async fn load_all_modules(
        &self,
        limit: Option<usize>,
    ) -> Result<Vec<Module>> {
        let mut l = self.load_module_list().await?;
        l.truncate(limit.unwrap_or(l.len()));
        let mut task = HashSet::from_iter(l.into_iter().map(|m| m.code()));
        let mut result = vec![];
        let mut attempts = 0;
        loop {
            result.extend(self.try_load_modules(&mut task).await);
            if attempts > 5 || task.is_empty() {
                break;
            }
            attempts += 1;
            eprintln!("[fetch] retry: {attempts}");
        }
        task.is_empty().then_some(result).ok_or(Error::UnableToLoadAllModules)
    }

    /// Tries to load modules given a list of module codes.
    async fn try_load_modules(
        &self,
        codes: &mut HashSet<String>,
    ) -> Vec<Module> {
        self.0.clear_source_counts();
        let (total, done) = (codes.len(), Arc::new(AtomicUsize::new(0)));
        let interval = 200.max(total / 5);
        println!("fetching {} modules.", total);
        let handles = codes.iter().map(|code| (code, Arc::clone(&done))).map(
            |(code, done)| async move {
                let res = self.load_module(code).await;
                match done.fetch_add(1, Ordering::SeqCst) + 1 {
                    x if x % interval != 0 && x != total => (),
                    x => println!("done: {x} {:?}", self.0.source_counts()),
                }
                res
            },
        );
        futures::stream::iter(handles)
            .buffer_unordered(40)
            .collect::<Vec<Result<Module>>>()
            .await
            .into_iter()
            .filter_map(|v| v.ok())
            .inspect(|v| {
                codes.remove(&v.code());
            })
            .collect()
    }
}
