use std::process::Command;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

const CHILL: Duration = Duration::new(0, 100_000_000);

// unordred buf
struct BufIter {
    done: Arc<AtomicBool>,
}

fn sleep() {
    Command::new("sleep").arg("0.1").output().ok();
}

impl BufIter {
    fn new() -> Self {
        Self { done: Arc::new(AtomicBool::new(false)) }
    }

    fn run(&mut self) {
        let done = Arc::clone(&self.done);
        thread::spawn(move || {
            for i in 0..10 {
                sleep();
                println!("{i}")
            }
            done.store(true, Ordering::SeqCst);
        });
    }

    fn is_done(&self) -> bool {
        self.done.load(Ordering::Relaxed)
        // self.total == self.core.len()
    }
}

#[allow(unused)]
fn main() {
    let mut bufiter: BufIter = BufIter::new();
    bufiter.run();
    while !bufiter.is_done() {
        println!("waiting...");
        thread::sleep(CHILL)
    }

    println!("Hello, world!");
}
