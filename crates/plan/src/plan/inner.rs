use crate::semester::Semester;
use crate::structs::*;
use database::collection::ModuleCollection;
use prereqtree::PrereqTree;
use std::cmp::Ordering;
use types::Module;

pub struct PlanBuilder {
    matric_year: usize,
    years: usize,
    semesters: Vec<Semester>,
}

impl PlanBuilder {
    /// Create a new plan.
    pub fn new(matric_year: usize, years: usize) -> Self {
        Self {
            matric_year,
            years,
            semesters: vec![Semester::new(5); years * 4],
        }
    }

    fn sem(&mut self, year: usize, sem: usize) -> &mut Semester {
        &mut self.semesters[year * 4 + sem - 5]
    }

    /// Commits to a module. The "I will do/have done this module, by this year,
    /// by this semester."
    pub fn commit(
        &mut self,
        year: usize,
        sem: usize,
        module_code: &str,
    ) -> &mut Self {
        self.sem(year, sem).insert(module_code, ModuleKind::Commit);
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
        self.sem(year, sem).insert(module_code, ModuleKind::Target);
        self
    }

    /// Fully loads the `remain` field with full prereqtrees from database. Used
    /// to make the root node in a Dijkstra search.
    pub async fn build(
        &mut self,
        module_collection: &ModuleCollection,
    ) -> Plan {
        let y = self.matric_year;
        let acad_year = format!("{}/{}", y, y + 1);
        let sample_space = module_collection
            .flatten_requirements(
                self.semesters.iter().flat_map(|s| s.targets()).collect(),
                &acad_year,
            )
            .await
            .unwrap();
        let mut remain = vec![];
        for module in sample_space {
            remain.push((module.to_code(), module.prereqtree()))
        }
        let semesters = std::mem::take(&mut self.semesters);
        Plan { years: self.years, length: 0, remain, semesters }
    }
}

#[derive(Clone, Debug)]
pub struct Plan {
    years: usize,
    semesters: Vec<Semester>,
    remain: Vec<(String, PrereqTree)>,
    length: usize,
}

impl Plan {
    /// Get the number of semesters required to complete the plan.
    pub fn len(&self) -> usize {
        self.length
    }

    fn sem(&mut self, year: usize, sem: usize) -> &mut Semester {
        &mut self.semesters[year * 4 + sem - 5]
    }
}

fn sem_idx(year: usize, sem: usize) -> usize {
    year * 4 + sem - 5
}

impl PartialOrd for Plan {
    fn partial_cmp(&self, rhs: &Plan) -> Option<Ordering> {
        match self.len().cmp(&rhs.len()) {
            Ordering::Equal => None,
            v => Some(v),
        }
    }
}
impl Ord for Plan {
    fn cmp(&self, rhs: &Plan) -> Ordering {
        self.len().cmp(&rhs.len())
    }
}
impl PartialEq for Plan {
    fn eq(&self, rhs: &Plan) -> bool {
        self.len() == rhs.len()
    }
}
impl Eq for Plan {}
