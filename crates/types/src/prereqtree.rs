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

impl Default for PrereqTree {
    fn default() -> Self {
        Self::Only("".to_string())
    }
}

impl PrereqTree {
    /// Checks if a code exists in the entire prereqtree.
    pub(crate) fn contains_code(&self, module_code: &str) -> bool {
        match self {
            Only(only) => !only.is_empty() && only.eq(module_code),
            And { and } => and.iter().any(|v| v.contains_code(module_code)),
            Or { or } => or.iter().any(|v| v.contains_code(module_code)),
        }
    }

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
fn t(s: &str) -> PrereqTree {
    PrereqTree::Only(s.to_string())
}

#[test]
fn prereqtree_satisfies_test() {
    use PrereqTree::*;
    macro_rules! test {
        ($tree:expr, $arr:expr, $expect:expr) => {{
            let set = HashSet::from_iter($arr.iter().map(|v| v.to_string()));
            assert!(!$expect ^ $tree._satisfied_by(&set));
        }};
    }
    assert!(t("")._satisfied_by(&HashSet::new()));
    test!(t("CS2040"), ["CS1231", "CS1010"], false);
    test!(t("CS2030"), ["CS2030"], true);
    // tests for "and"
    let tree = And { and: vec![t("A"), t("B")] };
    test!(tree, ["A", "B"], true);
    test!(tree, ["A", "C"], false);
    // tests for "or"
    let tree = Or { or: vec![t("A"), t("B")] };
    test!(tree, ["A"], true);
    test!(tree, ["C"], false);
    // tests for nested structures
    let tree = And { and: vec![Or { or: vec![t("A"), t("B")] }, t("C")] };
    test!(tree, ["A", "C"], true);
    test!(tree, ["B", "C"], true);
    test!(tree, ["A", "B"], false);
    test!(tree, ["C"], false);
    // tests for nested structures
    let tree = Or { or: vec![And { and: vec![t("A"), t("B")] }, t("C")] };
    test!(tree, ["A", "C"], true);
    test!(tree, ["B", "C"], true);
    test!(tree, ["A", "B"], true);
    test!(tree, ["C"], true);
}

#[test]
fn prereqtree_min_unlock_test() {
    use PrereqTree::*;
    macro_rules! test {
        ($tree:expr, $done:expr, $expect:expr) => {{
            let set = HashSet::from_iter($done.iter().map(|v| v.to_string()));
            assert_eq!($tree.min_to_unlock(&set), $expect);
        }};
    }
    let empty: [&str; 0] = [];
    assert_eq!(t("").min_to_unlock(&HashSet::new()), 0);
    test!(t(""), empty, 0);
    test!(t("A"), ["A"], 0);
    test!(t("A"), empty, 1);
    // tests for "and"
    let tree = And { and: vec![t("A"), t("B")] };
    test!(tree, empty, 2);
    test!(tree, ["A"], 1);
    test!(tree, ["A", "B"], 0);
    // tests for "or"
    let tree = Or { or: vec![t("A"), t("B")] };
    test!(tree, ["A", "B"], 0);
    test!(tree, ["A"], 0);
    test!(tree, empty, 1);
    // tests for nested structures
    let tree = And { and: vec![Or { or: vec![t("A"), t("B")] }, t("C")] };
    test!(tree, empty, 2);
    test!(tree, ["C"], 1);
    test!(tree, ["A"], 1);
    test!(tree, ["B"], 1);
    test!(tree, ["A", "C"], 0);
    // tests for nested structures
    let tree = Or { or: vec![And { and: vec![t("A"), t("B")] }, t("C")] };
    test!(tree, empty, 1);
    test!(tree, ["C"], 0);
    test!(tree, ["A"], 1);
    test!(tree, ["B"], 1);
    test!(tree, ["A", "C"], 0);
}
