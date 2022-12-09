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

#[allow(unused)]
async fn fetch_online(module_codes: &Vec<String>) {
    let total = module_codes.len();

    let handles = module_codes
        .iter()
        .enumerate()
        .map(|(i, code)| {
            let url = nusmods(&format!("modules/{code}.json"));
            let path = crate_dir().join("data").join(&format!("{code}.json"));
            Getter::new(&url, path)
        })
        .filter(|g| !g.already_exists())
        .map(|getter| async move {
            println!("{:?}", getter.path);
            getter.get_module().await
        });
    let stream = futures::stream::iter(handles).buffer_unordered(36);

    let results = stream.collect::<Vec<_>>().await;
    let failed: Vec<_> = results.iter().filter(|v| v.is_err()).collect();
    println!("total done: {:?}", failed);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let get_all_modules_summary = Getter::new(
        &nusmods("moduleList.json"),
        crate_dir().join("data/moduleList.json"),
    );
    let mut all_modules: Vec<ModuleSummary> = Vec::new();
    get_all_modules_summary.get(&mut all_modules).await?;
    let module_codes = all_modules
        .iter()
        .map(|m| m.module_code.to_string())
        .collect::<Vec<_>>();
    let getters = module_codes
        .iter()
        .map(|code| Getter::new("", crate_dir().join("data").join(&format!("{code}.json"))))
        .map(|g| g.get_module2());
    // println!("first: {:?}", getters.take(1).collect::<Vec<_>>());
    let results = getters.map(|g| g.is_ok()).collect::<Vec<_>>();
    let ok = results.iter().filter(|v| **v).collect::<Vec<_>>();
    let not = results.iter().filter(|v| !**v).collect::<Vec<_>>();
    println!("ok: {}", ok.len());
    println!("not: {}", not.len());
    println!("results: {:?}", results.get(0));

    Ok(())
}
