mod client;
mod database;

use client::Client;
use database::Database;
use fetcher::Loader;
use types::{Module, Result};

/// Loads all modules into the modules collection
async fn load_all_modules(db: &Database) -> Result<()> {
    use mongodb::bson::{doc, to_document};
    use mongodb::options::UpdateOptions;
    let loader = Loader::new()?;
    let module_list = loader.load_module_list().await?;
    let code_list: Vec<String> = module_list.iter().map(|v| v.code()).collect();
    let mut loaded_modules = vec![];
    // load all details
    for code in code_list {
        let val = loader.load_module(&code).await?;
        loaded_modules.push(Module::from(val));
    }
    let handles = loaded_modules.iter().map(|module| async move {
        let mut doc = to_document(&module).unwrap();
        doc.remove("_id");
        let res = db
            .modules()
            .update_one(
                doc! {
                    "module_code": module.code(),
                    "acad_year": module.academic_year()
                },
                // TODO: uniquely identify a module by semester too.
                doc! { "$set": doc },
                UpdateOptions::builder().upsert(true).build(),
            )
            .await;
        println!("code->{}", module.code());
        res
    });
    futures::future::join_all(handles).await;
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
    // remove_all_modules(&db).await?;
    load_all_modules(&db).await?;
    count_all_modules(&db).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    println!("crates::database!");
    mongo().await.unwrap();
}
