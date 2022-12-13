mod buf_iter;
mod tsq;
use std::process::Command;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
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
    stdout.trim().parse::<u8>().unwrap()
}

impl Data {
    fn new() -> Self {
        Self { data: Arc::new(TSQ::new()), thread: None }
    }

    fn main() {
        println!("Data::main()");
        let mut core = Data::new();
        core.gather();
        while core.data.len() < 10 {
            println!("waiting");
            thread::sleep(Duration::new(0, 100_000_100));
        }
        let a = core.thread.map(|v| v.join());
        println!("{a:?}")
    }

    fn gather(&mut self) {
        let data = Arc::clone(&self.data);
        let t = thread::spawn(move || {
            for i in 0..20 {
                // sleep();
                fetch(i);
                data.push(i);
                println!("{i}")
            }
            "hello!".to_string()
        });
        self.thread = Some(t);
    }
}

fn main() {
    println!("{}", fetch(1));
    Data::main()
}
