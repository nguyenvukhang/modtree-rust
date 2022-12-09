use serde::de::DeserializeOwned;
use serde::Serialize;
use std::error;
use std::fs;
use std::mem;
use std::path::{Path, PathBuf};

pub struct Getter {
    url: String,
    path: PathBuf,
}

impl Getter {
    pub fn new<P: AsRef<Path>>(url: &str, path: P) -> Self {
        let path = PathBuf::from(path.as_ref());
        fs::create_dir_all(&path.parent().unwrap()).unwrap();
        Self {
            url: String::from(url),
            path,
        }
    }

    pub fn clear_cache(&self) -> Result<(), std::io::Error> {
        fs::remove_file(&self.path)
    }

    pub async fn get<T>(&self, buffer: &mut T) -> Result<(), Box<dyn error::Error>>
    where
        T: DeserializeOwned + Serialize,
    {
        let mut json = match self.path.is_file() {
            true => {
                println!("Reading from local file...");
                let file = fs::File::open(&self.path)?;
                serde_json::from_reader::<fs::File, T>(file)?
            }
            false => {
                println!("Fetching from the internet...");
                let file = fs::File::create(&self.path)?;
                let response = reqwest::get(&self.url).await?;
                let json = response.json::<T>().await?;
                serde_json::to_writer(file, &json)?;
                json
            }
        };
        Ok(mem::swap(&mut json, buffer))
    }

    pub async fn debug_fetch(&self) -> Result<(), Box<dyn error::Error>> {
        println!("Fetching from the internet...");
        let response = reqwest::get(&self.url).await?;
        let raw = response.text().await?;
        println!("raw -> {raw:?}");
        Ok(())
    }
}
