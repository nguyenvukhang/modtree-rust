use crate::semester::Semester;
use crate::structs::*;
use database::collection::ModuleCollection;
use prereqtree::PrereqTree;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem;

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
        let mut offered = HashMap::new();
        for module in sample_space {
            remain.push((module.to_code(), module.prereqtree()));
            offered.insert(module.to_code(), module.to_semesters());
        }
        let semesters = mem::take(&mut self.semesters);
        Plan { years: self.years, length: 0, remain, semesters, offered }
    }
}

#[derive(Clone)]
pub struct Plan {
    years: usize,
    semesters: Vec<Semester>,
    remain: Vec<(String, PrereqTree)>,
    offered: HashMap<String, Vec<usize>>,
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

    pub fn commit_set(&self) -> HashSet<String> {
        self.semesters.iter().flat_map(|s| s.commits()).collect()
    }

    pub fn data(&self) -> HashMap<String, (PrereqTree, Vec<usize>)> {
        self.remain
            .iter()
            .map(|(code, tree)| {
                (
                    code.to_owned(),
                    (tree.clone(), self.offered.get(code).unwrap().clone()),
                )
            })
            .collect()
    }

    /// Marks a module as done in all prereqtrees of the remaining modules.
    fn resolve(&mut self, module_code: &str) {
        self.remain.iter_mut().for_each(|r| r.1.resolve(module_code));
    }

    pub fn topo_sort(&mut self) {
        let list = std::mem::take(&mut self.remain);
        self.remain = PrereqTree::topological_sort(list);
    }

    pub fn has_remaining(&self) -> bool {
        !self.remain.is_empty()
    }

    pub fn pop(&mut self) -> String {
        let (code, _) = self.remain.swap_remove(0);
        self.resolve(&code);
        self.topo_sort();
        code
    }

    pub fn flat(&self) -> Vec<String> {
        self.semesters.iter().flat_map(|v| v.commits()).collect()
    }

    /// Inserts a module at all possible year/sem combinations, checking for
    /// 1. prereqtree satisfied
    /// 2. offered in semester
    ///
    /// Precondition: module_code is already removed from `self.remain`
    pub fn fork(
        &self,
        module_code: &str,
        tree: &PrereqTree,
        semesters_offered: &Vec<usize>,
    ) -> Vec<Plan> {
        let mut tree = tree.clone();
        let mut plans = vec![];
        for i in 0..self.semesters.len() {
            // load committed modules of that sem and mark those as done
            self.semesters[i].commits().iter().for_each(|c| tree.resolve(c));
            // check if module is offered in sem
            let sem = i % 4 + 1;
            if !semesters_offered.contains(&sem) {
                continue;
            }
            // insert if module is satisfied
            if tree.is_empty() {
                let mut plan = self.clone();
                plan.semesters[i].insert(&module_code, ModuleKind::Commit);
                plans.push(plan)
            }
        }
        plans
    }
}

/// sort in order of decreasing length
impl PartialOrd for Plan {
    fn partial_cmp(&self, rhs: &Plan) -> Option<Ordering> {
        match rhs.len().cmp(&self.len()) {
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
impl Hash for Plan {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.semesters.hash(state);
    }
}
impl fmt::Debug for Plan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.semesters)
    }
}
