mod client;
mod collection;
mod database;
mod dump;

use client::Client;
use collection::ModuleCollection;
use database::Database;
use std::collections::HashSet;
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
            Ok(()) => {
                eprintln!("added {code}!", code = m.code());
                self.done.insert(m);
            }
            Err(e) => eprintln!("{}", e),
        }
    }

    fn count(&self) -> usize {
        self.done.len()
    }

    async fn radar(&self) -> Result<Vec<Module>> {
        self.collection
            .radar(&self.current_acad_year, self.done_codes::<Vec<_>>())
            .await
    }
}

/// Checks if the schema defined in the `types` crate works.
async fn check_schema() -> Result<()> {
    let db = init_client().await?.test_db("check_schema");
    let mod_coll = db.modules();
    mod_coll.drop().await?;
    mod_coll.import_academic_year("2021-2022").await?;
    mod_coll.import_academic_year("2022-2023").await?;
    println!("Successful JSON import");
    let modules = mod_coll.list_all().await?;
    println!("Successful struct collect");
    let total = mod_coll.count().await?;
    println!("[check schema] #module.collection->{total}");
    let valids = modules.len() as u64;
    println!("[check schema] #vec<module>->{valids}");
    println!("[check schema] match ? {}", valids == total);
    Ok(())
}

/// Goal: add a radar field to a module
/// Module A is in Module B's radar iff Module A is one of the modules inside of
/// Module B's prereqtree.
async fn create_radar() -> Result<()> {
    let module_code = "MA2101";
    let db = init_db().await?;
    let mods = db.modules();
    let mut graph = Graph::new("2022/2023", mods);
    // graph.add("MA1301X").await;
    graph.add("CS1010").await;
    graph.add("CS1231").await;
    let radar = graph.radar().await?;
    let radar = radar.iter().map(|v| v.code()).collect::<Vec<String>>();
    println!("radar->{:?}", radar);
    Ok(())
}

async fn short() -> Result<()> {
    let db = init_db().await?;
    let mods = db.modules();
    let all_mods = mods.list_all().await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    println!("crates::database!");
    // check_schema().await.unwrap();
    // play().await.unwrap();
    short().await.unwrap();
}
