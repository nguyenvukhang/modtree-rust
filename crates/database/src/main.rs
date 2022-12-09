use mongodb::{options::ClientOptions, Client};
use std::error;

async fn mongo() -> Result<(), Box<dyn error::Error>> {
    let url = "mongodb://localhost:27017";
    let opts = ClientOptions::parse(&url).await?;
    let client = Client::with_options(opts);
    println!("client->{:?}", client);
    Ok(())
}

#[tokio::main]
async fn main() {
    println!("crates::database!");
    mongo().await.ok();
}
