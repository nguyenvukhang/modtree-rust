use serde::de::DeserializeOwned;
use serde::Serialize;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use types::{ModuleDetails, ModuleSummary, Result};

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
    pub fn new() -> Result<Self> {
        dotenv::dotenv().expect(".env file not found");
        let root = env::var("MODTREE_CACHE_DIR").map(PathBuf::from)?;
        if root.is_relative() {
            Err(types::PathError::RequiresAbsolutePath(root.to_owned()))?
        }
        let base_url = PathBuf::from("https://api.nusmods.com/v2/2022-2023/");
        fs::create_dir_all(&root)?;
        Ok(Self {
            root,
            base_url,
            local_count: Arc::new(AtomicUsize::new(0)),
            remote_count: Arc::new(AtomicUsize::new(0)),
        })
    }

    pub fn report(&self) {
        println!("local: {}", self.local_count.load(Ordering::Relaxed));
        println!("remote: {}", self.remote_count.load(Ordering::Relaxed));
    }

    /// Load list of modules from NUSMods. This pulls an extremely minimal list of modules that
    /// only contains module code, title, and semesters offered.
    pub async fn load_module_list(&self) -> Result<Vec<ModuleSummary>> {
        Ok(self.load("moduleList.json").await?)
    }

    /// Load list of modules from NUSMods. This pulls an extremely minimal list of modules that
    /// only contains module code, title, and semesters offered.
    pub async fn load_module(&self, code: &str) -> Result<ModuleDetails> {
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
        let response = reqwest::get(url).await?;
        let parsed = response.json::<T>().await?;
        fs::create_dir_all(&path.parent().unwrap()).unwrap();
        let file = fs::File::create(path).ok();
        file.and_then(|file| serde_json::to_writer(file, &parsed).ok())
            .expect("Unable to locally cache result");
        let _ = &self.remote_count.fetch_add(1, Ordering::SeqCst);
        Ok(parsed)
    }
}
