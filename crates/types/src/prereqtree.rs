use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrereqTree {
    None,
    String(String),
    Node {
        #[serde(default)]
        and: Vec<PrereqTree>,
        #[serde(default)]
        or: Vec<PrereqTree>,
    },
}

impl Default for PrereqTree {
    fn default() -> Self {
        Self::String("".to_string())
    }
}
