use crate::structs::*;
use database::collection::ModuleCollection;
use std::collections::HashMap;
use types::{Error, Module, Result};

pub struct Plan {
    matric: Period,
    semesters: HashMap<Period, ModuleList>,
    src: ModuleCollection,
}

impl Plan {
    /// Create a new plan.
    pub fn new(
        matric_year: i32,
        matric_sem: i32,
        src: ModuleCollection,
    ) -> Result<Self> {
        let matric = Period::new(matric_year, matric_sem)?;
        Ok(Self { matric, semesters: HashMap::new(), src })
    }

    /// Get the matriculation year.
    pub fn matric(&self) -> Period {
        self.matric
    }

    /// Get a reference to the semesters.
    pub fn semesters(&self) -> &HashMap<Period, ModuleList> {
        &self.semesters
    }

    /// Tries to add a module to a particular semester. Returns true if the
    /// module was actually added.
    pub async fn add(
        &mut self,
        year: i32,
        sem: i32,
        module_code: &str,
    ) -> Result<bool> {
        let y = self.matric.year() + year - 1;
        let acad_year = format!("{}/{}", y, y + 1);
        let module = self.src.find_one(module_code, &acad_year).await?;
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

    /// Tries to remove a module from a particular semester. Returns true if the
    /// module was actually removed.
    pub fn remove(&mut self, year: i32, sem: i32, code: &str) -> Result<bool> {
        let period = Period::new(year, sem)?;
        let result = if let Some(mod_list) = self.semesters.get_mut(&period) {
            Ok(mod_list.remove(code))
        } else {
            Ok(false)
        };
        if self.semesters.get(&period).map_or(false, |v| v.len() == 0) {
            self.semesters.remove(&period);
        }
        result
    }
}
