#![allow(unused)]

use crate::structs::ModuleKind;
use std::collections::HashSet;
use std::fmt;

#[derive(Default, Clone)]
pub struct Semester {
    modules: HashSet<(String, ModuleKind)>,
    limit: usize,
}

impl fmt::Debug for Semester {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Sem")
            .field(&format!("mods[{}]", self.limit), &self.modules)
            .finish()
    }
}

impl Semester {
    pub fn new(limit: usize) -> Self {
        Self { modules: HashSet::new(), limit }
    }
    pub fn insert(&mut self, code: &str, kind: ModuleKind) -> bool {
        self.modules.insert((code.to_string(), kind))
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
