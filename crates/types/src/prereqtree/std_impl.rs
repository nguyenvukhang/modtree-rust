use crate::prereqtree::PrereqTree;

impl Default for PrereqTree {
    fn default() -> Self {
        Self::Only("".to_string())
    }
}

/// Compares vectors up to and including length of 200.
fn vec_eq<T, F: Fn(&T, &T) -> bool>(a: &Vec<T>, b: &Vec<T>, cmp: F) -> bool {
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

fn _eq(t1: &PrereqTree, t2: &PrereqTree) -> bool {
    use PrereqTree::*;
    match (t1, t2) {
        (Or { or: v1 }, Or { or: v2 }) => vec_eq(v1, v2, |a, b| _eq(a, b)),
        (And { and: v1 }, And { and: v2 }) => vec_eq(v1, v2, |a, b| _eq(a, b)),
        (Only(v1), Only(v2)) => v1.eq(v2),
        _ => false,
    }
}

/// Note that when two `PrereqTree`s are equal, they are not only logically
/// equal but structurally equal also. Within each layer, order does not matter.
impl PartialEq for PrereqTree {
    fn eq(&self, other: &Self) -> bool {
        _eq(self, other)
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

#[test]
fn test_prt_eq() {
    t!(done);
    // base trees
    assert_eq!(t!(), t!());
    assert_ne!(t!(), t!(A));
    assert_eq!(t!(A), t!(A));
    assert_ne!(t!(A), t!(B));
    // complex trees
    assert_eq!(
        t!(and, t!(A), t!(or, t!(B), t!(C))),
        t!(and, t!(or, t!(C), t!(B)), t!(A))
    );
    assert_eq!(
        t!(and, t!(A), t!(or, t!(B), t!(C), t!(and, t!(X), t!(Y), t!(Z)))),
        t!(and, t!(or, t!(C), t!(and, t!(X), t!(Z), t!(Y)), t!(B)), t!(A))
    );
}
