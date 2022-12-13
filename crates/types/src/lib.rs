use serde::{Deserialize, Serialize};

mod errors;
mod module;
mod nusmods_json;
pub use errors::*;
pub use module::Module;
pub use nusmods_json::{NusmodsModule, NusmodsModuleShort};

pub type Result<T> = std::result::Result<T, errors::Error>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum Workload {
    String(String),
    Numbers(Vec<f32>),
}

impl Default for Workload {
    fn default() -> Self {
        Self::Numbers(vec![])
    }
}
