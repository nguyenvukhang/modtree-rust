#![allow(unused)]

use crate::util;
use futures::prelude::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use types::{error, Error, Module, ModuleShort, Result};

#[derive(Debug)]
pub struct Loader {
    root: PathBuf,
    base_url: PathBuf,
    local_count: Arc<AtomicUsize>,
    remote_count: Arc<AtomicUsize>,
}

/// The spirit of this loader is to always do a two-step fetch:
///   1. from local cache
///   2. from remote data (only when step 1 fails)
impl Loader {
    pub fn new(academic_year: &str) -> Result<Self> {
        dotenv::dotenv().expect(".env file not found");
        let root = env::var("MODTREE_CACHE_DIR").map(PathBuf::from)?;
        if root.is_relative() {
            Err(types::Error::RequiresAbsolutePath(root.clone()))?
        }
        let academic_year = util::validate_academic_year(academic_year, '-')?;
        let base_url = PathBuf::from("https://api.nusmods.com/v2");
        fs::create_dir_all(&root)?;
        Ok(Self {
            root: root.join(&academic_year),
            base_url: base_url.join(&academic_year),
            local_count: Arc::new(AtomicUsize::new(0)),
            remote_count: Arc::new(AtomicUsize::new(0)),
        })
    }

    fn source_counts(&self) -> [usize; 2] {
        [
            self.local_count.load(Ordering::Relaxed),
            self.remote_count.load(Ordering::Relaxed),
        ]
    }

    fn clear_source_counts(&self) {
        self.local_count.store(0, Ordering::Relaxed);
        self.remote_count.store(0, Ordering::Relaxed);
    }

    /// Load list of modules from NUSMods. This pulls an extremely minimal list of modules that
    /// only contains module code, title, and semesters offered.
    pub async fn load_module_list(&self) -> Result<Vec<ModuleShort>> {
        Ok(self.load("moduleList.json").await?)
    }

    /// Loads all full-info modules.
    pub async fn load_all_modules(
        &self,
        limit: Option<usize>,
    ) -> Result<Vec<Module>> {
        eprintln!("Reading list of modules...");
        let mut shorts: Vec<ModuleShort> = self.load("moduleList.json").await?;
        if let Some(limit) = limit {
            shorts = shorts.into_iter().take(limit).collect();
        }
        let target = shorts.len();
        eprintln!("Fetch target count: {target}");
        let mut remaining: HashSet<String> =
            HashSet::from_iter(shorts.into_iter().map(|m| m.code()));
        let mut result: Vec<Module> = vec![];
        let mut attempts = 0;
        while attempts < 5 && result.len() < target {
            if attempts > 0 {
                eprintln!("[fetcher] retry: {attempts}");
            }
            let prelim = self._load_all_modules(&remaining).await;
            for module in &prelim {
                remaining.remove(&module.code());
            }
            result.extend(prelim);
            eprintln!("[fetcher] fetched: {}/{target}", result.len());
            attempts += 1;
        }
        if result.len() == target {
            Ok(result)
        } else {
            Err(Error::UnableToLoadAllModules)
        }
    }

    /// Inner helper function to load all modules.
    async fn _load_all_modules(&self, list: &HashSet<String>) -> Vec<Module> {
        self.clear_source_counts();
        let done = Arc::new(AtomicUsize::new(0));
        let target = list.len();
        let interval = 200.max(target / 10);
        println!("[fetcher]: inner fetching {} modules.", list.len());
        let handles = list.iter().map(|code| (code, Arc::clone(&done))).map(
            |(code, done)| async move {
                let res = self.load_module(code).await;
                let _ = done.fetch_update(
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                    |x| {
                        if x % interval == 0 {
                            println!(
                                "[fetcher] {x}/{target} {:?}",
                                self.source_counts()
                            );
                        }
                        Some(x + 1)
                    },
                );
                res
            },
        );
        let stream = futures::stream::iter(handles).buffer_unordered(40);
        let results = stream.collect::<Vec<_>>().await;
        println!("[fetcher] attempt done: {:?}", self.source_counts());
        results.into_iter().filter_map(|v| v.ok()).collect()
    }

    /// Loads one module and all of its information.
    pub async fn load_module(&self, code: &str) -> Result<Module> {
        Ok(self.load(&format!("modules/{code}.json")).await?)
    }

    /// Loads an arbitrary data type, first from local cache, and next from
    /// a remote url.
    async fn load<T>(&self, url: &str) -> Result<T>
    where
        T: DeserializeOwned + Serialize,
    {
        let path = self.root.join(&url);
        let url = self.base_url.join(&url);
        let url = url.to_str().unwrap();
        match self.local::<T>(&path) {
            Err(_) => self.remote(&path, &url).await,
            v => v,
        }
    }

    /// Reset counters for local/remote pulls.
    pub fn reset_counts(&self) {
        let _ = &self.local_count.store(0, Ordering::SeqCst);
        let _ = &self.remote_count.store(0, Ordering::SeqCst);
    }

    /// Try to load a struct from a local path
    fn local<T: DeserializeOwned>(&self, path: &PathBuf) -> Result<T> {
        let file = fs::File::open(path)?;
        let result = serde_json::from_reader::<fs::File, T>(file)?;
        let _ = &self.local_count.fetch_add(1, Ordering::SeqCst);
        Ok(result)
    }

    /// Try to load a struct from a remote url
    async fn remote<T>(&self, path: &PathBuf, url: &str) -> Result<T>
    where
        T: DeserializeOwned + Serialize,
    {
        let response2 = reqwest::get(url).await?;
        let text = response2.text().await?;
        fs::create_dir_all(&path.parent().unwrap()).unwrap();
        let mut file = fs::File::create(path)?;
        use std::io::Write;
        file.write_fmt(format_args!("{text}"))?;
        let _ = &self.remote_count.fetch_add(1, Ordering::SeqCst);
        self.local::<T>(&path)
    }
}
