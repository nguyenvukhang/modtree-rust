use database::client;

#[tokio::main]
async fn main() {
    let (_, db) = client::Client::debug_init().await.unwrap();
    let module_collection = db.modules();
    let flat = module_collection
        .flatten_requirements("CS3244", "2022/2023")
        .await
        .unwrap();
    println!("flat->{flat:?}");

    plan::main();
}
