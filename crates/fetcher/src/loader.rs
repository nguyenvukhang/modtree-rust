use serde::de::DeserializeOwned;
use serde::Serialize;
use std::env;
use std::fs;
use std::path::PathBuf;
use types::{ModuleDetails, ModuleSummary, Result};

#[derive(Debug)]
pub struct Loader {
    root: PathBuf,
    base_url: PathBuf,
}

impl Loader {
    pub fn new() -> Result<Self> {
        dotenv::dotenv().expect(".env file not found");
        let root = env::var("MODTREE_CACHE_DIR").map(PathBuf::from)?;
        if root.is_relative() {
            Err(types::PathError::RequiresAbsolutePath(root.to_owned()))?
        }
        fs::create_dir_all(&root)?;
        Ok(Self {
            root,
            base_url: PathBuf::from("https://api.nusmods.com/v2/2022-2023/"),
        })
    }

    /// Load list of modules from NUSMods. This pulls an extremely minimal list of modules that
    /// only contains module code, title, and semesters offered.
    pub async fn load_module_list(&self) -> Result<(Vec<ModuleSummary>, bool)> {
        Ok(self.load("moduleList.json").await?)
    }

    /// Load list of modules from NUSMods. This pulls an extremely minimal list of modules that
    /// only contains module code, title, and semesters offered.
    pub async fn load_module(
        &self,
        code: &str,
    ) -> Result<(ModuleDetails, bool)> {
        Ok(self.load(&format!("modules/{code}.json")).await?)
    }

    /// Loads an arbitrary data type, first from local cache, and next from
    /// a remote url.
    async fn load<T>(&self, url: &str) -> Result<(T, bool)>
    where
        T: DeserializeOwned + Serialize,
    {
        let path = self.root.join(&url);
        let url = self.base_url.join(&url);
        let url = url.to_str().unwrap();
        let local = Loader::local::<T>(&path);
        match local {
            Ok(v) => Ok((v, true)),
            Err(_) => Loader::remote(&path, &url).await.map(|v| (v, false)),
        }
    }

    /// Try to load a struct from a local path
    fn local<T: DeserializeOwned>(path: &PathBuf) -> Result<T> {
        eprintln!("reading local cache...");
        let file = fs::File::open(path)?;
        Ok(serde_json::from_reader::<fs::File, T>(file)?)
    }

    /// Try to load a struct from a remote url
    async fn remote<T>(path: &PathBuf, url: &str) -> Result<T>
    where
        T: DeserializeOwned + Serialize,
    {
        eprintln!("fetching remote data...");
        let response = reqwest::get(url).await?;
        let parsed = response.json::<T>().await?;
        fs::create_dir_all(&path.parent().unwrap()).unwrap();
        let file = fs::File::create(path).ok();
        file.and_then(|file| serde_json::to_writer(file, &parsed).ok())
            .expect("Unable to locally cache result");
        Ok(parsed)
    }
}
