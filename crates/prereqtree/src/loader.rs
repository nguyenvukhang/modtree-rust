use crate::PrereqTree;
use std::collections::{HashMap, HashSet};
use std::future::Future;

/// A helper struct to load prereqtrees from a database.
pub struct Loader<F> {
    source: F,
    cache: HashMap<String, PrereqTree>,
}

impl<F, R> Loader<F>
where
    F: Fn(Vec<String>) -> R,
    R: Future<Output = Option<HashMap<String, PrereqTree>>>,
{
    /// Initialize a loader with a function that returns a prereqtree based
    /// on a module code
    pub fn new(source: F) -> Self {
        Self { source, cache: HashMap::new() }
    }

    pub async fn load_trees(&mut self, codes: &Vec<String>) {
        let codes = codes
            .iter()
            .filter_map(|v| {
                (!self.cache.contains_key(v)).then_some(v.to_string())
            })
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let fetch_target = codes.len();
        if fetch_target == 0 {
            return;
        }
        let fetched = (&self.source)(codes).await.unwrap_or_default();
        if fetched.len() < fetch_target {
            eprintln!("WARNING: did not manage to fetch all requested trees.")
        }
        self.cache.extend(fetched);
    }

    pub fn get(&self, code: &str) -> Option<PrereqTree> {
        self.cache.get(code).map(|t| t.clone())
    }
}
