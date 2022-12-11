#[macro_use]
mod macros;
mod experimental;
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
    pub(crate) fn contains_code(&self, module_code: &str) -> bool {
        match self {
            Only(only) => !only.is_empty() && only.eq(module_code),
            And { and } => and.iter().any(|v| v.contains_code(module_code)),
            Or { or } => or.iter().any(|v| v.contains_code(module_code)),
        }
    }

    /// Counts the minimum number of modules required to satisfy the tree.
    pub(crate) fn min_to_unlock(&self, done: &HashSet<String>) -> u8 {
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
    pub(crate) fn satisfied_by(&self, done: &HashSet<String>) -> bool {
        match self {
            PrereqTree::Only(only) => only.is_empty() || done.contains(only),
            PrereqTree::And { and } => {
                and.iter().fold(true, |a, p| a && p.satisfied_by(done))
            }
            PrereqTree::Or { or } => {
                or.iter().fold(or.is_empty(), |a, p| a || p.satisfied_by(done))
            }
        }
    }

    /// Returns one possible path that is shortest
    pub(crate) fn min_path(&self) -> Vec<String> {
        self._min_path(vec![])
    }
}

/// Private (usually recursive) functions.
impl PrereqTree {
    fn _min_path(&self, mut path: Vec<String>) -> Vec<String> {
        match self {
            Only(v) if v.is_empty() => path,
            Only(only) => {
                path.push(only.to_string());
                path
            }
            And { and } => {
                let mut set = HashSet::new();
                and.iter().for_each(|v| set.extend(v._min_path(vec![])));
                path.extend(set);
                path
            }
            Or { or } => {
                let paths = or.iter().map(|v| v._min_path(vec![]));
                if let Some(min) = paths.min_by(|a, b| a.len().cmp(&b.len())) {
                    path.extend(min)
                }
                path
            }
        }
    }
}

#[cfg(test)]
mod spec;
