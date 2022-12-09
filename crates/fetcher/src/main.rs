mod loader;
use futures::prelude::*;
use loader::Loader;
use std::sync::Arc;
use types::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let loader = Arc::new(Loader::new()?);
    let module_list = loader.load_module_list().await?;
    let codes: Vec<String> =
        module_list.iter().map(|v| v.module_code.to_string()).collect();
    let module_count = module_list.len();
    loader.reset_counts();
    let handles =
        codes.iter().enumerate().map(|v| (v.0, v.1, Arc::clone(&loader))).map(
            |(index, code, loader)| async move {
                println!("[{index}/{module_count}]");
                loader.load_module(code).await
            },
        );
    let stream = futures::stream::iter(handles).buffer_unordered(40);
    let _ = stream.collect::<Vec<_>>().await;
    loader.report();
    Ok(())
}
