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
    ("cp", $src:expr,$dst:expr) => {{
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

#[allow(unused)]
pub fn create() -> Result<()> {
    let docker = Docker::new()?;
    let tmp_dump = docker.tmp_dump();
    fs::remove_dir_all(&docker.dump_dir).ok();
    docker!(
        "sh",
        "mongodump -u={u} -p={p} --out={tmp_dump}",
        u = docker.username,
        p = docker.password,
    );
    docker!("cp", format!("{CONTAINER}:{tmp_dump}"), &docker.dump_dir);
    docker!("sh", "rm -rf {tmp_dump}");
    Ok(())
}

#[allow(unused)]
pub fn restore() -> Result<()> {
    let docker = Docker::new()?;
    let tmp_dump = docker.tmp_dump();
    docker!("sh", "rm -rf {tmp_dump}");
    docker!("cp", docker.dump_dir, format!("{CONTAINER}:{tmp_dump}"));
    docker!(
        "sh",
        "mongorestore -u={u} -p={p} {tmp_dump}",
        u = docker.username,
        p = docker.password,
    );
    docker!("sh", "rm -rf {tmp_dump}");
    Ok(())
}

fn report(cmd: &mut Command) {
    let output = match cmd.output() {
        Ok(v) => v,
        _ => return println!("Command failed to spawn: {cmd:?}"),
    };
    if !output.status.success() {
        println!("Database dump failed.");
        println!("[stdout] {:?}", String::from_utf8_lossy(&output.stdout));
        println!("[stderr] {:?}", String::from_utf8_lossy(&output.stderr));
    }
}

/// Get a path from an environment variable, and ensure that it is absolute
fn abs_dir(env_var: &str) -> Result<PathBuf> {
    match env::var(env_var).map(PathBuf::from) {
        Ok(dir) if dir.is_absolute() => Ok(dir),
        Ok(dir) => Err(Error::RequiresAbsolutePath(dir))?,
        Err(err) => Err(Box::new(err)),
    }
}
