mod getter;
mod types;
use getter::Getter;

use futures;
use futures::prelude::*;
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

    // module_codes.iter().for_each(|code| {
    //     let path = crate_dir().join("data").join(&format!("{code}.json"));
    //     let getter = Getter::new("", path);
    //     getter.clear_cache().ok();
    // });
    let total = module_codes.len();

    let handles = module_codes
        .iter()
        .enumerate()
        .map(|(i, code)| {
            println!("[{i}/{total}]: {code}");
            let url = nusmods(&format!("modules/{code}.json"));
            let path = crate_dir().join("data").join(&format!("{code}.json"));
            Getter::new(&url, path)
        })
        .filter(|g| !g.already_exists())
        .map(|getter| async move { getter.get_module().await });
    let stream = futures::stream::iter(handles).buffer_unordered(36);

    let results = stream.collect::<Vec<_>>().await;
    let failed: Vec<_> = results.iter().filter(|v| v.is_err()).collect();
    println!("total done: {:?}", failed);

    Ok(())
}
