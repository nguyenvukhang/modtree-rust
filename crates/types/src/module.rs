use crate::{Error, Result, Workload};
use bson::oid::ObjectId;
use prereqtree::PrereqTree;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// `modtree` edition of a module
#[derive(Serialize, Deserialize, Default)]
pub struct Module {
    acad_year: String,
    preclusion: String,
    description: String,
    title: String,
    department: String,
    faculty: String,
    prerequisite: String,
    module_credit: String,
    module_code: String,
    fulfill_requirements: Vec<String>,
    prereqtree: PrereqTree,
    workload: Workload,
    // extra stuff on top of standard NUSMods API
    semesters: Vec<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    _id: Option<ObjectId>,
}

impl Module {
    pub fn set_semesters(&mut self, sems: &Vec<usize>) -> Result<()> {
        self.semesters = sems.clone();
        sems.iter()
            .all(|v| 1 <= *v && *v <= 4)
            .then_some(())
            .ok_or(Error::InvalidSemesters(sems.clone()))
    }
    pub fn is_offered_in_sem(&self, sem: usize) -> bool {
        self.semesters.contains(&sem)
    }
    pub fn code(&self) -> &String {
        &self.module_code
    }
    pub fn semesters(&self) -> &Vec<usize> {
        &self.semesters
    }
    pub fn to_code(&self) -> String {
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
    pub fn left_to_unlock(&self, done: &HashSet<String>) -> u8 {
        self.prereqtree.left_to_unlock(done)
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

impl From<nusmods::Workload> for Workload {
    fn from(w: nusmods::Workload) -> Self {
        match w {
            nusmods::Workload::String(s) => Self::String(s),
            nusmods::Workload::Numbers(v) => Self::Numbers(v),
        }
    }
}

impl From<nusmods::Module> for Module {
    fn from(m: nusmods::Module) -> Self {
        Self {
            _id: None,
            semesters: vec![],
            acad_year: m.acad_year,
            preclusion: m.preclusion,
            description: m.description,
            title: m.title,
            department: m.department,
            faculty: m.faculty,
            prerequisite: m.prerequisite,
            module_credit: m.module_credit,
            module_code: m.module_code,
            fulfill_requirements: m.fulfill_requirements,
            workload: Workload::from(m.workload),
            prereqtree: PrereqTree::from(m.prereqtree),
        }
    }
}
