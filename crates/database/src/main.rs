mod client;
mod collection;
mod database;
mod dump;

use client::Client;
use types::Result;

async fn mongo() -> Result<()> {
    let mut client = Client::new("localhost:27017").await?;
    client.assert_running()?;
    let db = client.modtree_db();
    let module_collection = db.modules();
    println!("#modules->{}", module_collection.count().await?);
    // module_collection.drop().await?;
    // println!("#modules->{}", module_collection.count().await?);
    // module_collection.import_academic_year("2022-2023").await?;
    // module_collection.import_academic_year("2021-2022").await?;
    let one = module_collection.find_one_module("CS2040", "2022/2023").await?;
    println!("found->{:?}", one);
    // let one = module_collection.find_one_module("CS2040", "2021/2022").await?;
    // println!("found->{:?}", one);
    Ok(())
}

#[tokio::main]
async fn main() {
    println!("crates::database!");
    dump::restore().unwrap();
    // dump::create().unwrap();
    mongo().await.unwrap();
}
