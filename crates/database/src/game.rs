#![allow(unused)]
use crate::collection::ModuleCollection;
use std::collections::{HashMap, HashSet};
use types::{Module, Result};

pub async fn play() -> Result<()> {
    // let db = init_db().await?;
    // let mods = db.modules();
    // let mut graph = Graph::new("2022/2023", mods);
    // graph.add("MA1301X").await;
    // graph.add("CS1010").await;
    // graph.add("CS1231").await;
    // graph.add(mods.find_one("CS1010", "2022/2023").await?);
    // graph.add(mods.find_one("CS2040", "2022/2023").await?);
    // let la = mods.find_one("MA2101", "2022/2023").await?;
    // // println!("{:?}", la);
    // graph.add(la);
    // graph.add(mods.find_one("MA2001", "2022/2023").await?);
    // TODO: list the "up next modules"
    // TODO: get smallest number of modules left to unlock for each module
    // println!("{}", graph.pretty());
    Ok(())
}

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
        self.done.iter().map(|v| v.code()).collect()
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
        let done = self.done.iter().map(|v| v.code()).collect();
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

    async fn radar(
        &self,
        max_distance: u8,
    ) -> Result<HashMap<u8, Vec<Module>>> {
        self.collection
            .radar(
                &self.current_acad_year,
                self.done_codes::<HashSet<_>>(),
                max_distance,
            )
            .await
    }
}
