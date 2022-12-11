/// Convenience routine for prereqtree testing
#[macro_export]
macro_rules! t {
    () => {
        crate::prereqtree::PrereqTree::Only("".to_string())
    };
    (and, $($module:expr),*) => {
        crate::prereqtree::PrereqTree::And { and: vec![ $(($module),)*] }
    };
    (or, $($module:expr),*) => {
        crate::prereqtree::PrereqTree::Or { or: vec![ $(($module),)*] }
    };
    (done, $($module:ident),*) => {{
        let mut vec: Vec<String> = vec![];
        $(vec.push(stringify!($module).to_string());)*
        type S = std::collections::HashSet<String>;
        let set: S = HashSet::from_iter(vec.into_iter());
        set
    }};
    (none) => {
        std::collections::HashSet::new()
    };
    ($t:ident) => {
        crate::prereqtree::PrereqTree::Only(stringify!($t).to_string())
    };
}
