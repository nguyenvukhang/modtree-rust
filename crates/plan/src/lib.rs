mod debug;
mod plan;
mod structs;

use database::client::Client;
use plan::Plan;

pub async fn main() {
    println!("--- PLAN::main() ---");
    let (_, db) = Client::debug_init().await.unwrap();
    let c = db.modules();
    let module = c.find_one("CS1010", "2022/2023").await.unwrap();
    let mut plan = Plan::new(2021, 1).unwrap();
    plan.add(1, 1, module).unwrap();
    println!("{plan:?}")
}
