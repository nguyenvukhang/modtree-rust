use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrereqTree {
    Only(String),
    // Node { and: Vec<PrereqTree>, or: Vec<PrereqTree> },
    And { and: Vec<PrereqTree> },
    Or { or: Vec<PrereqTree> },
    // Node { and: Vec<PrereqTree>, or: Vec<PrereqTree> },
}

impl Default for PrereqTree {
    fn default() -> Self {
        Self::Only("".to_string())
    }
}

impl PrereqTree {
    pub(crate) fn is_valid(&self) -> bool {
        match self {
            Self::Only(_) => true,
            // Self::Node { and, or } => and.is_empty() ^ or.is_empty(),
            _ => true,
        }
    }
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
        // PrereqTree::Node { and, or } => {
        //     match (and.is_empty(), or.is_empty()) {
        //         (true, true) => true,
        //         (false, _) => or
        //             .iter()
        //             .fold(or.is_empty(), |a, p| a || p._satisfied_by(done)),
        //         (_, false) => {
        //             and.iter().fold(true, |a, p| a && p._satisfied_by(done))
        //         }
        //     }
        // }
    }
}

#[cfg(test)]
macro_rules! test {
    ($tree:expr, $arr:expr, $expect:expr) => {{
        let set = HashSet::from_iter($arr.iter().map(|v| v.to_string()));
        assert!(!$expect ^ $tree._satisfied_by(&set));
    }};
}

// #[test]
// fn prereqtree_satisfies_test() {
//     use PrereqTree::*;
//     fn t(s: &str) -> PrereqTree {
//         PrereqTree::Only(s.to_string())
//     }
//     assert!(t("")._satisfied_by(&HashSet::new()));
//     test!(t("CS2040"), ["CS1231", "CS1010"], false);
//     test!(t("CS2030"), ["CS2030"], true);
//     // tests for "and"
//     let tree = Node { and: vec![t("A"), t("B")], or: vec![] };
//     test!(tree, ["A", "B"], true);
//     test!(tree, ["A", "C"], false);
//     // tests for "or"
//     let tree = Node { and: vec![], or: vec![t("A"), t("B")] };
//     test!(tree, ["A"], true);
//     test!(tree, ["C"], false);
//     // tests for nested structures
//     let tree = Node {
//         and: vec![Node { and: vec![], or: vec![t("A"), t("B")] }, t("C")],
//         or: vec![],
//     };
//     test!(tree, ["A", "C"], true);
//     test!(tree, ["B", "C"], true);
//     test!(tree, ["A", "B"], false);
//     test!(tree, ["C"], false);
//     // tests for nested structures
//     let tree = Node {
//         and: vec![],
//         or: vec![Node { and: vec![t("A"), t("B")], or: vec![] }, t("C")],
//     };
//     test!(tree, ["A", "C"], true);
//     test!(tree, ["B", "C"], true);
//     test!(tree, ["A", "B"], true);
//     test!(tree, ["C"], true);
// }
