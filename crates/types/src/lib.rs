use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

mod errors;
pub use errors::*;
use prereqtree::PrereqTree;

pub type Result<T> = std::result::Result<T, errors::Error>;

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

/// Short summary of a module.
#[derive(Serialize, Deserialize, Debug)]
pub struct ModuleShort {
    #[serde(alias = "moduleCode")]
    pub module_code: String,
    pub title: String,
    pub semesters: Vec<i32>,
}

/// [nusmods] Literally everything about a module.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Module {
    #[serde(default)]
    _id: ObjectId,
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
    // extra stuff
    #[serde(default)]
    semesters: Vec<i32>,
}

impl ModuleShort {
    pub fn code(&self) -> String {
        self.module_code.to_string()
    }
}

impl Module {
    pub fn code(&self) -> String {
        self.module_code.to_string()
    }
    pub fn academic_year(&self) -> String {
        self.acad_year.to_string()
    }
    pub fn is_leaf(&self) -> bool {
        self.prereqtree.is_leaf()
    }
    pub fn satisfied_by(&self, done: &HashSet<String>) -> bool {
        self.prereqtree.satisfied_by(done)
    }
    pub fn prereqtree(&self) -> PrereqTree {
        self.prereqtree.clone()
    }
    pub fn prereqtree_contains(&self, module_code: &str) -> bool {
        self.prereqtree.contains_code(module_code)
    }
    pub fn prereqtree_flatten(&self) -> Vec<String> {
        self.prereqtree.flatten()
    }
    pub fn prereqtree_has_one_of(&self, module_code: &HashSet<String>) -> bool {
        module_code.iter().any(|code| self.prereqtree.contains_code(code))
    }
    pub fn min_to_unlock(&self, done: &HashSet<String>) -> u8 {
        self.prereqtree.min_to_unlock(done)
    }
    pub fn min_path(&self) -> Vec<String> {
        self.prereqtree.min_path()
    }
    pub fn min_path_filtered<S>(&self, filter: &Vec<S>) -> Option<Vec<String>>
    where
        S: AsRef<str>,
    {
        let f: Vec<_> = filter.iter().map(|v| v.as_ref().to_string()).collect();
        self.prereqtree.min_path_filtered(&f)
    }
}

impl PartialEq for Module {
    fn eq(&self, other: &Self) -> bool {
        self._id == other._id
    }
    fn ne(&self, other: &Self) -> bool {
        self._id != other._id
    }
}
impl Eq for Module {}

use std::hash::{Hash, Hasher};
impl Hash for Module {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self._id.hash(state);
    }
}
