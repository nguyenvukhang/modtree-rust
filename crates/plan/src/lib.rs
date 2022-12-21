#![allow(unused)]

mod builder;
mod plan;
mod semester;
mod structs;

use builder::PlanBuilder;
use database::client::Client;
// use plan::Plan;
// use std::collections::BinaryHeap;

pub async fn main() {
    // println!("--- PLAN::main() ---");
    // let (_, db) = Client::debug_init().await.unwrap();
    // let (plan, sample_space) = PlanBuilder::new(2022, 4)
    //     .commit(1, 1, "CS1010")
    //     .target(3, 1, "CS2103T")
    //     .build(&db.modules())
    //     .await;
    // println!("plan->{plan:?}");
    // println!("sample_space->{sample_space:?}");
}
