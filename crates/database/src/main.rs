mod client;
mod database;

use client::Client;
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

async fn mongo() -> Result<()> {
    let mut client = Client::new().await?;
    let databases = client.list_all_database_names().await?;
    // TODO: insert 1 module -> DONE
    // TODO: remove all modules
    println!("databases -> {:?}", databases);
    let db = client.modtree_db().await?;
    let modules = db.collection::<types::Module>("modules");
    use futures::stream::StreamExt;
    let mut cursor = modules.find(None, None).await?;
    for i in cursor.next().await {
        println!("mod->{i:?}");

    }
    // let loader = Loader::new()?;
    // let (module, _) = loader.load_module("CS2040").await?;
    // println!("module -> {module:?}");

    // collection.insert_one(Module::from(module), None).await?;
    // collection.insert_many(vec![module], None);
    // // insert_data(&mut db).await?;
    // list_collections(&db).await?;
    // let filter = doc! { "author": "George Orwell" };
    // let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
    // let mut cursor = db
    //     .collection::<Book>("books")
    //     .find(filter, find_options)
    //     .await?;
    // // Iterate over the results of the cursor.
    // while let Some(book) = cursor.try_next().await? {
    //     println!("title: {}", book.title);
    // }
    // // List the names of the databases in that deployment.
    Ok(())
}

#[tokio::main]
async fn main() {
    println!("crates::database!");
    mongo().await.unwrap();
}
