mod loader;
use loader::Loader;

use futures;
use futures::prelude::*;
use types::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let loader = Loader::new()?;
    let (module_list, _) = loader.load_module_list().await?;
    let module_count = module_list.len();
    let handles = module_list.iter().enumerate().map(|(i, m)| async move {
        println!("[{i}/{module_count}]");
        Loader::new().unwrap().load_module(&m.module_code).await
    });
    let stream = futures::stream::iter(handles).buffer_unordered(36);
    let results = stream.collect::<Vec<_>>().await;
    let local =
        results.iter().filter_map(|v| v.as_ref().ok()).filter(|v| v.1).count();
    let ok = results.iter().filter_map(|v| v.as_ref().ok()).count();
    println!("local: {}", local);
    println!("ok: {}", ok);
    Ok(())
}
