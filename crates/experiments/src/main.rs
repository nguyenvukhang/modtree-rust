use futures::prelude::*;
use futures::stream::iter;
mod serialize;
mod tsq;

/// takes 200ms to fetch a u8 artificially
async fn read_from_database() -> String {
    println!("start exec");
    let stdout = std::process::Command::new("sh")
        .arg("-c")
        .arg(format!("sleep 0.4 && echo '<data>'"))
        .output()
        .unwrap()
        .stdout;
    let stdout = String::from_utf8_lossy(&stdout).to_string();
    stdout.trim().parse().unwrap()
}

#[tokio::main]
async fn main() {
    let mut t = vec![];
    for i in 0..5 {
        let data = tokio::spawn(async move {
            format!("[{i}] -> {}", read_from_database().await)
        });
        let formatted = data.map(|v| format!("fmt->{v:?}"));
        t.push(formatted);
    }

    let mut res = vec![];
    for i in t {
        let f = i.await;
        println!("-> {f}");
        res.push(f);
    }
}
