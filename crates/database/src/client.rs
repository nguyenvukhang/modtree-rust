use mongodb::options::{ClientOptions, Credential, ServerAddress};
use std::env;
use types::Result;

/// wrapper for the standard mongo-db client with project-specific tooling
pub struct Client(mongodb::Client);

impl Client {
    /// Creates a new mongo-db client, which will have access to databases.
    pub async fn new() -> Result<Self> {
        dotenv::dotenv().expect(".env file not found");
        let creds = Credential::builder()
            .username(env::var("MONGO_DB_USERNAME").ok())
            .password(env::var("MONGO_DB_PASSWORD").ok())
            .build();
        let opts = ClientOptions::builder()
            .hosts(vec![ServerAddress::parse("localhost:27017")?])
            .credential(creds)
            .build();
        Ok(Self(mongodb::Client::with_options(opts)?))
    }

    async fn first_time_setup(&mut self) -> Result<()> {
        // list of all collections used.
        let collections = ["modules"];
        let db = self.modtree_db().await?;
        for name in collections {
            db.create_collection(name, None).await?;
        }
        Ok(())
    }

    /// The database of modtree.
    pub async fn modtree_db(&mut self) -> Result<mongodb::Database> {
        Ok(self.0.database("modtree"))
    }

    pub async fn database(&mut self, name: &str) -> Result<mongodb::Database> {
        Ok(self.0.database(name))
    }

    /// Gets a list of all database names in list of `String`s.
    pub async fn list_all_database_names(&mut self) -> Result<Vec<String>> {
        Ok(self.0.list_database_names(None, None).await?)
    }

    /// Drops (deletes) all databases except for "admin", "config", and "local".
    pub async fn drop_all_databases(&mut self) -> Result<()> {
        let databases = self.list_all_database_names().await?;
        let to_delete = databases
            .iter()
            .filter(|name| match name.as_ref() {
                "admin" | "config" | "local" => false,
                _ => true,
            })
            .map(|name| self.0.database(name));
        for db in to_delete {
            println!("deleting database [{}]", db.name());
            db.drop(None).await?;
        }
        Ok(())
    }
}
