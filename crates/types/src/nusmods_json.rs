/// Direct copy of the NUSMods module data structure, defined over at
/// [NUSMods API Docs](https://api.nusmods.com/v2/)
///
/// Used for pulling data from NUSMods.
use crate::Workload;
use prereqtree::PrereqTree;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NusmodsModule {
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
    prereqtree: PrereqTree,
    #[serde(default, alias = "fulfillRequirements")]
    fulfill_requirements: Vec<String>,
    #[serde(default)]
    workload: Workload,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NusmodsModuleShort {
    #[serde(alias = "moduleCode")]
    pub module_code: String,
    pub title: String,
    pub semesters: Vec<i32>,
}

impl NusmodsModuleShort {
    pub fn code(&self) -> String {
        self.module_code.to_string()
    }
}
