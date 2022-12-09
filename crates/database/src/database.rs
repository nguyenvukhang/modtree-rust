use mongodb::options::{ClientOptions, Credential, ServerAddress};
use std::env;
use types::Result;

/// wrapper for the standard mongo-db database with project-specific tooling
pub struct Database(mongodb::Database);

// impl Database {
//     pub async fn add
//
// }
