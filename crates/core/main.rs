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
        .flatten_requirements(
            vec!["CS3244".to_string(), "CS3216".to_string()],
            "2022/2023",
        )
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

async fn algo() {
    use database::Client;
    let m = Client::debug_init().await.unwrap();
    // let sample_space = sample_space(&m, vec!["CS3244", "CS3216"]).await;
    let sample_space = sample_space(&m, vec!["CS3244"]).await;

    // List of possible paths to take that reaches CS3244
    let mut possible_routes = vec![];
    // Possible routes length
    let mut prl = usize::MAX;

    let mut pq: BinaryHeap<Path> = BinaryHeap::new();
    pq.push(Path::new());
    // set the modules that want to be completed
    // let want = vec!["CS3244".to_string(), "CS3216".to_string()];
    let want = vec!["CS3244".to_string()];

    while let Some(mut path) = pq.pop() {
        // check if done, and if shorter then update best path
        if path.is_done(&want) {
            if path.doing_count() > 0 {
                path.next_sem()
            }
            if path.len() < prl {
                prl = path.len(); // update shortest path length
                possible_routes.clear();
                possible_routes.push(path);
            } else if path.len() == prl {
                possible_routes.push(path);
            }
            continue;
        }
        // dijkstra assumption
        if path.len() > prl {
            break;
        }
        if path.len() >= 16 {
            continue;
        }
        // Generate list of choices
        let choices = path.choices(&sample_space);
        print!(
            "{}, {path:?}, [{}, {}]",
            pq.len(),
            choices.len(),
            path.doing_count()
        );

        // go to next sem only if no mods left to do,
        // or if the doing count has exceeded limit
        if choices.len() == 0 || path.doing_count() >= 5 {
            println!("-->NEXT");
            path.next_sem();
            pq.push(path);
            continue;
        }
        println!("-->CHOICES");
        // else, go through the choices and add them to the queue
        for next_mod in choices {
            let mut path = path.clone();
            path.mark(next_mod);
            pq.push(path);
        }
    }
    possible_routes.sort_by(|a, b| b.mod_count().cmp(&a.mod_count()));
    for i in possible_routes {
        println!("ok: {i:?}");
    }
}

#[tokio::main]
async fn main() {
    algo().await;
}
