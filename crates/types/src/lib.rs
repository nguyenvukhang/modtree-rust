use serde::{Deserialize, Serialize};

mod errors;
mod module;
pub use errors::*;
pub use module::Module;

pub type Result<T> = std::result::Result<T, errors::Error>;

#[derive(Debug, Serialize, Deserialize, Clone)]
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
