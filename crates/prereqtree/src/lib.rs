#[macro_use]
mod macros;
mod loader;
mod std_impl;
mod util;

use loader::Loader;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::future::Future;

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
    /// Checks if a code exists in the entire prereqtree.
    pub fn contains_code(&self, module_code: &str) -> bool {
        match self {
            Only(only) => !only.is_empty() && only.eq(module_code),
            And { and } => and.iter().any(|v| v.contains_code(module_code)),
            Or { or } => or.iter().any(|v| v.contains_code(module_code)),
        }
    }

    /// Checks if prereqtree is a leaf node.
    pub fn is_leaf(&self) -> bool {
        match self {
            Only(only) if only.is_empty() => true,
            _ => false,
        }
    }

    /// Counts the minimum number of modules required to satisfy the tree.
    pub fn min_to_unlock(&self, done: &HashSet<String>) -> u8 {
        match self {
            Only(only) => match only.is_empty() || done.contains(only) {
                true => 0,
                false => 1,
            },
            And { and } => and.iter().map(|v| v.min_to_unlock(done)).sum(),
            Or { or } => {
                or.iter().map(|v| v.min_to_unlock(done)).min().unwrap_or(0)
            }
        }
    }

    /// Checks if a set of modules done satisfies the prereqtree.
    pub fn satisfied_by(&self, done: &HashSet<String>) -> bool {
        match self {
            Only(only) => only.is_empty() || done.contains(only),
            And { and } => {
                and.iter().fold(true, |a, p| a && p.satisfied_by(done))
            }
            Or { or } => {
                or.iter().fold(or.is_empty(), |a, p| a || p.satisfied_by(done))
            }
        }
    }

    /// Returns one possible path that is shortest.
    pub fn min_path(&self) -> Vec<String> {
        match self {
            Only(v) if v.is_empty() => vec![],
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
            Only(only) if only.is_empty() => vec![],
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
            Only(only) if only.is_empty() => vec![],
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
        F: Fn(String) -> R,
        R: Future<Output = Option<Self>>,
    {
        let mut remaining: Vec<String> = self.flatten();
        let mut result: HashSet<String> = HashSet::new();
        let mut loader = Loader::new(loader);
        while let Some(code) = remaining.pop() {
            match loader.get(&code).await {
                Some(And { and: t } | Or { or: t }) => remaining.extend(
                    t.iter()
                        .flat_map(|v| v.flatten())
                        .filter(|v| !result.contains(v)),
                ),
                Some(Only(only)) if !only.is_empty() => {
                    result.insert(only);
                }
                _ => {
                    result.insert(code);
                }
            }
        }
        Vec::from_iter(result)
    }
}

#[cfg(test)]
mod spec;
