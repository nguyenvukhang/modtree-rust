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
    let all = mods.list_all().await?;
    for module in all {
        // if module.semesters.len() == 3 {
        //     println!("{module:?}");
        // }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    println!("crates::database!");
    dump::test_schema().await;
    // test().await.unwrap();
}
