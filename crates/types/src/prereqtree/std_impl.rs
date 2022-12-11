use crate::prereqtree::util::vec_eq;
use crate::prereqtree::PrereqTree;

impl Default for PrereqTree {
    fn default() -> Self {
        Self::Only("".to_string())
    }
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
