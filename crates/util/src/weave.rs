use crate::Counter;
use std::collections::HashSet;
use std::hash::Hash;

/// Extract elements from a list of lists into one list of unique elements.
///
/// Application: T is a module, &Vec<T> represents a set of modules/a path to take
/// `lol` represents a chain of paths to take
/// the returned result is the combined chain.
fn chain<T: Eq + Hash + Clone>(lol: Vec<&Vec<T>>) -> Vec<T> {
    let mut set: HashSet<T> = HashSet::new();
    lol.iter().for_each(|l| set.extend(l.iter().map(|v| v.to_owned())));
    Vec::from_iter(set)
}

/// Cross-chains all possible paths.
/// `T` refers to a module.
/// A path is a `Vec<T>` in that it is a list of modules to take..
/// `llp` represents a List of List of Paths that can complete a sub-prereqtree,
/// where to complete the full prereqtree, we need at least one element from
/// each List of Paths.
///
/// For example with this `llp`:
///   - [path1, path2]
///   - [path3, path4]
///
/// To satisfy requirements, we can use path1 + path3, but not path1 + path2,
/// since the second list will not be satisfied.
///
/// The return value Vec<Vec<T>> represents a Vec (list) of possible chained paths:
///   - [path1 + path3]
///   - [path1 + path4]
///   - [path2 + path3]
///   - [path2 + path4]
///
/// and also makes sure that each new path contains only unique modules.
///
/// Caveat: this function does not check if the final result returns any
/// duplicate paths.
pub fn weave<T: Eq + Hash + Clone>(llp: &Vec<Vec<Vec<T>>>) -> Vec<Vec<T>> {
    let bases: Vec<usize> = llp.iter().map(|v| v.len()).collect();
    let mut res = vec![];
    for state in Counter::new(bases) {
        res.push(chain((0..state.len()).map(|i| &llp[i][state[i]]).collect()));
    }
    res
}

#[test]
fn merge_test() {
    use crate::vec_eq;
    // possible paths
    let mut llp = vec![];
    llp.push(vec![vec![1, 2], vec![3, 4]]);
    llp.push(vec![vec![91, 92], vec![3, 94]]);
    assert!(vec_eq(
        &weave(&llp),
        &vec![
            vec![1, 2, 91, 92],
            vec![1, 2, 3, 94],
            vec![3, 4, 91, 92],
            vec![3, 4, 94]
        ],
        |a, b| vec_eq(a, b, |a, b| a == b)
    ));
}
