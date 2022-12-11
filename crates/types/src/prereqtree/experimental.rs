use crate::prereqtree::PrereqTree;
use std::collections::HashSet;
use PrereqTree::*;

/// Mininum module list required to clear an `Or` variant
/// Returns multiple shortest paths
fn min_or_many(trees: &Vec<PrereqTree>) -> Vec<Vec<String>> {
    let mut min_len = usize::MAX;
    let mut min_paths: Vec<Vec<String>> = vec![];
    trees.iter().map(|v| v._min_path(vec![])).for_each(|p| {
        if p.len() == min_len {
            min_paths.push(p);
        } else if p.len() < min_len {
            min_len = p.len();
            min_paths.clear();
            min_paths.push(p);
        }
    });
    min_paths
}

impl PrereqTree {
    fn multi_min_path(&self) -> Vec<Vec<String>> {
        self._multi_min_path(vec![])
    }
    fn _multi_min_path(&self, mut paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
        match self {
            Only(v) if v.is_empty() => paths,
            Only(only) if paths.is_empty() => vec![vec![only.to_string()]],
            Only(only) => {
                paths.iter_mut().for_each(|v| v.push(only.to_string()));
                paths
            }
            Or { or } if paths.is_empty() => min_or_many(or),
            Or { or } => {
                let choices = min_or_many(or);
                // cross-weave paths and choices here
                paths
                    .iter()
                    .flat_map(|p| choices.iter().map(move |c| (p, c)))
                    .map(|(p, c)| [p.clone(), c.clone()].concat())
                    .collect()
            }
            And { and } => paths,
            And { and } if paths.is_empty() => {
                paths
                // let choices: Vec<Vec<String>> =
                //     and.iter().map(|v| v._multi_min_path(vec![])).collect();
                // // hello
                // vec![]
            }
        }
    }
}
