use crate::database::Database;
use mongodb::options::{ClientOptions, Credential, ServerAddress};
use std::env;
use types::{Error, Result};

/// wrapper for the standard mongo-db client with project-specific tooling
#[derive(Debug)]
pub struct Client {
    c: mongodb::Client,
    /// (<host>, <port>)
    addr: (String, u16),
}

impl Client {
    /// Creates a new mongo-db client, which will have access to databases.
    pub async fn new(url: &str) -> Result<Self> {
        dotenv::dotenv().expect(".env file not found");
        let creds = Credential::builder()
            .username(env::var("MONGO_DB_USERNAME").ok())
            .password(env::var("MONGO_DB_PASSWORD").ok())
            .build();
        let opts = ClientOptions::builder()
            .hosts(vec![ServerAddress::parse(&url)?])
            .credential(creds)
            .build();
        Ok(Self {
            c: mongodb::Client::with_options(opts)?,
            addr: parse_url(&url)?,
        })
    }

    pub fn assert_running(&self) -> Result<()> {
        use std::net::TcpStream;
        let connection = TcpStream::connect(&self.addr)
            .map_err(|_| Error::MongoDbClientNotRunning)
            .map(|_| ());
        Ok(connection?)
    }

    /// The database of modtree.
    pub async fn modtree_db(&mut self) -> Database {
        Database::new(self.c.database("modtree"))
    }

    pub async fn database(&mut self, name: &str) -> Result<mongodb::Database> {
        Ok(self.c.database(name))
    }

    /// Gets a list of all database names in list of `String`s.
    pub async fn list_all_database_names(&mut self) -> Result<Vec<String>> {
        Ok(self.c.list_database_names(None, None).await?)
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
            .map(|name| self.c.database(name));
        for db in to_delete {
            println!("deleting database [{}]", db.name());
            db.drop(None).await?;
        }
        Ok(())
    }
}

/// Converts a url to a host and a port.
pub fn parse_url<A: AsRef<str>>(address: A) -> Result<(String, u16)> {
    let address = address.as_ref();
    let mut parts = address.split(':');
    let host = match parts.next() {
        Some(v) if !v.is_empty() => v,
        _ => Err(Error::InvalidData(format!("invalid host")))?,
    };
    let port = match parts.next().and_then(|v| v.parse::<u16>().ok()) {
        Some(v) if v > 0 => v,
        Some(v) if v == 0 => {
            Err(Error::InvalidData(format!("port must be non-zero")))?
        }
        Some(v) => Err(Error::InvalidData(format!(
            "port must be a valid 16-bit unsigned integer, instead got {v}"
        )))?,
        _ => Err(Error::InvalidData(format!("invalid port")))?,
    };
    if parts.next().is_some() {
        Err(Error::InvalidData(format!(
            "address {address} contains more than one unescaped ':'"
        )))?;
    }
    Ok((host.to_string(), port))
}
