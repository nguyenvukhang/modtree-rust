use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PathError {
    #[error("Path needs to be absolute: {0}")]
    RequiresAbsolutePath(PathBuf),
}
