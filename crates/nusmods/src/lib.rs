/// Direct copy of the NUSMods module data structure, defined over at
/// [NUSMods API Docs](https://api.nusmods.com/v2/)
///
/// Used for pulling data from NUSMods.
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum PrereqTree {
    Only(String),
    And { and: Vec<PrereqTree> },
    Or { or: Vec<PrereqTree> },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Workload {
    String(String),
    Numbers(Vec<f32>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Module {
    #[serde(default, alias = "acadYear")]
    pub acad_year: String,
    #[serde(default)]
    pub preclusion: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub department: String,
    #[serde(default)]
    pub faculty: String,
    #[serde(default)]
    pub prerequisite: String,
    #[serde(default, alias = "moduleCredit")]
    pub module_credit: String,
    #[serde(default, alias = "moduleCode")]
    pub module_code: String,
    #[serde(default, alias = "prereqTree")]
    pub prereqtree: PrereqTree,
    #[serde(default, alias = "fulfillRequirements")]
    pub fulfill_requirements: Vec<String>,
    #[serde(default)]
    pub workload: Workload,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModuleShort {
    #[serde(alias = "moduleCode")]
    pub module_code: String,
    pub title: String,
    pub semesters: Vec<usize>,
}

impl ModuleShort {
    pub fn to_code(&self) -> String {
        self.module_code.to_string()
    }
}

// For pulling modules that do not have these fields
impl Default for PrereqTree {
    fn default() -> Self {
        Self::Only("".to_string())
    }
}
impl Default for Workload {
    fn default() -> Self {
        Self::Numbers(vec![])
    }
}
