use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PathError {
    #[error("Path needs to be absolute: {0}")]
    RequiresAbsolutePath(PathBuf),
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("Mongo DB client is not running")]
    MongoDbClientNotRunning,
    #[error("Unable to fully load all modules")]
    UnableToLoadAllModules,
}
