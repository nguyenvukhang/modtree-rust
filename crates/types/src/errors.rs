use std::path::PathBuf;
use thiserror::Error;

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

    #[error("Path needs to be absolute: {0}")]
    RequiresAbsolutePath(PathBuf),

    #[error("Not found")]
    NotFound,

    #[error("Module not found: {0}")]
    ModuleNotFound(String),

    #[error("Pre-requisites not satisfied for module: {0} -> {1}")]
    PrerequisitesNotSatisfied(String, String),

    #[error("reqwuest Error: {0:#?}")]
    ReqwestErr(reqwest::Error),

    #[error("io Error: {0:#?}")]
    IoErr(std::io::Error),

    #[error("serde Error: {0:#?}")]
    SerdeErr(serde_json::Error),

    #[error("mongodb Error: {0:#?}")]
    MongoDbErr(mongodb::error::Error),

    #[error("mongodb serialize Error: {0:#?}")]
    MongoDbSerErr(mongodb::bson::ser::Error),

    #[error("env Error: {0:#?}")]
    EnvErr(std::env::VarError),

    #[error("Unable to parse integer: {0:#?}")]
    ParseIntError(std::num::ParseIntError),

    #[error("Unable to parse float: {0:#?}")]
    ParseFloatError(std::num::ParseFloatError),

    #[error("Tried to delete a core database: {0}")]
    MongoDbBadDrop(String),
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::ReqwestErr(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::IoErr(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::SerdeErr(error)
    }
}

impl From<mongodb::error::Error> for Error {
    fn from(error: mongodb::error::Error) -> Self {
        Self::MongoDbErr(error)
    }
}

impl From<std::env::VarError> for Error {
    fn from(error: std::env::VarError) -> Self {
        Self::EnvErr(error)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Self {
        Self::ParseIntError(error)
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(error: std::num::ParseFloatError) -> Self {
        Self::ParseFloatError(error)
    }
}

impl From<mongodb::bson::ser::Error> for Error {
    fn from(error: mongodb::bson::ser::Error) -> Self {
        Self::MongoDbSerErr(error)
    }
}

#[macro_export]
macro_rules! error {
    ($type:ident) => {
        Error::$type
    };
    ($type:ident, $($args:expr),*) => {
        Error::$type($($args.into()),*)
    };
}

fn compile_test() {
    error!(UnableToLoadAllModules);
    error!(RequiresAbsolutePath, "/hello");
    error!(PrerequisitesNotSatisfied, "CS2040", "CS1010");
}
