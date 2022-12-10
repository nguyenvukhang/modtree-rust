mod client;
mod database;

use client::Client;
use database::Database;
use fetcher::Loader;
use types::{Module, Result};

// async fn insert_data(db: &mut Database) -> Result<(), BoxErr> {
//     let collection = db.collection::<Document>("books");
//     let docs = vec![
//         doc! { "title": "1984", "author": "George Orwell" },
//         doc! { "title": "Animal Farm", "author": "George Orwell" },
//         doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
//     ];
//     collection.insert_many(docs, None).await?;
//     Ok(())
// }
//
// async fn list_collections(db: &Database) -> Result<(), BoxErr> {
//     println!("Listing collections...");
//     for collection_name in db.list_collection_names(None).await? {
//         println!("{}", collection_name);
//     }
//     Ok(())
// }

/// Loads all modules into the modules collection
async fn load_all_modules(db: &Database) -> Result<()> {
    let loader = Loader::new()?;
    let module_list = loader.load_module_list().await?;
    let code_list: Vec<String> = module_list.iter().map(|v| v.code()).collect();
    let mut to_insert: Vec<Module> = vec![];
    for code in code_list {
        println!("code->{code}");
        let details = loader.load_module(&code).await?;
        to_insert.push(Module::from(details));
    }
    let modules = db.modules();
    modules.insert_many(to_insert, None).await?;
    Ok(())
}

/// Lists all modules into the modules collection
async fn list_all_modules(db: &Database) -> Result<()> {
    let modules = db.modules();
    use futures::stream::StreamExt;
    let mut cursor = modules.find(None, None).await?;
    for module in cursor.next().await {
        println!("mod->{:?}", module.map(|v| v.code()));
    }
    Ok(())
}

/// Counts all modules into the modules collection
async fn count_all_modules(db: &Database) -> Result<()> {
    let modules = db.modules();
    let count = modules.count_documents(None, None).await?;
    println!("module count: {count}");
    Ok(())
}

/// Removes all modules into the modules collection
async fn remove_all_modules(db: &Database) -> Result<()> {
    let modules = db.modules();
    modules.drop(None).await?;
    Ok(())
}

async fn mongo() -> Result<()> {
    let mut client = Client::new("localhost:27017").await?;
    client.assert_running()?;
    // let databases = client.list_all_database_names().await.unwrap();
    // println!("databases -> {:?}", databases);
    let db = client.modtree_db().await;
    remove_all_modules(&db).await?;
    // load_all_modules(&db).await?;
    count_all_modules(&db).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    println!("crates::database!");
    mongo().await.unwrap();
}
