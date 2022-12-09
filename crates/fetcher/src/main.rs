mod getter;
mod types;
use getter::Getter;

use std::error;
use std::path::PathBuf;
use types::{ModuleDetails, ModuleSummary};

/// returns a full url with the long stuff pre-pended
fn nusmods(ext: &str) -> String {
    format!("https://api.nusmods.com/v2/2022-2023/{ext}")
}

fn crate_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let get_all_modules_summary = Getter::new(
        &nusmods("moduleList.json"),
        crate_dir().join("data/moduleList.json"),
    );
    let mut buffer: Vec<ModuleSummary> = Vec::new();
    get_all_modules_summary.get(&mut buffer).await?;
    let module_codes = buffer
        .iter()
        .map(|m| m.module_code.to_string())
        .collect::<Vec<_>>();
    println!("total count: {}", module_codes.len());

    // create getter for one module
    let code = "CS2040S";
    let url = nusmods(&format!("modules/{code}.json"));
    println!("url -> {url}");
    let path = crate_dir().join("data").join(&format!("{code}.json"));
    let getter = Getter::new(&url, path);
    // buffer for one module
    let mut buffer = ModuleDetails::default();
    // getter.debug_fetch().await?;
    // getter.clear_cache().ok();
    getter.get(&mut buffer).await?;

    println!("result -> {:?}", buffer);
    Ok(())
}
