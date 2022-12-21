use crate::dump::TMP_DUMP;
use std::env;
use std::path::PathBuf;
use std::process::Command;
use types::{Error, Result};

pub struct Docker {
    pub username: String,
    pub password: String,
    /// Location of git-indexed dump
    pub dump_dir: PathBuf,
    /// Mongo's static `/data/db` within the container
    pub container_dir: PathBuf,
}

impl Docker {
    /// Reads .env file for username and password.
    pub fn new() -> Result<Self> {
        dotenv::dotenv().expect(".env file not found");
        Ok(Self {
            username: env::var("MONGO_DB_USERNAME")?,
            password: env::var("MONGO_DB_PASSWORD")?,
            dump_dir: abs_dir("MONGO_DB_DUMP_DIR")?,
            container_dir: PathBuf::from("data/db"),
        })
    }

    /// Path to dump within the container
    pub fn tmp_dump(&self) -> String {
        let tmp_dump = self.container_dir.join(TMP_DUMP);
        tmp_dump.to_string_lossy().to_string()
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

/// Run and report the output of a command if it failed to exit gracefully.
pub fn report(cmd: &mut Command) {
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
