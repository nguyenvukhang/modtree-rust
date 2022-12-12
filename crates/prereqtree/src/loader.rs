use crate::PrereqTree;
use std::collections::HashMap;
use std::future::Future;

/// A helper struct to load prereqtrees from a database.
pub struct Loader<F> {
    source: F,
    cache: HashMap<String, PrereqTree>,
}

impl<F, R> Loader<F>
where
    F: Fn(String) -> R,
    R: Future<Output = Option<PrereqTree>>,
{
    /// Initialize a loader with a function that returns a prereqtree based
    /// on a module code
    pub fn new(source: F) -> Self {
        Self { source, cache: HashMap::new() }
    }

    pub async fn get(&mut self, code: String) -> Option<PrereqTree> {
        let source = &self.source;
        if let Some(tree) = self.cache.get(&code) {
            return Some(tree.clone());
        }
        match source(code.clone()).await {
            Some(tree) => {
                self.cache.insert(code, tree.clone());
                Some(tree)
            }
            None => None,
        }
    }
}
