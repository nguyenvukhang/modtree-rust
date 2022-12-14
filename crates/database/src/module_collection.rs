use crate::ModuleCollection;
use prereqtree::PrereqTree;
use std::collections::{HashMap, HashSet};
use types::{Module, Result};

impl ModuleCollection {
    /// Obtain every requirement of a list of codes until all leaf nodes are
    /// reached.
    pub async fn flatten_requirements(
        &self,
        codes: Vec<String>,
        acad_year: &str,
    ) -> Result<Vec<Module>> {
        let mut remain = codes;
        let mut result: HashSet<Module> = HashSet::new();
        let mut fetched: HashSet<String> = HashSet::new();
        while !remain.is_empty() {
            let response = self.find_many(&remain, &acad_year).await?;
            remain = vec![];
            for (code, module) in response {
                let prereqs = match module {
                    Ok(v) => {
                        fetched.insert(code.to_string());
                        let prereqs = v.prereqtree_flatten();
                        result.insert(v);
                        prereqs
                    }
                    Err(_) => {
                        fetched.insert(code);
                        continue;
                    }
                };
                remain.extend(
                    prereqs.into_iter().filter(|c| !fetched.contains(c)),
                );
            }
        }
        Ok(Vec::from_iter(result))
    }

    pub fn topological_sort(
        modules: Vec<(String, Module)>,
    ) -> Vec<(String, Module)> {
        let mut hash = HashMap::new();
        let mut sorter = vec![];
        for (code, module) in modules {
            sorter.push((module.to_code(), module.to_prereqtree()));
            hash.insert(code, module);
        }
        PrereqTree::topological_sort(sorter)
            .into_iter()
            .map(|(code, _)| {
                let module = std::mem::take(hash.get_mut(&code).unwrap());
                (code, module)
            })
            .collect()
    }
}
