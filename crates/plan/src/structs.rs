use std::collections::HashSet;
use std::hash::Hash;
use types::{Error, Module, Result};

#[derive(Debug)]
pub struct ModuleList(HashSet<Module>);

impl ModuleList {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn with_one(m: Module) -> Self {
        Self(HashSet::from_iter(vec![m]))
    }

    /// Adds a new module to the list. Returns true on success.
    pub fn insert(&mut self, m: Module) -> bool {
        self.0.insert(m)
    }

    /// Removes a new module to the list. Returns true on success.
    pub fn remove(&mut self, code: &str) -> bool {
        let len = self.0.len();
        self.0.retain(|v| !v.code().eq(code));
        self.0.len() < len
    }
}

/// Period(<year>, <semester>)
#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Period(i32, i32);

impl Period {
    pub fn new(year: i32, sem: i32) -> Result<Self> {
        if !(1 <= sem && sem <= 4) {
            Err(Error::InvalidSemester)
        } else {
            Ok(Self(year, sem))
        }
    }
    pub fn year(&self) -> i32 {
        self.0
    }
    pub fn sem(&self) -> i32 {
        self.1
    }
}
