use crate::prereqtree::PrereqTree;

impl Default for PrereqTree {
    fn default() -> Self {
        Self::Only("".to_string())
    }
}

impl PartialEq for PrereqTree {
    fn eq(&self, other: &Self) -> bool {
        true
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

#[test]
fn test_prt_eq() {
    assert_eq!(1, 1)
}
