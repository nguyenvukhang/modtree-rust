use crate::plan::Plan;
use database::collection::ModuleCollection;
use std::collections::BinaryHeap;
use types::Result;

impl Plan {
    // pub async fn min_path(&self) -> Result<()> {
    //     let mut plan = self.to_owned();
    //     let targets = plan.get_targets();
    //     let acad_year = self.acad_year(1);
    //
    //     // sample space of all the modules related to the target modules
    //     let sample_space =
    //         self.src.flatten_requirements(targets, &acad_year).await?;
    //
    //     let mut pq: BinaryHeap<Plan> = BinaryHeap::new();
    //     plan.clear();
    //     pq.push(plan);
    //
    //     for module in sample_space {
    //         let code = module.to_code();
    //         let sems = module.semesters();
    //         // generate a list of all valid year/sem combinations
    //         // check if prereqs are fulfilled at each juncture
    //         // insert them into a plan
    //         // push that plan onto the pq
    //         println!("code->{code}");
    //         println!("sems->{sems:?}");
    //         println!("pq->{pq:?}");
    //         break;
    //     }
    //
    //     Ok(())
    // }
    //
    // #[allow(unused)]
    // pub async fn fill(&self) -> Result<Self> {
    //     let plan = self.to_owned();
    //     let targets = plan.get_targets();
    //     let commits = plan.get_commits();
    //     let acad_year = self.acad_year(1);
    //
    //     // sample space of all the modules related to the target modules
    //     let sample_space =
    //         self.src.flatten_requirements(targets, &acad_year).await?;
    //
    //     // remove the modules that are already committed
    //     // TODO: uncomment the next line
    //     // sample_space.retain(|v| !commits.contains(v.code()));
    //
    //     println!(
    //         "sample space -> {:?}",
    //         sample_space.iter().map(|v| v.code()).collect::<Vec<_>>()
    //     );
    //
    //     let sample_space =
    //         sample_space.into_iter().map(|m| (m.to_code(), m)).collect();
    //     let sorted = ModuleCollection::topological_sort(sample_space);
    //     println!(
    //         "topo sorted -> {:?}",
    //         sorted.iter().map(|v| &v.0).collect::<Vec<_>>()
    //     );
    //
    //     // sort sample_space by topological order
    //     // poll this queue while populating the `plan`
    //     // remember to check for sem availability on each module
    //
    //     // println!("target -> {targets:?}");
    //     println!("commits -> {commits:?}");
    //     Ok(plan)
    // }
}
