#[macro_use]
mod macros;
mod loader;
mod std_impl;
mod util;

use loader::Loader;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::mem;

pub use util::vec_eq;

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum PrereqTree {
    Only(String),
    And { and: Vec<PrereqTree> },
    Or { or: Vec<PrereqTree> },
}
use PrereqTree::*;

/// Public-facing API
impl PrereqTree {
    /// Creates an empty `PrereqTree`.
    pub fn empty() -> Self {
        PrereqTree::Only("".to_string())
    }

    /// Creates a `PrereqTree` with one module.
    pub fn only(code: &str) -> Self {
        PrereqTree::Only(code.to_string())
    }

    /// Checks if a code exists in the entire prereqtree.
    pub fn contains_code(&self, module_code: &str) -> bool {
        match self {
            Only(only) => only.eq(module_code),
            And { and } => and.iter().any(|v| v.contains_code(module_code)),
            Or { or } => or.iter().any(|v| v.contains_code(module_code)),
        }
    }

    /// Checks if prereqtree is a leaf node.
    pub fn is_empty(&self) -> bool {
        match self {
            Only(only) if only.is_empty() => true,
            _ => false,
        }
    }

    /// Counts the minimum number of modules required to satisfy the tree.
    pub fn min_to_unlock(&self) -> u8 {
        match self {
            Only(v) => match v.as_str() {
                "" => 0,
                _ => 1,
            },
            And { and } => and.iter().map(|v| v.min_to_unlock()).sum(),
            Or { or } => {
                or.iter().map(|v| v.min_to_unlock()).min().unwrap_or(0)
            }
        }
    }

    /// Counts the minimum number of modules required to satisfy the tree, given
    /// a list that is already done.
    pub fn left_to_unlock(&self, done: &HashSet<String>) -> u8 {
        match self {
            Only(only) if only.eq("") || done.contains(only) => 0,
            Only(_) => 1,
            And { and } => and.iter().map(|v| v.left_to_unlock(done)).sum(),
            Or { or } => {
                or.iter().map(|v| v.left_to_unlock(done)).min().unwrap_or(0)
            }
        }
    }

    /// Checks if a set of modules done satisfies the prereqtree.
    pub fn satisfied_by(&self, done: &HashSet<String>) -> bool {
        match self {
            Only(only) => only.eq("") || done.contains(only),
            And { and } => {
                and.iter().fold(true, |a, p| a && p.satisfied_by(done))
            }
            Or { or } => {
                or.iter().fold(or.is_empty(), |a, p| a || p.satisfied_by(done))
            }
        }
    }

    /// Checks if a set of modules done satisfies the prereqtree.
    pub fn satisfied_by_one(&self, done: &str) -> bool {
        match self {
            Only(only) => done.eq("") || done.eq(only),
            And { and } => {
                and.iter().fold(true, |a, p| a && p.satisfied_by_one(done))
            }
            Or { or } => or
                .iter()
                .fold(or.is_empty(), |a, p| a || p.satisfied_by_one(done)),
        }
    }

    /// Returns one possible path that is shortest.
    pub fn min_path(&self) -> Vec<String> {
        match self {
            Only(only) if only.eq("") => vec![],
            Only(only) => vec![only.to_string()],
            And { and } => {
                let mut set = HashSet::new();
                and.iter().for_each(|v| set.extend(v.min_path()));
                Vec::from_iter(set)
            }
            Or { or } => or
                .iter()
                .map(|v| v.min_path())
                .min_by(|a, b| a.len().cmp(&b.len()))
                .unwrap_or(vec![]),
        }
    }

    /// Returns one possible path that is shortest, but it must also contain all
    /// of the modules listed in `required`.
    pub fn min_path_filtered(
        &self,
        required: &Vec<String>,
    ) -> Option<Vec<String>> {
        self.all_paths()
            .into_iter()
            // must contain all required modules
            .filter(|p| required.iter().all(|r| p.contains(r)))
            .min_by(|a, b| a.len().cmp(&b.len()))
    }

    /// Returns every module found in the PrereqTree in a list.
    pub fn flatten(&self) -> Vec<String> {
        match self {
            Only(only) if only.eq("") => vec![],
            Only(only) => vec![only.to_string()],
            Or { or: t } | And { and: t } => {
                let mut set = HashSet::new();
                t.iter().for_each(|v| set.extend(v.flatten()));
                Vec::from_iter(set)
            }
        }
    }

