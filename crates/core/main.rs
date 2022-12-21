use database::ModuleCollection;
use prereqtree::PrereqTree;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
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

async fn sample_space(m: &ModuleCollection) -> Vec<Module> {
    m.flatten_requirements(vec!["CS3244".to_string()], "2022/2023")
        .await
        .unwrap()
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Node {
    code: String,
    sems: Vec<u8>,
    req: Tree2,
}

#[derive(Debug, Clone, Default, Serialize)]
pub enum Tree2 {
    #[default]
    None,
    Node(Box<Node>),
    And(Vec<Tree2>),
    Or(Vec<Tree2>),
}
use Tree2::*;

impl Tree2 {
    fn node(m: &Node) -> Self {
        Self::Node(Box::new(m.clone()))
    }
}

#[derive(Debug)]
struct Mods(HashMap<String, Module>);
impl Mods {
    fn get(&self, code: &str) -> Module {
        self.0.get(code).expect(&format!("should have {code}")).clone()
    }
}

fn to_tree(tree: PrereqTree, data: &Mods) -> Tree2 {
    use PrereqTree::*;
    match tree {
        Only(v) if v.is_empty() => Tree2::None,
        Only(v) => Tree2::Node(Box::new(to_node(&data.get(&v), data))),
        And { and } => {
            Tree2::And(and.into_iter().map(|t| to_tree(t, data)).collect())
        }
        Or { or } => {
            Tree2::Or(or.into_iter().map(|t| to_tree(t, data)).collect())
        }
    }
}

fn to_node(module: &Module, data: &Mods) -> Node {
    Node {
        code: module.to_code(),
        sems: module.to_semesters().into_iter().map(|v| v as u8).collect(),
        req: to_tree(module.to_prereqtree(), data),
    }
    // Self::default()
}

#[tokio::main]
async fn main() {
    use database::Client;
    let m = Client::debug_init().await.unwrap();
    let sample_space = sample_space(&m).await;

    // mods that still exist
    let valid_mods: HashSet<_> =
        sample_space.iter().map(|m| m.to_code()).collect();

    // HashMap of modules, where only valid mods in prereqtrees are kept
    let hashed_mods = sample_space
        .clone()
        .into_iter()
        .map(|mut m| {
            m.set_tree(
                m.prereqtree()
                    .retain(&valid_mods)
                    .unwrap_or(PrereqTree::empty()),
            );
            (m.to_code(), m)
        })
        .collect::<HashMap<String, _>>();
    let mods = Mods(hashed_mods.clone());
    for (code, module) in hashed_mods {
        if code.eq("CS3244") {
            println!("all: {:?}", module.prereqtree().all_paths());
            // println!("orignal ->{:?}", module);
            // println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
            // let node = to_node(&module, &mods);
            // println!("tranf   ->{:?}", node);
            // let pretty = to_string_pretty(&node);
            // println!("tranf   ->{}", pretty.unwrap());
        }
        // println!("mod->{module:?}")
    }
}
