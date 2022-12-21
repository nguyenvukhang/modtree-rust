use crate::file_parser::FileParser;
use futures::prelude::*;
use std::collections::HashMap;
use types::{Error, Module, Result};

/// The spirit of this loader is to always do a two-step fetch:
///   1. from local cache
///   2. from remote data (only when step 1 fails)
#[derive(Debug)]
pub struct Loader(FileParser);

impl Loader {
    pub fn new(academic_year: &str) -> Result<Self> {
        Ok(Self(FileParser::new(academic_year)?))
    }

    /// Loads one module and all of its information.
    pub async fn load_module(&self, code: &str) -> Result<nusmods::Module> {
        Ok(self.0.load(&format!("modules/{code}.json")).await?)
    }

    /// Load list of modules from NUSMods. This pulls an extremely minimal list of modules that
    /// only contains module code, title, and semesters offered.
    pub async fn load_module_list(&self) -> Result<Vec<nusmods::ModuleShort>> {
        Ok(self.0.load("moduleList.json").await?)
    }

    /// Loads all full-info modules.
    pub async fn load_all_modules(
        &self,
        limit: Option<usize>,
    ) -> Result<Vec<Module>> {
        let shorts = self.load_module_list().await?;
        let limit = limit.unwrap_or(shorts.len());
        let mut task: HashMap<String, _> = HashMap::from_iter(
            shorts.iter().take(limit).map(|v| (v.code(), v)),
        );
        let mut result = vec![];
        let mut attempts = 0;
        let mut errors: HashMap<String, Result<Module>> = HashMap::new();
        loop {
            let (ok, bad) = self.try_load_modules(&mut task).await;
            result.extend(ok);
            bad.into_iter().for_each(|(code, err)| {
                errors.insert(code, err);
            });
            if attempts >= 5 || task.is_empty() {
                break;
            }
            attempts += 1;
            eprintln!("[fetch] retry: {attempts}");
        }
        if !errors.is_empty() {
            panic!("\nLoad failed after 5 attempts:\n\n{errors:?}\n\n")
        }
        task.is_empty().then_some(result).ok_or(Error::UnableToLoadAllModules)
    }

    /// Tries to load modules given a list of module codes.
    async fn try_load_modules(
        &self,
        codes: &mut HashMap<String, &nusmods::ModuleShort>,
    ) -> (Vec<Module>, HashMap<String, Result<Module>>) {
        self.0.clear_source_counts();
        let (total, mut done) = (codes.len(), 0);
        let interval = 200.max(total / 20);
        println!("fetching {} modules.", total);
        let handles = codes
            .iter()
            .inspect(|_| {
                done += 1;
                match done {
                    x if x % interval != 0 && x != total => (),
                    x => println!("done: {x} {:?}", self.0.source_counts()),
                }
            })
            .map(|(code, short)| async move {
                let module = self.load_module(code).await.and_then(|m| {
                    let mut m = Module::from(m);
                    m.set_semesters(&short.semesters).map(|_| m)
                });
                (code.to_owned(), module)
            });
        let results = futures::stream::iter(handles)
            .buffer_unordered(40)
            .collect::<Vec<_>>()
            .await;
        let (ok, bad): (HashMap<_, _>, HashMap<_, _>) =
            results.into_iter().partition(|v| v.1.is_ok());
        let ok = ok
            .into_iter()
            .map(|(code, result)| {
                codes.remove(&code);
                result.unwrap()
            })
            .collect();
        (ok, bad)
    }
}
