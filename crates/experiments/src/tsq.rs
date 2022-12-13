use std::sync::Mutex;

pub struct TSQ<T> {
    data: Mutex<Vec<T>>,
}

impl<T> TSQ<T> {
    pub fn new() -> Self {
        Self { data: Mutex::new(vec![]) }
    }
    pub fn push(&self, elem: T) {
        let mut data = self.data.lock().unwrap();
        data.push(elem);
    }
    pub fn len(&self) -> usize {
        self.data.lock().unwrap().len()
    }
}
