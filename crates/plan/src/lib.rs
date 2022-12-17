mod plan;
mod semester;
mod structs;

use database::client::Client;
use plan::{Plan, PlanBuilder};
use std::collections::{BinaryHeap, HashSet};

pub async fn main() {
    println!("--- PLAN::main() ---");
    let (_, db) = Client::debug_init().await.unwrap();
    let plan = PlanBuilder::new(2022, 4)
        .commit(1, 1, "CS1010")
        .target(3, 1, "CS2103T")
        .build(&db.modules())
        .await;
    let commits = plan.commit_set();
    let data = plan.data();
    let mut pq: BinaryHeap<Plan> = BinaryHeap::new();
    pq.push(plan);
    let mut done = HashSet::<Plan>::new();
    let mut seen = HashSet::<Plan>::new();
    while let Some(mut plan) = pq.pop() {
        if !plan.has_remaining() {
            done.insert(plan);
            continue;
        }
        plan.topo_sort();
        let mut code = plan.pop();
        while commits.contains(&code) {
            code = plan.pop();
        }
        let (tree, sems) = data.get(&code).unwrap();
        let mut next = plan.fork(&code, tree, sems);
        next.retain(|v| !seen.contains(v));
        seen.extend(next.clone());
        pq.extend(next);
        // println!("{code} -> {} , {}", pq.len(), done.len());
    }
    for done in done {
        println!("{done:?}");
        println!("~ done ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    }
}
