#![allow(unused)]
use crate::collection::ModuleCollection;
use std::collections::{HashMap, HashSet};
use types::{Module, Result};

#[derive(Debug)]
pub struct Graph {
    done: HashSet<Module>,
    /// The academic year used to add new modules with
    current_acad_year: String,
    collection: ModuleCollection,
}

impl Graph {
    fn new(acad_year: &str, module_collection: ModuleCollection) -> Self {
        Self {
            done: HashSet::new(),
            current_acad_year: String::from(acad_year),
            collection: module_collection,
        }
    }

    fn done_codes<T: FromIterator<String>>(&self) -> T {
        self.done.iter().map(|v| v.to_code()).collect()
    }

    fn pretty(&self) -> String {
        format!("Graph {:#?}", self.done_codes::<HashSet<_>>())
    }

    /// Tries to add a module by module code, at the same academic year as self.
    /// Emits feedback.
    async fn add(&mut self, module_code: &str) {
        let acad_year = &self.current_acad_year.to_owned();
        self.add_at_year(module_code, acad_year).await;
    }

    async fn add_at_year(&mut self, module_code: &str, acad_year: &str) {
        let m = match self.collection.find_one(module_code, acad_year).await {
            Ok(v) => v,
            _ => return eprintln!("Unable to fetch module from database."),
        };
        let done = self.done.iter().map(|v| v.to_code()).collect();
        match m.satisfied_by(&done) {
            true => {
                eprintln!("added {code}!", code = m.code());
                self.done.insert(m);
            }
            false => eprintln!(
                "{} pre-requisites not satisfied -> {:?}",
                m.code(),
                m.prereqtree()
            ),
        }
    }

    fn count(&self) -> usize {
        self.done.len()
    }
}
