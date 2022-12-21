use database::ModuleCollection;
use path::Path;
use std::collections::BinaryHeap;
use types::Module;

#[allow(unused)]
async fn db() {
    use database::Client;
    use prereqtree::PrereqTree;
    let m = Client::debug_init().await.unwrap();
    let top = m.find_one("CS3244", "2022/2023").await.unwrap();
    let sample_space = m
        .flatten_requirements(vec!["CS3244".to_string()], "2022/2023")
        .await
        .unwrap();
    println!("{sample_space:?}");
}

async fn sample_space(m: &ModuleCollection, codes: Vec<&str>) -> Vec<Module> {
    m.flatten_requirements(
        codes.iter().map(|v| v.to_string()).collect(),
        "2022/2023",
    )
    .await
    .unwrap()
}

#[tokio::main]
async fn main() {
    use database::Client;
    let m = Client::debug_init().await.unwrap();
    let sample_space = sample_space(&m, vec!["CS3244"]).await;

    // List of possible paths to take that reaches CS3244
    let mut possible_routes = vec![];
    let mut prl = usize::MAX;

    let mut pq: BinaryHeap<Path> = BinaryHeap::new();
    pq.push(Path::new());
    // set the modules that want to be completed
    // let want = vec!["CS3244".to_string(), "CS3216".to_string()];
    let want = vec!["CS3244".to_string()];

    while let Some(mut path) = pq.pop() {
        println!("{}", pq.len());
        println!("{path:?}");
        if path.len() > prl {
            break;
        }
        if path.len() >= 16 {
            continue;
        }
        let choices = path.choices(&sample_space);
        if path.doing_count() < 5 && choices.len() > 0 {
            for next_mod in choices {
                let mut path = path.clone();
                path.mark(next_mod);
                if path.is_done(&want) {
                    path.next_sem();
                    if path.len() < prl {
                        prl = path.len();
                        possible_routes.clear();
                        possible_routes.push(path);
                    } else if path.len() == prl {
                        possible_routes.push(path);
                    }
                    continue;
                }
                pq.push(path);
            }
        } else {
            path.next_sem();
            pq.push(path);
        }
    }
    possible_routes.sort_by(|a, b| b.mod_count().cmp(&a.mod_count()));
    for i in possible_routes {
        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~\n{i:?}");
    }
}
