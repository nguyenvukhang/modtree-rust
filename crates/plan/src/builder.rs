use crate::plan::Plan;
use crate::semester::Semester;
use crate::structs::*;
use database::collection::ModuleCollection;
use std::collections::HashMap;
use std::mem;
use types::{Module, Result};

pub struct PlanBuilder {
    matric_year: usize,
    years: usize,
    sems: Vec<Vec<Semester>>,
    targets: Vec<String>,
}

impl PlanBuilder {
    /// Create a new plan.
    pub fn new(matric_year: usize, years: usize) -> Self {
        Self {
            targets: vec![],
            matric_year,
            years,
            sems: vec![vec![Semester::new(5); 4]; years],
        }
    }

    /// Commits to a module. The "I will do/have done this module, by this year,
    /// by this semester."
    pub fn commit(
        &mut self,
        year: usize,
        sem: usize,
        module_code: &str,
    ) -> &mut Self {
        self.sems[year][sem].insert(module_code, ModuleKind::Commit);
        self
    }

    /// Sets a target module. The "I want to do this module in the future, by
    /// this year, by this semester."
    pub fn target(
        &mut self,
        year: usize,
        sem: usize,
        module_code: &str,
    ) -> &mut Self {
        self.targets.push(module_code.to_string());
        self.sems[year][sem].insert(module_code, ModuleKind::Target);
        self
    }

    /// Gets all the modules that are connected to these codes.
    async fn get_sample_space(
        collection: &ModuleCollection,
        codes: Vec<String>,
        acad_year: &str,
    ) -> Result<HashMap<String, Module>> {
        let list = collection.flatten_requirements(codes, &acad_year).await?;
        Ok(list.into_iter().map(|m| (m.to_code(), m)).collect())
    }

    // Builds a plan.
    // pub async fn build(
    //     &mut self,
    //     collection: &ModuleCollection,
    // ) -> (Plan, HashMap<String, Module>) {
    //     let y = self.matric_year;
    //     let acad_year = format!("{}/{}", y, y + 1);
    //     let targets = mem::take(&mut self.targets);
    //     let semesters = mem::take(&mut self.sems);
    //     (
    //         Plan::new(self.years, semesters),
    //         Self::get_sample_space(collection, targets, &acad_year)
    //             .await
    //             .unwrap(),
    //     )
    // }
}
