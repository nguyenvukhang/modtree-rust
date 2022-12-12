use std::collections::HashSet;
use std::hash::Hash;

/// Compares vectors up to and including length of 200.
pub fn vec_eq<T, F: Fn(&T, &T) -> bool>(
    a: &Vec<T>,
    b: &Vec<T>,
    cmp: F,
) -> bool {
    if a.len() > 200 || b.len() > 200 || a.len() != b.len() {
        return false;
    }
    let (a, b, len) = (a.as_slice(), b.as_slice(), a.len());
    let (mut ok_a, mut ok_b) = ([false; 200], [false; 200]);
    for idx_a in 0..len {
        if ok_a[idx_a] {
            continue;
        }
        for idx_b in 0..len {
            if ok_b[idx_b] {
                continue;
            }
            if cmp(&a[idx_a], &b[idx_b]) {
                ok_a[idx_a] = true;
                ok_b[idx_b] = true;
            }
        }
    }
    (0..len).fold(true, |a, x| a && ok_a[x] && ok_b[x])
}

#[test]
fn test_vec_eq() {
    macro_rules! test {
        ($a:expr, $b:expr, $expected:expr) => {
            let [a, b]: [Vec<u8>; 2] = [$a, $b];
            assert!(!$expected ^ vec_eq(&a, &b, |a, b| a.eq(b)));
        };
    }
    test!(vec![1, 2, 3, 4], vec![4, 3, 2, 1], true);
    test!(vec![2, 3, 4], vec![4, 3, 1], false);
    test!(vec![2, 3, 4], vec![4, 3, 1], false);
    test!(vec![], vec![1], false);
}

/// Counter that allows a different base for each digit.
/// For example, with bases [5, 3, 2], the count goes
/// 0 -> 1 -> 10 -> 11 -> 20 -> 21 -> 100
struct Counter {
    count: Vec<usize>, // stores count in reverse order
    bases: Vec<usize>,
    done: bool,
}

impl Iterator for Counter {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.done {
            true => return None,
            false => self.count.clone(),
        };
        self.increment();
        Some(result)
    }
}

impl Counter {
    fn new(bases: Vec<usize>) -> Self {
        Self { count: vec![0; bases.len()], bases, done: false }
    }
    /// Returns false only upon reset
    fn _increment(&mut self, place: Option<usize>) -> bool {
        let place = match place {
            None => return false,
            Some(v) if v >= self.bases.len() => return false, // RESET
            Some(v) => v,
        };
        if self.count[place] + 1 == self.bases[place] {
            self.count[place] = 0;
            self._increment(place.checked_sub(1))
        } else {
            self.count[place] += 1;
            true
        }
    }
    fn increment(&mut self) {
        if !self._increment(self.bases.len().checked_sub(1)) {
            self.done = true
        }
    }
}

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
