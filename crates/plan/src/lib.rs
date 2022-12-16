mod plan;
mod semester;
mod structs;

use database::client::Client;
use plan::PlanBuilder;

pub async fn main() {
    println!("--- PLAN::main() ---");
    let (_, db) = Client::debug_init().await.unwrap();
    let plan = PlanBuilder::new(2022, 4)
        .commit(1, 1, "CS1010")
        .target(3, 1, "CS3244")
        .build(&db.modules())
        .await;
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    // plan.min_path().await.unwrap();
    println!("plan:{plan:?}");
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
}
