use crate::{Workload};
use bson::oid::ObjectId;
use prereqtree::PrereqTree;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};

/// `modtree` edition of a module
#[derive(Serialize, Deserialize, Default, Clone)]
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
    /// Gets a reference to the module's academic year
    pub fn acad_year(&self) -> &String {
        &self.acad_year
    }

    /// Gets a reference to the module's code
    pub fn code(&self) -> &String {
        &self.module_code
    }

    /// Clones the module's code
    pub fn to_code(&self) -> String {
        self.module_code.to_string()
    }

    /// Gets a reference to the module's semester list
    pub fn semesters(&self) -> &Vec<usize> {
        &self.semesters
    }

    /// Clones the module's semester list
    pub fn to_semesters(&self) -> Vec<usize> {
        self.semesters.clone()
    }

    /// Gets a reference to the module's prereqtree
    pub fn prereqtree(&self) -> &PrereqTree {
        &self.prereqtree
    }

    /// Clones the module's prereqtree
    pub fn to_prereqtree(&self) -> PrereqTree {
        self.prereqtree.clone()
    }

    /// Gets all module codes in the prereqtree
    pub fn prereqtree_flatten(&self) -> Vec<String> {
        self.prereqtree.flatten()
    }

    /// Sets the tree
    pub fn set_tree(&mut self, tree: PrereqTree) {
        self.prereqtree = tree
    }

    /// Sets the semesters
    pub fn set_semesters(&mut self, sems: Vec<usize>) {
        self.semesters = sems;
    }
}

impl PartialEq for Module {
    fn eq(&self, other: &Self) -> bool {
        self._id == other._id
    }
}
impl Eq for Module {}

impl Hash for Module {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self._id.hash(state);
    }
}

impl fmt::Debug for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Module")
            .field("code", &self.module_code)
            .field("sems", &self.semesters)
            .field("tree", &self.prereqtree)
            .finish()
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
