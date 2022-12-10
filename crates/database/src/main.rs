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

async fn mongo() -> Result<()> {
    let db = init_db().await?;
    let module_coll = db.modules();
    let mut graph = Graph::new();
    graph.add(module_coll.find_one("CS2040", "2022/2023").await?.unwrap());
    graph.add(module_coll.find_one("CS2040", "2022/2023").await?.unwrap());
    graph.add(module_coll.find_one("CS2040", "2021/2022").await?.unwrap());
    // TODO: need to check if a module is add-able to graph by its prereqTree
    println!("graph->{}", graph.module_count());
    Ok(())
}

#[derive(Debug)]
struct Graph {
    modules: HashSet<Module>,
}

impl Graph {
    fn new() -> Self {
        Self { modules: HashSet::new() }
    }
    fn add(&mut self, module: Module) {
        self.modules.insert(module);
    }
    fn module_count(&self) -> usize {
        self.modules.len()
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
    check_schema().await.unwrap();
    // dump::restore().unwrap();
    // dump::create().unwrap();
}
