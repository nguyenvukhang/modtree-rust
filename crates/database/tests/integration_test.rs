/// These require correctly importing data from AY2021/2022 and AY2022/2023.
use database::Client;
use types::Result;
use util::vec_eq;

fn s_vec(v: &[&str]) -> Vec<String> {
    v.iter().map(|v| v.to_string()).collect()
}

#[tokio::test]
async fn flatten_requirements_test() -> Result<()> {
    let collection = Client::debug_init().await?;
    let modules = s_vec(&["CS3244"]);
    let expected = s_vec(&[
        "CS3244", // 0
        "CS2040", //  1
        "MA1513", "MA1508E", // 2
        "MA1505", "MA1511", "MA1512", "MA1521", // 3
        "ST2131", "ST2334", "EE2012", // 4
        "MA1301X", "MA1301", // needed by MA1505
        "CS1010", // needed by CS2040
        "MA2002", "MA1312", // needed by ST2334
    ]);
    let received = collection
        .flatten_requirements(modules, "2022/2023")
        .await?
        .into_iter()
        .map(|v| v.to_code())
        .collect();
    assert!(vec_eq(&received, &expected, |a, b| a.eq(b)));
    println!("received: {received:?}");
    println!("expected: {expected:?}");
    Ok(())
}
