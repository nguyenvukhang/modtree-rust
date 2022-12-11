#[macro_use]
mod macros;
mod std_impl;

use crate::{Error, Result};
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
    pub(crate) fn satisfied_by(
        &self,
        code: String,
        done: &HashSet<String>,
    ) -> Result<()> {
        let ok = self._satisfied_by(done);
        let tree = format!("{:?}", self);
        let err = Error::PrerequisitesNotSatisfied(code, tree);
        ok.then_some(()).ok_or(err)
    }
    fn _satisfied_by(&self, done: &HashSet<String>) -> bool {
        match self {
            PrereqTree::Only(only) => only.is_empty() || done.contains(only),
            PrereqTree::And { and } => {
                and.iter().fold(true, |a, p| a && p._satisfied_by(done))
            }
            PrereqTree::Or { or } => {
                or.iter().fold(or.is_empty(), |a, p| a || p._satisfied_by(done))
            }
        }
    }
}

#[cfg(test)]
mod spec;
