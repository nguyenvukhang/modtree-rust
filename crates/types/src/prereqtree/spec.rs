use crate::prereqtree::PrereqTree;
use std::collections::HashSet;

#[test]
fn prereqtree_macro_test() {
    // TODO: complete the PartialEq first
}

#[test]
fn satisfies_test() {
    fn test(tree: &PrereqTree, done: HashSet<String>, expect: bool) {
        assert!(!expect ^ tree._satisfied_by(&done));
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
fn min_unlock_test() {
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
