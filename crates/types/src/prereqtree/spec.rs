use crate::prereqtree::util::vec_eq;
use crate::prereqtree::PrereqTree;
use std::collections::HashSet;

#[cfg(test)]
fn s_vec(s: Vec<&str>) -> Vec<String> {
    s.iter().map(|v| v.to_string()).collect()
}

#[test]
fn satisfied_by_test() {
    fn test(tree: &PrereqTree, done: HashSet<String>, expect: bool) {
        assert!(!expect ^ tree.satisfied_by(&done));
    }
    // empty tree
    test(&t!(), t!(none), true);
    // tests for "and"
    let tree = &t!(and, t!(A), t!(B));
    test(tree, t!(done, A, B), true);
    test(tree, t!(done, A, C), false);
    // tests for "or"
    let tree = &t!(or, t!(A), t!(B));
    test(tree, t!(done, A), true);
    test(tree, t!(done, C), false);
    // tests for nested structures "and(or())"
    let tree = &t!(and, t!(or, t!(A), t!(B)), t!(C));
    test(tree, t!(done, A, C), true);
    test(tree, t!(done, B, C), true);
    test(tree, t!(done, A, B), false);
    test(tree, t!(done, C), false);
    // tests for nested structures "or(and())"
    let tree = &t!(or, t!(and, t!(A), t!(B)), t!(C));
    test(tree, t!(done, A, C), true);
    test(tree, t!(done, B, C), true);
    test(tree, t!(done, A, B), true);
    test(tree, t!(done, C), true);
}

#[test]
fn min_to_unlock_test() {
    fn test(tree: &PrereqTree, done: HashSet<String>, expect: u8) {
        assert_eq!(tree.min_to_unlock(&done), expect);
    }
    // empty tree
    test(&t!(), t!(none), 0);
    test(&t!(A), t!(done, A), 0);
    test(&t!(A), t!(none), 1);
    // tests for "and"
    let tree = &t!(and, t!(A), t!(B));
    test(tree, t!(none), 2);
    test(tree, t!(done, A), 1);
    test(tree, t!(done, A, B), 0);
    // tests for "or"
    let tree = &t!(or, t!(A), t!(B));
    test(tree, t!(none), 1);
    test(tree, t!(done, A), 0);
    test(tree, t!(done, A, B), 0);
    // tests for nested structures "and(or())"
    let tree = &t!(and, t!(or, t!(A), t!(B)), t!(C));
    test(tree, t!(none), 2);
    test(tree, t!(done, A), 1);
    test(tree, t!(done, C), 1);
    test(tree, t!(done, A, C), 0);
    // tests for nested structures "or(and())"
    let tree = &t!(or, t!(and, t!(A), t!(B)), t!(C));
    test(tree, t!(none), 1);
    test(tree, t!(done, A), 1);
    test(tree, t!(done, C), 0);
    test(tree, t!(done, A, C), 0);
}

