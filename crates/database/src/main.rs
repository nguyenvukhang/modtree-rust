mod client;
mod collection;
mod database;
mod dump;
mod game;

use client::Client;
use types::Result;

async fn test() -> Result<()> {
    let (_, db) = Client::debug_init().await?;
    let mods = db.modules();
    // let module = mods.find_one("CS3244", "2022/2023").await?;
    // println!("mod -> {:?}", module);
    mods.min_path("CS3244", "2022/2023").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    println!("crates::database!");
    test().await.unwrap();
}
