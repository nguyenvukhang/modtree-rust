/// Convenience routine for prereqtree testing
#[macro_export]
macro_rules! t {
    () => {
        crate::PrereqTree::empty()
    };
    (and, $($module:expr),*) => {
        crate::PrereqTree::And { and: vec![ $(($module),)*] }
    };
    (or, $($module:expr),*) => {
        crate::PrereqTree::Or { or: vec![ $(($module),)*] }
    };
    ($t:ident) => {
        crate::PrereqTree::Only(stringify!($t).to_string())
    };
}