#[test]
fn min_path_test() {
    use crate::prereqtree::util::vec_eq;
    macro_rules! test {
        ($tree:expr, $expected:expr, $equal:expr) => {
            let expected = s_vec($expected);
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
    let tree = t!(or, t!(and, t!(A), t!(B), t!(C)), t!(and, t!(A), t!(C)));
    test!(tree, vec!["A", "C"], true);
}

#[test]
fn flatten_test() {
    use crate::prereqtree::util::vec_eq;
    macro_rules! flat {
        ($tree:expr, $expected:expr) => {
            let expected = s_vec($expected);
            let received = &$tree.flatten();
            let ok = vec_eq(&received, &expected, |a, b| a.eq(b));
            if !ok {
                println!("received->{:?}", received);
                println!("expected->{:?}", expected);
            }
            assert!(ok);
        };
    }
    // tests for "and"
    let tree = &t!(and, t!(A), t!(B));
    flat!(tree, vec!["A", "B"]);
    // tests for "or"
    let tree = &t!(or, t!(A), t!(B));
    flat!(tree, vec!["A", "B"]);
    // tests for nested structures "and(or())"
    let tree = &t!(and, t!(or, t!(A), t!(B)), t!(C));
    flat!(tree, vec!["A", "B", "C"]);
    // tests for nested structures "or(and())"
    let tree = &t!(or, t!(and, t!(A), t!(B)), t!(C));
    flat!(tree, vec!["A", "B", "C"]);
    // other trees
    let tree = t!(or, t!(and, t!(A), t!(B)), t!(and, t!(C), t!(D), t!(E)));
    flat!(tree, vec!["A", "B", "C", "D", "E"]);
    let tree = t!(and, t!(or, t!(A), t!(B)), t!(and, t!(C), t!(D), t!(E)));
    flat!(tree, vec!["A", "B", "C", "D", "E"]);
    let tree = t!(or, t!(and, t!(A), t!(B), t!(C)), t!(and, t!(A), t!(C)));
    flat!(tree, vec!["A", "B", "C"]);
}

#[test]
#[ignore]
fn min_path_filtered_test() {
    use crate::prereqtree::util::vec_eq;
    macro_rules! mpf {
        ($tree:expr, $filter:expr, $expected:expr) => {
            let expected = s_vec($expected);
            let filter = s_vec($filter);
            let received = &$tree.min_path_filtered(&filter).unwrap();
            let ok = vec_eq(&received, &expected, |a, b| a.eq(b));
            if !ok {
                println!("received->{:?}", received);
                println!("expected->{:?}", expected);
            }
            assert!(ok);
        };
        ($tree:expr, $filter:expr) => {
            assert_eq!($tree.min_path_filtered(&s_vec($filter)), None);
        };
    }
    let tree = &t!(and, t!(A), t!(B));
    mpf!(tree, vec![], vec!["A", "B"]);
    mpf!(tree, vec!["A"], vec!["A", "B"]);
    mpf!(tree, vec!["C"]);
    // complex trees
    let tree = t!(
        or,
        t!(and, t!(A), t!(B), t!(C), t!(D)),
        t!(and, t!(E), t!(F), t!(G))
    );
    mpf!(tree, vec![], vec!["A", "B"]);
    mpf!(tree, vec!["C"], vec!["C", "D", "E"]);
    let tree = t!(and, t!(or, t!(A), t!(B)), t!(and, t!(C), t!(D), t!(E)));
    mpf!(tree, vec!["A"], vec!["A", "C", "D", "E"]);
    mpf!(tree, vec!["B"], vec!["B", "C", "D", "E"]);
    mpf!(tree, vec!["C"], vec!["C", "D", "E"]);
    let tree = t!(or, t!(and, t!(A), t!(B), t!(C)), t!(and, t!(A), t!(C)));
    mpf!(tree, vec!["A"], vec!["A", "C"]);
    mpf!(tree, vec!["B"], vec!["A", "B", "C"]);
    let tree = t!(
        and,
        t!(or, t!(A), t!(B)),
        t!(or, t!(C), t!(D)),
        t!(or, t!(E), t!(F), t!(and, t!(X), t!(Y))),
        t!(or, t!(G), t!(H))
    );
    mpf!(tree, vec![], vec!["A", "C", "E", "G"]);
    mpf!(tree, vec!["B"], vec!["B", "C", "E", "G"]);
}

#[test]
fn all_paths() {
    let tree = t!(
        and,
        t!(or, t!(A), t!(B)),
        t!(or, t!(C), t!(D)),
        t!(or, t!(E), t!(F), t!(and, t!(X), t!(Y)))
    );
    let all_paths = tree.all_paths();
    fn s(vec: &[&str]) -> Vec<String> {
        s_vec(vec.to_vec())
    }
    assert!(vec_eq(
        &all_paths,
        &vec![
            s(&["A", "C", "E"]),
            s(&["A", "C", "F"]),
            s(&["A", "C", "X", "Y"]),
            s(&["A", "D", "E"]),
            s(&["A", "D", "F"]),
            s(&["A", "D", "X", "Y"]),
            s(&["B", "C", "E"]),
            s(&["B", "C", "F"]),
            s(&["B", "C", "X", "Y"]),
            s(&["B", "D", "E"]),
            s(&["B", "D", "F"]),
            s(&["B", "D", "X", "Y"]),
        ],
        |a, b| vec_eq(a, b, |a, b| a.eq(b))
    ));
}
