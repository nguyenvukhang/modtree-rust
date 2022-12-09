use mongodb::bson::{doc, Document};
use mongodb::options::{ClientOptions, Credential, ServerAddress};
use mongodb::{Client, Database};
use std::error;

type BoxErr = Box<dyn error::Error>;

fn some_string(s: &str) -> Option<String> {
    Some(s.to_string())
}

async fn get_client() -> Result<Client, BoxErr> {
    let host = ServerAddress::parse("localhost:27017")?;
    let creds = Credential::builder()
        .username(some_string("modtree"))
        .password(some_string("modtree"))
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

async fn mongo() -> Result<(), Box<dyn error::Error>> {
    let mut client = get_client().await.unwrap();
    list_databases(&mut client).await.unwrap();
    let mut db = client.database("hello");
    db.create_collection("books", None).await.unwrap();
    insert_data(&mut db).await?;
    list_collections(&db).await?;
    // List the names of the databases in that deployment.
    Ok(())
}

#[tokio::main]
async fn main() {
    println!("crates::database!");
    mongo().await.ok();
}
