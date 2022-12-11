mod loader;
mod util;
use loader::Loader;
use types::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // let loader = Loader::new("2021-2022")?;
    let loader = Loader::new("2022-2023")?;
    let results = loader.load_all_modules(None).await?;
    let count = results.len();
    println!("loaded count: {count}");
    Ok(())
}
