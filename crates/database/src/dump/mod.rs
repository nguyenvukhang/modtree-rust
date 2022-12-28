mod docker;

use crate::{Client, Database};
use docker::report;
use docker::Docker;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use types::Result;

const TMP_DUMP: &str = "rust_dump";
const CONTAINER: &str = "modtree-mongo-db";
const SH: [&str; 5] = ["exec", "-t", CONTAINER, "sh", "-c"];

macro_rules! docker {
    (sh, $($arg:tt)*) => {{
        report(Command::new("docker").args(SH).arg(format!($($arg)*)));
    }};
    (cp, $src:expr, $dst:expr) => {{
        report(Command::new("docker").arg("cp").arg($src).arg($dst));
    }};
}

/// Create a dump for exactly one database.
pub fn create(db: &str) -> Result<()> {
    let docker = Docker::new()?;
    let tmp_dump = docker.tmp_dump();
    fs::remove_dir_all(&docker.dump_dir.join(db)).ok();
    fs::create_dir_all(&docker.dump_dir).ok();
    docker!(sh, "rm -rf {tmp_dump}");
    docker!(sh, "mkdir {tmp_dump}");
    docker!(
        sh,
        "mongodump -u={u} -p={p} --out={tmp_dump}",
        u = docker.username,
        p = docker.password,
    );
    let db_dump = PathBuf::from(&tmp_dump).join(db);
    let db_dump = db_dump.to_str().unwrap();
    docker!(cp, format!("{CONTAINER}:{db_dump}"), &docker.dump_dir.join(db));
    docker!(sh, "rm -rf {tmp_dump}");
    Ok(())
}

/// Restore a dump of exactly one database.
pub fn restore(db: &str) -> Result<()> {
    let docker = Docker::new()?;
    let tmp_dump = docker.tmp_dump();
    docker!(sh, "rm -rf {tmp_dump}");
    docker!(sh, "mkdir {tmp_dump}");
    docker!(cp, docker.dump_dir.join(db), format!("{CONTAINER}:{tmp_dump}"));
    docker!(
        sh,
        "mongorestore -u={u} -p={p} {tmp_dump}",
        u = docker.username,
        p = docker.password,
    );
    docker!(sh, "rm -rf {tmp_dump}");
    Ok(())
}

/// Uses a full import routine to check the validity of the current schema.
pub async fn test_schema() {
    const DB_NAME: &str = "test_dump";
    const DB_URL: &str = "localhost:27017";
    let mut client = Client::new("localhost:27017").await.unwrap();
    client.assert_running().unwrap();
    let db = client.test_db(DB_NAME);
    let m = db.modules();

    // build from scratch
    client.drop_database(DB_NAME).await.unwrap();
    assert!(m.count().await.map_or(false, |v| v == 0));
    build(&db).await.unwrap();
    create(DB_NAME).unwrap();

    // clear the database
    client.drop_database(DB_NAME).await.unwrap();
    assert!(m.count().await.map_or(false, |v| v == 0));

    // restore the database from the dump
    restore(DB_NAME).unwrap();
    validate_modules(&db).await;

    // post-test teardown
    client.drop_database(DB_NAME).await.unwrap();
    assert!(m.count().await.map_or(false, |v| v == 0));
}

/// Builds an entire database from scratch.
///
/// Requires either an internet connection (to pull from NUSMods), or an existing
/// local cache of NUSMods data.
pub async fn build(db: &Database) -> Result<()> {
    let mods = db.modules();
    mods.drop().await?;
    mods.import_academic_year("2021-2022", None).await?;
    mods.import_academic_year("2022-2023", None).await?;
    Ok(())
}

/// Builds an entire dump from scratch.
///
/// Requires either an internet connection (to pull from NUSMods), or an existing
/// local cache of NUSMods data.
pub async fn bootstrap() {
    let mut client = Client::new("localhost:27017").await.unwrap();
    client.assert_running().unwrap();
    let db = client.modtree_db();
    build(&db).await.unwrap();
    create(db.name()).unwrap();
}

/// Validate the state of a particular database.
/// Expect errors to throw here if things aren't ok.
///
/// Checks if all documents in the database's `module` collection can
/// be successfully parsed as a typed `Module` in Rust.
async fn validate_modules(db: &Database) {
    let collection = db.modules();

    let total = collection.count().await.unwrap();
    println!("[dump:validate] total documents database: {total}");

    // try to load every module into a Rust struct.
    let modules = collection.list_all().await.unwrap();
    eprintln!("[dump:validate] successfully parsed: {}", modules.len());

    // ensure that all documents are successfully parsed.
    assert_eq!(total, modules.len() as u64);
}
