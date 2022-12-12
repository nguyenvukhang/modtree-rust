mod client;
mod collection;
mod database;
mod dump;

use client::Client;
use collection::ModuleCollection;
use database::Database;
use std::collections::{HashMap, HashSet};
use types::{Module, Result};

async fn init_client() -> Result<Client> {
    let client = Client::new("localhost:27017").await?;
    client.assert_running()?;
    Ok(client)
}

async fn init_db() -> Result<Database> {
    Ok(init_client().await?.modtree_db())
}

async fn play() -> Result<()> {
    let db = init_db().await?;
    let mods = db.modules();
    let mut graph = Graph::new("2022/2023", mods);
    graph.add("MA1301X").await;
    graph.add("CS1010").await;
    graph.add("CS1231").await;
    // graph.add(mods.find_one("CS1010", "2022/2023").await?);
    // graph.add(mods.find_one("CS2040", "2022/2023").await?);
    // let la = mods.find_one("MA2101", "2022/2023").await?;
    // // println!("{:?}", la);
    // graph.add(la);
    // graph.add(mods.find_one("MA2001", "2022/2023").await?);
    // TODO: list the "up next modules"
    // TODO: get smallest number of modules left to unlock for each module
    println!("{}", graph.pretty());
    Ok(())
}

#[derive(Debug)]
struct Graph {
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

/// Goal: add a radar field to a module
/// Module A is in Module B's radar iff Module A is one of the modules inside of
/// Module B's prereqtree.
async fn create_radar() -> Result<()> {
    let module_code = "MA2101";
    let db = init_db().await?;
    let mods = db.modules();
    let mut graph = Graph::new("2022/2023", mods);
    graph.add("MA1301X").await;
    graph.add("CS1010").await;
    graph.add("CS1231").await;
    let radar = graph.radar(5).await?;
    let radar: HashMap<u8, Vec<String>> = radar
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().map(|v| v.code()).collect()))
        .collect();
    println!("radar->{:?}", radar);
    println!("#radar->{:?}", radar.len());
    Ok(())
}

async fn short() -> Result<()> {
    let db = init_db().await?;
    let mods = db.modules();
    mods.drop().await?;
    let module = mods.find_one("ZB3311", "2022/2023").await?;
    println!("mod -> {:?}", module);
    Ok(())
}

#[tokio::main]
async fn main() {
    println!("crates::database!");
    // play().await.unwrap();
    // check_schema().await.unwrap();
    short().await.unwrap();
}
