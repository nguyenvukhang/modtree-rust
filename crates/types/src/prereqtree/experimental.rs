use crate::prereqtree::PrereqTree;
use std::collections::HashSet;
use PrereqTree::*;

impl PrereqTree {
    /// Returns one possible path that is shortest
    /// TODO: return all possible shortest paths
    fn min_path(&self) -> Vec<String> {
        self._min_path(vec![])
    }
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
                path.extend(Vec::from_iter(set));
                path
            }
            Or { or } => or
                .iter()
                .map(|v| v._min_path(path.clone()))
                .min_by(|a, b| a.len().cmp(&b.len()))
                .unwrap_or(path),
        }
    }
}

#[test]
// #[ignore]
fn test() {
    use crate::prereqtree::util::vec_eq;
    macro_rules! test {
        ($tree:expr, $expected:expr, $equal:expr) => {
            let expected: Vec<String> =
                $expected.iter().map(|v| v.to_string()).collect();
            let received = &$tree.min_path();
            let ok = !$equal ^ vec_eq(&received, &expected, |a, b| a.eq(b));
            if !ok {
                println!("received->{:?}", received);
                println!("expected->{:?}", expected);
            }
            assert!(ok);
        };
    }
    let tree = t!(and, t!(A), t!(B), t!(C));
    test!(tree, vec!["A", "B", "C"], true);
    let tree = t!(or, t!(and, t!(A), t!(B)), t!(C));
    test!(tree, vec!["C"], true);
    let tree = t!(or, t!(and, t!(A), t!(B)), t!(and, t!(C), t!(D), t!(E)));
    test!(tree, vec!["A", "B"], true);
    let tree = t!(and, t!(or, t!(A), t!(B)), t!(and, t!(C), t!(D), t!(E)));
    test!(tree, vec!["A", "C", "D", "E"], true);
}
