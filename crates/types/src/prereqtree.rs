use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrereqTree {
    Empty,
    Only(String),
    And { and: Vec<PrereqTree> },
    Or { or: Vec<PrereqTree> },
}

impl Default for PrereqTree {
    fn default() -> Self {
        Self::Only("".to_string())
    }
}

impl PrereqTree {
    fn satisfied_by(&self, done: &HashSet<String>) -> bool {
        match self {
            PrereqTree::Empty => true,
            PrereqTree::Only(only) => done.contains(only),
            PrereqTree::And { and } => {
                and.iter().fold(true, |a, p| a && p.satisfied_by(done))
            }
            PrereqTree::Or { or } => {
                or.iter().fold(or.is_empty(), |a, p| a || p.satisfied_by(done))
            }
        }
    }
}

#[cfg(test)]
macro_rules! test {
    ($tree:expr, $arr:expr, $expect:expr) => {{
        let set = HashSet::from_iter($arr.iter().map(|v| v.to_string()));
        assert!(!$expect ^ $tree.satisfied_by(&set));
    }};
}

/// Single-requirement PrereqTree
#[cfg(test)]
fn t(s: &str) -> PrereqTree {
    PrereqTree::Only(s.to_string())
}

#[test]
fn prereqtree_satisfies_test() {
    use PrereqTree::*;
    println!();
    assert!(PrereqTree::Empty.satisfied_by(&HashSet::new()));
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
