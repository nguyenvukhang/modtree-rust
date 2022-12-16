use crate::structs::*;
use database::collection::ModuleCollection;
use std::collections::HashSet;
use std::fmt;
use std::hash::Hash;
use types::{Error, Result};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum ModuleKind {
    Commit,
    Target,
}

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
}

#[derive(Clone)]
pub struct Plan {
    semesters: Vec<Semester>,
    /// start state
    matric: Period,
    /// Source of data
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
        Ok(Self {
            matric,
            // 5 years * 4 sems
            semesters: vec![Semester::new(5); 20],
            src,
        })
    }

    /// Sets a target module. The "I want to do this module in the future, by
    /// this year, by this semester."
    pub fn target(&mut self, year: i32, sem: i32, code: &str) -> Result<bool> {
        let semester = self.get_semester(year, sem)?;
        Ok(semester.insert(code, ModuleKind::Target))
    }

    /// Commits to a module. The "I will do/have done this module, by this year,
    /// by this semester."
    pub fn commit(&mut self, year: i32, sem: i32, code: &str) -> Result<bool> {
        let semester = self.get_semester(year, sem)?;
        Ok(semester.insert(code, ModuleKind::Commit))
    }

    pub fn get_targets(&self) -> Vec<String> {
        self.semesters
            .iter()
            .flat_map(|sem| {
                sem.modules.iter().filter_map(|v| match v.1 {
                    ModuleKind::Target => Some(v.0.to_string()),
                    _ => None,
                })
            })
            .collect()
    }

    pub fn get_commits(&self) -> Vec<String> {
        self.semesters
            .iter()
            .flat_map(|sem| {
                sem.modules.iter().filter_map(|v| match v.1 {
                    ModuleKind::Commit => Some(v.0.to_string()),
                    _ => None,
                })
            })
            .collect()
    }

    pub async fn fill(&self) -> Result<Self> {
        let plan = self.to_owned();
        let targets = plan.get_targets();
        let commits = plan.get_commits();
        let acad_year = self.matric.acad_year();

        // sample space of all the modules related to the target modules
        let mut sample_space =
            self.src.flatten_requirements(targets, &acad_year).await?;

        // remove the modules that are already committed
        // TODO: uncomment the next line
        // sample_space.retain(|v| !commits.contains(v.code()));
        
        println!(
            "sample space -> {:?}",
            sample_space.iter().map(|v| v.code()).collect::<Vec<_>>()
        );

        let sample_space =
            sample_space.into_iter().map(|m| (m.to_code(), m)).collect();
        let sorted = ModuleCollection::topological_sort(sample_space);
        println!(
            "topo sorted -> {:?}",
            sorted.iter().map(|v| &v.0).collect::<Vec<_>>()
        );

        // sort sample_space by topological order
        // poll this queue while populating the `plan`
        // remember to check for sem availability on each module

        // println!("target -> {targets:?}");
        println!("commits -> {commits:?}");
        Ok(plan)
    }

    /// get the index of `self.road` from year and sem.
    fn get_semester(&mut self, year: i32, sem: i32) -> Result<&mut Semester> {
        let raw = (year - 1) * 4 + sem - 1;
        if !(0 <= raw && raw < 20) {
            Err(Error::InvalidData(format!("year: {year}, sem: {sem}")))
        } else {
            let idx = raw as usize;
            self.semesters.get_mut(idx).ok_or(Error::InvalidIndex(idx, 0, 19))
        }
    }

    pub fn acad_year(&self, year: i32) -> String {
        let base = self.matric.year() + year - 1;
        format!("{}/{}", base, base + 1)
    }

    pub fn semesters(&self) -> &Vec<Semester> {
        &self.semesters
    }

    /// Get the matriculation year.
    pub fn matric(&self) -> Period {
        self.matric
    }
}
