use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

pub struct TSQ<T> {
    data: Mutex<Vec<T>>,
    done: AtomicBool,
}

impl<T> TSQ<T> {
    pub fn new() -> Self {
        Self { data: Mutex::new(vec![]), done: AtomicBool::new(false) }
    }
    pub fn push(&self, elem: T) {
        let mut data = self.data.lock().unwrap();
        data.push(elem);
    }
    pub fn pop(&self) -> Option<T> {
        self.data.lock().unwrap().pop()
    }
    pub fn len(&self) -> usize {
        self.data.lock().unwrap().len()
    }
    pub fn done(&self) {
        self.done.store(true, Ordering::SeqCst)
    }
    pub fn is_done(&self) -> bool {
        self.done.load(Ordering::Relaxed)
    }
}
