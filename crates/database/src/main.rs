mod client;
mod collection;
mod database;
mod dump;

use client::Client;
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
    let mut graph = Graph::new();
    graph.add(mods.find_one("CS2040", "2022/2023").await?);
    graph.add(mods.find_one("CS2040", "2021/2022").await?);
    graph.add(mods.find_one("CS1010", "2021/2022").await?);
    graph.add(mods.find_one("CS2040", "2021/2022").await?);
    // TODO: list the "up next modules"
    // TODO: get smallest number of modules left to unlock for each module
    println!("#graph->{}", graph.count());
    Ok(())
}

#[derive(Debug)]
struct Graph {
    done: HashSet<Module>,
}

impl Graph {
    fn new() -> Self {
        Self { done: HashSet::new() }
    }
    fn add(&mut self, module: Module) {
        let done = self.done.iter().map(|v| v.code()).collect();
        match module.satisfied_by(&done) {
            Ok(()) => {
                eprintln!("added {code}!", code = module.code());
                self.done.insert(module);
            }
            Err(e) => eprintln!("{}", e),
        }
    }
    fn count(&self) -> usize {
        self.done.len()
    }
}

async fn check_schema() -> Result<()> {
    let db = init_client().await?.test_db("check_schema");
    let mod_coll = db.modules();
    mod_coll.drop().await?;
    mod_coll.import_academic_year("2021-2022").await?;
    mod_coll.import_academic_year("2022-2023").await?;
    println!("Successful JSON import");
    let modules = mod_coll.list_all().await?;
    println!("Successful struct collect");
    let count = mod_coll.count().await?;
    println!("[check schema] #module.collection->{count}");
    let count = modules.len();
    println!("[check schema] #vec<module>->{count}");
    Ok(())
}

#[tokio::main]
async fn main() {
    println!("crates::database!");
    // check_schema().await.unwrap();
    play().await.unwrap();
}
