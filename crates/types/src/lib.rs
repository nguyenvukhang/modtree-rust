use serde::{Deserialize, Serialize};

mod errors;
pub use errors::*;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub enum ModtreeError {}

/// Short summary of a module found when pulling a module list.
#[derive(Serialize, Deserialize, Debug)]
pub struct ModuleSummary {
    #[serde(alias = "moduleCode")]
    pub module_code: String,
    pub title: String,
    pub semesters: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrereqTree {
    String(String),
    Node {
        #[serde(default)]
        and: Vec<PrereqTree>,
        #[serde(default)]
        or: Vec<PrereqTree>,
    },
}

impl Default for PrereqTree {
    fn default() -> Self {
        Self::String("".to_string())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Workload {
    String(String),
    Numbers(Vec<u8>),
    Fractions(Vec<f64>),
}

impl Default for Workload {
    fn default() -> Self {
        Self::Numbers(vec![])
    }
}

/// Literally everything about a module.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ModuleDetails {
    #[serde(default, alias = "acadYear")]
    acad_year: String,
    #[serde(default)]
    preclusion: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    title: String,
    #[serde(default)]
    department: String,
    #[serde(default)]
    faculty: String,
    #[serde(default)]
    prerequisite: String,
    #[serde(default, alias = "moduleCredit")]
    module_credit: String,
    #[serde(default, alias = "moduleCode")]
    module_code: String,
    #[serde(default, alias = "prereqTree")]
    prereq_tree: PrereqTree,
    #[serde(default, alias = "fulfillRequirements")]
    fulfill_requirements: Vec<String>,
    #[serde(default)]
    workload: Workload,
}

/// Condensed version of a module, enough to execute business logic.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Module {
    #[serde(default, alias = "acadYear")]
    acad_year: String,
    #[serde(default)]
    preclusion: String,
    #[serde(default)]
    title: String,
    #[serde(default)]
    prerequisite: String,
    #[serde(default, alias = "moduleCode")]
    module_code: String,
    #[serde(default, alias = "prereqTree")]
    prereq_tree: PrereqTree,
    #[serde(default, alias = "fulfillRequirements")]
    fulfill_requirements: Vec<String>,
}

impl From<ModuleDetails> for Module {
    fn from(m: ModuleDetails) -> Self {
        Self {
            acad_year: m.acad_year,
            preclusion: m.preclusion,
            title: m.title,
            prerequisite: m.prerequisite,
            module_code: m.module_code,
            prereq_tree: m.prereq_tree,
            fulfill_requirements: m.fulfill_requirements,
        }
    }
}
