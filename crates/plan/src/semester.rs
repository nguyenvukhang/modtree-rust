#![allow(unused)]

use crate::structs::ModuleKind;
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Default, Clone)]
pub struct Semester {
    modules: Vec<(String, ModuleKind)>,
    limit: usize,
}

impl Hash for Semester {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.modules.hash(state);
    }
}

impl fmt::Debug for Semester {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.modules.iter().map(|v| &v.0).collect::<Vec<_>>())
    }
}

impl Semester {
    pub fn new(limit: usize) -> Self {
        Self { modules: vec![], limit }
    }
    pub fn insert(&mut self, code: &str, kind: ModuleKind) {
        self.modules.push((code.to_string(), kind));
        self.modules.sort_by(|a, b| a.0.cmp(&b.0))
    }
    pub fn is_empty(&self) -> bool {
        self.modules.is_empty()
    }
    fn get_kind(&self, kind: ModuleKind) -> Vec<String> {
        self.modules
            .iter()
            .filter_map(|v| (v.1 == kind).then_some(v.0.to_string()))
            .collect()
    }
    pub fn targets(&self) -> Vec<String> {
        self.get_kind(ModuleKind::Target)
    }
    pub fn commits(&self) -> Vec<String> {
        self.get_kind(ModuleKind::Commit)
    }
    pub fn count_commits(&self) -> usize {
        self.modules.iter().filter(|v| v.1 == ModuleKind::Commit).count()
    }
    pub fn clear(&mut self) {
        self.modules.clear();
    }
}
