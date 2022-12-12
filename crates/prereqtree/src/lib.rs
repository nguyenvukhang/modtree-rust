#[macro_use]
mod macros;
mod std_impl;
mod util;

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize)]
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
    /// of the modules listed in `require`.
    pub fn min_path_filtered(
        &self,
        filter: &Vec<String>,
    ) -> Option<Vec<String>> {
        let path = self._min_path_filtered(&filter);
        filter.iter().all(|v| path.contains(v)).then_some(path)
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
}

impl PrereqTree {
    /// Simply a biased traversal that returns a shortest path biased towards
    /// the filter. A final check will be done in the entry-point function to see if
    /// the result actually contains all elements in the filter.
    fn _min_path_filtered(&self, filter: &Vec<String>) -> Vec<String> {
        match self {
            Only(v) if v.is_empty() => vec![],
            Only(only) => vec![only.to_string()],
            And { and } => {
                let mut set = HashSet::new();
                and.iter()
                    .for_each(|v| set.extend(v._min_path_filtered(filter)));
                Vec::from_iter(set)
            }
            Or { or } => or
                .iter()
                .map(|v| v._min_path_filtered(filter))
                // TODO: write a all_paths method that generates all valid paths
                // and then filter from that list
                .filter(|p| filter.iter().any(|f| p.contains(f)))
                .min_by(|a, b| a.len().cmp(&b.len()))
                .unwrap_or(vec![]),
        }
    }
}

#[cfg(test)]
mod spec;
