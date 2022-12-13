use crate::Workload;
use bson::oid::ObjectId;
use prereqtree::PrereqTree;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// [nusmods] Literally everything about a module.
#[derive(Serialize, Deserialize, Default)]
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
    // TODO: toggle back the pub
    #[serde(default)]
    pub semesters: Vec<i32>,
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

use std::fmt;
impl fmt::Debug for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Module")
            .field("code", &self.module_code)
            .field("sems", &self.semesters)
            .field("tree", &self.prereqtree)
            .finish()
    }
}
