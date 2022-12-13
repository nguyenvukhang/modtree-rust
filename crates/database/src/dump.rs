use crate::database::Database;
use crate::Client;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};
use types::{Error, Result};

const TMP_DUMP: &str = "rust_dump";
const CONTAINER: &str = "modtree-mongo-db";
const SH: [&str; 5] = ["exec", "-t", CONTAINER, "sh", "-c"];

struct Docker {
    username: String,
    password: String,
    /// Location of git-indexed dump
    dump_dir: PathBuf,
    /// Mongo's static `/data/db` within the container
    container_dir: PathBuf,
}

macro_rules! docker {
    ("sh", $($arg:tt)*) => {{
        report(Command::new("docker").args(SH).arg(format!($($arg)*)));
    }};
    ("cp", $src:expr, $dst:expr) => {{
        report(Command::new("docker").arg("cp").arg($src).arg($dst));
    }};
}

impl Docker {
    /// Reads .env file for username and password.
    fn new() -> Result<Self> {
        dotenv::dotenv().expect(".env file not found");
        Ok(Self {
            username: env::var("MONGO_DB_USERNAME")?,
            password: env::var("MONGO_DB_PASSWORD")?,
            dump_dir: abs_dir("MONGO_DB_DUMP_DIR")?,
            container_dir: PathBuf::from("data/db"),
        })
    }

    /// Path to dump within the container
    fn tmp_dump(&self) -> String {
        let tmp_dump = self.container_dir.join(TMP_DUMP);
        tmp_dump.to_string_lossy().to_string()
    }
}

/// Create a dump for exactly one database
pub fn create(db: &str) -> Result<()> {
    let docker = Docker::new()?;
    let tmp_dump = docker.tmp_dump();
    fs::remove_dir_all(&docker.dump_dir.join(db)).ok();
    fs::create_dir_all(&docker.dump_dir).ok();
    docker!("sh", "rm -rf {tmp_dump}");
    docker!("sh", "mkdir {tmp_dump}");
    docker!(
        "sh",
        "mongodump -u={u} -p={p} --out={tmp_dump}",
        u = docker.username,
        p = docker.password,
    );
    let db_dump = PathBuf::from(&tmp_dump).join(db);
    let db_dump = db_dump.to_str().unwrap();
    docker!("cp", format!("{CONTAINER}:{db_dump}"), &docker.dump_dir.join(db));
    docker!("sh", "rm -rf {tmp_dump}");
    Ok(())
}

pub fn restore(db: &str) -> Result<()> {
    let docker = Docker::new()?;
    let tmp_dump = docker.tmp_dump();
    docker!("sh", "rm -rf {tmp_dump}");
    docker!("sh", "mkdir {tmp_dump}");
    docker!("cp", docker.dump_dir.join(db), format!("{CONTAINER}:{tmp_dump}"));
    docker!(
        "sh",
        "mongorestore -u={u} -p={p} {tmp_dump}",
        u = docker.username,
        p = docker.password,
    );
    docker!("sh", "rm -rf {tmp_dump}");
    Ok(())
}

/// Builds an entire dump from scratch.
///
/// Requires either an internet connection (to pull from NUSMods), or an existing
/// local cache of NUSMods data.
async fn build(db: &Database) -> Result<()> {
    let mods = db.modules();
    mods.drop().await?;
    mods.import_academic_year("2021-2022", None).await?;
    mods.import_academic_year("2022-2023", None).await?;
    Ok(())
}

/// Validate the state of a particular database.
/// Expect errors to throw here if things aren't ok.
///
/// Checks if all documents in the database's `module` collection can
/// be successfully parsed as a typed `Module` in Rust.
pub async fn validate_modules(db: &Database) {
    let collection = db.modules();

    let total = collection.count().await.unwrap();
    println!("[dump:validate] total documents database: {total}");

    // try to load every module into a Rust struct.
    let modules = collection.list_all().await.unwrap();
    eprintln!("[dump:validate] successfully parsed: {}", modules.len());

    // ensure that all documents are successfully parsed.
    assert_eq!(total, modules.len() as u64);
}

#[allow(unused)]
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

fn report(cmd: &mut Command) {
    let mut cmd_debug = vec![String::from(cmd.get_program().to_string_lossy())];
    cmd_debug.extend(cmd.get_args().map(|v| String::from(v.to_string_lossy())));
    let output = match cmd.output() {
        Ok(v) => v,
        _ => return println!("Command failed to spawn: {cmd:?}"),
    };
    if !output.status.success() {
        eprintln!("Database dump failed.");
        eprintln!("[cmd] {:?}", cmd_debug);
        eprintln!("[stdout] {:?}", String::from_utf8_lossy(&output.stdout));
        eprintln!("[stderr] {:?}", String::from_utf8_lossy(&output.stderr));
    }
}

/// Get a path from an environment variable, and ensure that it is absolute
fn abs_dir(env_var: &str) -> Result<PathBuf> {
    match env::var(env_var).map(PathBuf::from) {
        Ok(dir) if dir.is_absolute() => Ok(dir),
        Ok(dir) => Err(Error::RequiresAbsolutePath(dir))?,
        Err(err) => Err(err)?,
    }
}
