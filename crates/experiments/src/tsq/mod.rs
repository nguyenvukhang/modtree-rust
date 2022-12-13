mod tsq;

use std::process::Command;
use std::sync::Arc;
use std::thread;
use tsq::TSQ;

struct Data {
    data: Arc<tsq::TSQ<u8>>,
    thread: Option<thread::JoinHandle<String>>,
}

fn fetch(val: u8) -> u8 {
    let stdout = Command::new("sh")
        .arg("-c")
        .arg(format!("sleep 0.2 && echo '{val}'"))
        .output()
        .unwrap()
        .stdout;
    let stdout = String::from_utf8_lossy(&stdout).to_string();
    stdout.trim().parse().unwrap()
}

impl Iterator for Data {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let mut v = self.data.pop();
        while v.is_none() && !self.data.is_done() {
            v = self.data.pop();
        }
        match (v, self.data.is_done()) {
            (None, true) => self.close(),
            _ => {}
        };
        v
    }
}

impl Data {
    fn new() -> Self {
        Self { data: Arc::new(TSQ::new()), thread: None }
    }

    fn gather(&mut self) {
        let data = Arc::clone(&self.data);
        let t = thread::spawn(move || {
            for i in 0..20 {
                data.push(fetch(i));
            }
            data.done();
            "hello!".to_string()
        });
        self.thread = Some(t);
    }

    fn close(&mut self) {
        std::mem::take(&mut self.thread).map(|v| v.join());
    }
}

pub fn demo() {
    println!("Data::main()");
    let mut src = Data::new();
    src.gather();
    for i in src {
        println!("{i}")
    }
}
