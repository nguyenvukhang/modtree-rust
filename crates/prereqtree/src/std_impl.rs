use util::vec_eq;
use crate::PrereqTree;
use serde_json::to_string_pretty;
use std::fmt;

impl From<nusmods::PrereqTree> for PrereqTree {
    fn from(t: nusmods::PrereqTree) -> Self {
        use nusmods::PrereqTree as N;
        match t {
            N::Or { or } => {
                Self::Or { or: or.into_iter().map(Self::from).collect() }
            }
            N::And { and } => {
                Self::And { and: and.into_iter().map(Self::from).collect() }
            }
            N::Only(v) => match v {
                v if v.is_empty() => Self::empty(),
                v => Self::Only(v),
            },
        }
    }
}

impl fmt::Debug for PrereqTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Only(v) => write!(f, "{v}"),
            Self::Or { or: t } => write!(f, "OR {t:?}"),
            Self::And { and: t } => write!(f, "AND {t:?}"),
        }
    }
}

impl fmt::Display for PrereqTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let t = to_string_pretty(self).unwrap_or("[Invalid tree]".to_string());
        write!(f, "{t}")
    }
}

impl Default for PrereqTree {
    fn default() -> Self {
        Self::Only("".to_string())
    }
}

/// Note that when two `PrereqTree`s are equal, they are not only logically
/// equal but structurally equal also. Within each layer, order does not matter.
impl PartialEq for PrereqTree {
    fn eq(&self, other: &PrereqTree) -> bool {
        use PrereqTree::*;
        match (self, other) {
            (Or { or: x }, Or { or: y }) => vec_eq(x, y, |a, b| a.eq(b)),
            (And { and: x }, And { and: y }) => vec_eq(x, y, |a, b| a.eq(b)),
            (Only(x), Only(y)) => x.eq(y),
            _ => false,
        }
    }
}
impl Eq for PrereqTree {}

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
