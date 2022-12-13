use crate::util;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use types::Result;

const NUSMODS_API_URL: &str = "https://api.nusmods.com/v2";

#[derive(Debug)]
pub struct FileParser {
    root: PathBuf,
    base_url: PathBuf,
    local_count: Arc<AtomicUsize>,
    remote_count: Arc<AtomicUsize>,
}

impl FileParser {
    pub fn new(academic_year: &str) -> Result<Self> {
        dotenv::dotenv().expect(".env file not found");
        let root = std::env::var("MODTREE_CACHE_DIR").map(PathBuf::from)?;
        if root.is_relative() {
            Err(types::Error::RequiresAbsolutePath(root.clone()))?
        }
        let academic_year = util::validate_academic_year(academic_year, '-')?;
        let base_url = PathBuf::from(NUSMODS_API_URL);
        fs::create_dir_all(&root)?;
        Ok(Self {
            root: root.join(&academic_year),
            base_url: base_url.join(&academic_year),
            local_count: Arc::new(AtomicUsize::new(0)),
            remote_count: Arc::new(AtomicUsize::new(0)),
        })
    }

    /// Loads an arbitrary data type, first from local cache, and next from
    /// a remote url.
    pub async fn load<T>(&self, url: &str) -> Result<T>
    where
        T: DeserializeOwned + Serialize,
    {
        let (path, url) = (self.root.join(&url), self.base_url.join(&url));
        match self.local::<T>(&path) {
            Err(_) => self.remote(&path, &url.to_str().unwrap()).await,
            v => v,
        }
    }

    /// Try to load a struct from a local path
    pub fn local<T: DeserializeOwned>(&self, path: &PathBuf) -> Result<T> {
        let result = serde_json::from_reader::<File, T>(File::open(path)?)?;
        self.local_count.fetch_add(1, Ordering::SeqCst);
        Ok(result)
    }

    /// Try to load a struct from a remote url
    pub async fn remote<T>(&self, path: &PathBuf, url: &str) -> Result<T>
    where
        T: DeserializeOwned + Serialize,
    {
        fs::create_dir_all(&path.parent().unwrap()).unwrap();
        let mut file = File::create(path)?;
        let text = reqwest::get(url).await?.text().await?;
        file.write_fmt(format_args!("{text}"))?;
        self.remote_count.fetch_add(1, Ordering::SeqCst);
        self.local::<T>(&path)
    }

    pub fn source_counts(&self) -> [usize; 2] {
        [
            self.local_count.load(Ordering::Relaxed),
            self.remote_count.load(Ordering::Relaxed),
        ]
    }

    pub fn clear_source_counts(&self) {
        self.local_count.store(0, Ordering::Relaxed);
        self.remote_count.store(0, Ordering::Relaxed);
    }
}
