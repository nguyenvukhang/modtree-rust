use crate::structs::*;
use std::collections::HashMap;
use types::{Error, Module, Result};

pub struct Plan {
    matric: Period,
    semesters: HashMap<Period, ModuleList>,
}

impl Plan {
    pub fn new(matric_year: i32, matric_sem: i32) -> Result<Self> {
        let matric = Period::new(matric_year, matric_sem)?;
        Ok(Self { matric, semesters: HashMap::new() })
    }

    /// Tries to add a module to a particular semester. Returns true if the
    /// module was actually added.
    pub fn add(&mut self, year: i32, sem: i32, module: Module) -> Result<bool> {
        if !module.is_offered_in_sem(sem) {
            return Err(Error::ModuleNotOfferedInSem(module.code(), sem));
        }
        let period = Period::new(year, sem)?;
        if let Some(mod_list) = self.semesters.get_mut(&period) {
            Ok(mod_list.insert(module))
        } else {
            self.semesters.insert(period, ModuleList::with_one(module));
            Ok(true)
        }
    }

    pub fn matric(&self) -> Period {
        self.matric
    }

    pub fn semesters(&self) -> &HashMap<Period, ModuleList> {
        &self.semesters
    }
}
