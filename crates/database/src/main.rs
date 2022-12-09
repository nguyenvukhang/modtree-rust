mod types;

use mongodb::bson::{doc, Document};
use mongodb::options::{
    ClientOptions, Credential, DropDatabaseOptions, FindOptions, ServerAddress,
};
use mongodb::{Client, Database};
use std::env;
use std::error;

use types::Book;

type BoxErr = Box<dyn error::Error>;

fn some_string(s: &str) -> Option<String> {
    Some(s.to_string())
}

async fn get_client() -> Result<Client, BoxErr> {
    dotenv::dotenv().expect(".env file not found");
    let host = ServerAddress::parse("localhost:27017")?;
    let creds = Credential::builder()
        .username(env::var("MONGO_DB_USERNAME").ok())
        .password(env::var("MONGO_DB_PASSWORD").ok())
        .build();
    let opts = ClientOptions::builder()
        .hosts(vec![host])
        .credential(creds)
        .build();
    println!("opts->{:?}", opts);
    Ok(Client::with_options(opts)?)
}

async fn list_databases(client: &mut Client) -> Result<(), BoxErr> {
    println!("Listing databases...");
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }
    Ok(())
}

async fn insert_data(db: &mut Database) -> Result<(), BoxErr> {
    let collection = db.collection::<Document>("books");
    let docs = vec![
        doc! { "title": "1984", "author": "George Orwell" },
        doc! { "title": "Animal Farm", "author": "George Orwell" },
        doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
    ];
    collection.insert_many(docs, None).await?;
    Ok(())
}

async fn list_collections(db: &Database) -> Result<(), BoxErr> {
    println!("Listing collections...");
    for collection_name in db.list_collection_names(None).await? {
        println!("{}", collection_name);
    }
    Ok(())
}

/// Deletes all databases except for "admin", "config", and "local"
async fn reset_all_databases(client: &mut Client) -> Result<(), Box<dyn error::Error>> {
    let databases = client.list_database_names(None, None).await?;
    let to_delete = databases
        .iter()
        .filter(|name| match name.as_ref() {
            "admin" | "config" | "local" => false,
            _ => true,
        })
        .map(|name| client.database(name));
    for db in to_delete {
        println!("deleting database [{}]", db.name());
        db.drop(DropDatabaseOptions::builder().build()).await?;
    }
    Ok(())
}

async fn mongo() -> Result<(), Box<dyn error::Error>> {
    let mut client = get_client().await.unwrap();
    list_databases(&mut client).await.unwrap();
    reset_all_databases(&mut client).await.unwrap();
    // let mut db = client.database("hello");
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
