mod debug;
mod plan;
mod structs;

use database::client::Client;
use plan::Plan;

pub async fn main() {
    println!("--- PLAN::main() ---");
    let (_, db) = Client::debug_init().await.unwrap();
    let mut plan = Plan::new(2021, 1, db.modules()).unwrap();
    plan.add(1, 1, "CS1010").await.unwrap();
    // plan.remove(1, 1, "CS2010").unwrap();
    // plan.remove(1, 1, "CS1010").unwrap();
    println!("{plan:?}")
}