    /// Returns every valid path taken to satisfy this prereqtree.
    pub fn all_paths(&self) -> Vec<Vec<String>> {
        match self {
            Only(only) if only.eq("") => vec![],
            Only(only) => vec![vec![only.to_string()]],
            Or { or: t } => {
                // several possible journeys.
                t.iter().flat_map(|subtree| subtree.all_paths()).collect()
            }
            And { and: t } => {
                // cross-chains all children journeys into one.
                util::weave(&t.iter().map(|st| st.all_paths()).collect())
            }
        }
    }

    /// Flattens the tree until leaf nodes are reached.
    pub async fn global_flatten<F, R>(&mut self, loader: F) -> Vec<String>
    where
        F: Fn(Vec<String>) -> R,
        R: Future<Output = Option<HashMap<String, Self>>>,
    {
        let mut remaining: Vec<String> = self.flatten();
        let mut result: HashSet<String> = HashSet::new();
        let mut loader = Loader::new(loader);
        while !remaining.is_empty() {
            remaining.sort_by(|a, b| match (loader.get(a), loader.get(b)) {
                (Some(_), _) => Ordering::Greater,
                (_, Some(_)) => Ordering::Less,
                _ => Ordering::Equal,
            });
            let (tree, code) = match loader.get(remaining.last().unwrap()) {
                None => {
                    loader.load_trees(&remaining).await;
                    let code = remaining.pop().unwrap();
                    (loader.get(&code).unwrap(), code)
                }
                Some(v) => (v, remaining.pop().unwrap()),
            };

            match tree {
                And { and: t } | Or { or: t } => remaining.extend(
                    t.iter()
                        .flat_map(|v| v.flatten())
                        .filter(|v| !result.contains(v)),
                ),
                Only(only) if only.eq("") => {
                    result.insert(code.to_string());
                }
                Only(only) => {
                    result.insert(only);
                }
            }
        }
        Vec::from_iter(result)
    }

    /// Resolves a module code in th prereqtree. This is not just a function to
    /// remove the module. This function treats the module as done and updates
    /// the tree to reflect that. So if module A needs B and (C or D), then
    /// calling A.resolve(C) will reduce A's prereqtree to just "needs B", since
    /// the rest have been satisfied.
    pub fn resolve(&mut self, module_code: &str) {
        fn filter(v: Vec<PrereqTree>, code: &str) -> Vec<PrereqTree> {
            v.into_iter().filter_map(|t| f(t, code)).collect()
        }
        fn f(t: PrereqTree, code: &str) -> Option<PrereqTree> {
            match t {
                Only(v) if v.eq(code) || v.eq("") => None,
                Only(v) => Some(Only(v)),
                And { and } => match filter(and, code) {
                    and if and.is_empty() => None,
                    and => Some(And { and }),
                },
                Or { or } => match (or.len(), filter(or, code)) {
                    (len, or) if or.len() == len => Some(Or { or }),
                    _ => None,
                },
            }
        }
        mem::swap(
            self,
            &mut f(self.clone(), module_code).unwrap_or(Self::empty()),
        );
    }

    pub fn debug<T: std::fmt::Debug>(trees: &Vec<(String, T)>) {
        // let a: Vec<_> = trees.iter().map(|v| v.0.to_string()).collect();
        println!("trees: ------------");
        for (code, tree) in trees {
            println!("{code}: {tree:?}")
        }
        println!("-------------------");
    }

    /// Sorts the modules into the order which they must be done in. Tie-breaks
    /// by lexicographical order.
    pub fn topological_sort(
        modules: Vec<(String, PrereqTree)>,
    ) -> Vec<(String, PrereqTree)> {
        type T = PrereqTree;
        let mut keyed: Vec<(String, T, T)> =
            modules.into_iter().map(|v| (v.0, v.1.clone(), v.1)).collect();
        fn comp(a: &(String, T, T), b: &(String, T, T)) -> std::cmp::Ordering {
            match a.2.min_to_unlock().cmp(&b.2.min_to_unlock()) {
                std::cmp::Ordering::Equal => a.0.cmp(&b.0),
                v => v,
            }
        }
        let mut i = 0;
        while i < keyed.len() {
            let n = keyed.len();
            keyed[i..n].sort_by(comp);
            if keyed[i].2.min_to_unlock() > 0 {
                eprintln!("WARNING: module can't be done: {}", keyed[i].0);
                keyed.remove(i);
                continue;
            }
            let code = keyed[i].0.clone();
            let n = keyed.len();
            keyed[i..n].iter_mut().for_each(|t| t.2.resolve(&code));
            i += 1;
        }
        keyed.into_iter().map(|v| (v.0, v.1)).collect()
    }
}

#[cfg(test)]
mod spec;
