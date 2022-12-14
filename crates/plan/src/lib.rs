mod debug;
mod plan;
mod structs;

use database::client::Client;
use plan::Plan;

pub async fn main() {
    println!("--- PLAN::main() ---");
    let (_, db) = Client::debug_init().await.unwrap();
    let mut plan = Plan::new(2021, 1, db.modules()).unwrap();
    plan.commit(1, 1, "CS1010").unwrap();
    plan.target(3, 1, "CS3244").unwrap();
    let filled = plan.fill().await;
    // plan.add(1, 1, "CS1010").await.unwrap();
    // plan.remove(1, 1, "CS2010").unwrap();
    // plan.remove(1, 1, "CS1010").unwrap();
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("{filled:?}");
}
