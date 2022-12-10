mod client;
mod collection;
mod database;

use client::Client;
use database::Database;
use fetcher::Loader;
use types::Result;

async fn import_modules(db: &Database) -> Result<()> {
    let module_collection = db.modules();
    module_collection.drop().await?;
    let loader = Loader::new()?;
    println!("Loading modules from JSON...");
    let modules = loader.load_all_modules().await?;
    println!("Done loading all modules from JSON");
    println!("Inserting modules to mongo-db...");
    module_collection.insert_many_modules(&modules).await;
    println!("Done.");
    Ok(())
}

async fn mongo() -> Result<()> {
    let mut client = Client::new("localhost:27017").await?;
    client.assert_running()?;
    let db = client.modtree_db();
    let module_collection = db.modules();
    println!("modules in database->{}", module_collection.count().await?);
    let one = module_collection.find_one_module("CS2040", "2022/2023").await?;
    println!("found->{:?}", one);
    Ok(())
}

#[tokio::main]
async fn main() {
    println!("crates::database!");
    mongo().await.unwrap();
}
