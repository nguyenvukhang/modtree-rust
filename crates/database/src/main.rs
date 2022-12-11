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
    graph.add(mods.find_one("CS1010", "2022/2023").await?);
    graph.add(mods.find_one("CS2040", "2022/2023").await?);
    let la = mods.find_one("MA2101", "2022/2023").await?;
    // println!("{:?}", la);
    graph.add(la);
    graph.add(mods.find_one("MA2001", "2022/2023").await?);
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
    let total = mod_coll.count().await?;
    println!("[check schema] #module.collection->{total}");
    let valids = modules.len() as u64;
    println!("[check schema] #vec<module>->{valids}");
    println!("[check schema] match ? {}", valids == total);
    Ok(())
}

async fn short_test() -> Result<()> {
    let module_code = "MA2101";
    let db = init_db().await?;
    let mods = db.modules();
    // mods.drop().await?;
    // mods.import_one("2022-2023", module_code).await?;
    mods.import_academic_year("2022-2023").await?;
    let all = mods.list_all().await?;
    let codes: Vec<_> = all.iter().map(|v| v.code()).collect();
    println!("all->{codes:?}");
    let la = mods.find_one(module_code, "2022/2023").await?;
    println!("{module_code}.prereqtree->{:?}", la.prereqtree());
    Ok(())
}

#[tokio::main]
async fn main() {
    println!("crates::database!");
    // check_schema().await.unwrap();
    play().await.unwrap();
    // short_test().await.unwrap();
}
