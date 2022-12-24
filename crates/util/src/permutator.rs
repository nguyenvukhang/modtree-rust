/// Permutator of a maximum of 16 values
#[derive(Debug)]
struct Permutator<T> {
    data: Vec<T>,
    done: bool,
    mask: Vec<bool>,
}

impl<T> Permutator<T> {
    fn new(data: Vec<T>, choose: usize) -> Self {
        let mut mask = vec![false; data.len()];
        let len = mask.len();
        (0..choose).for_each(|i| mask[len - i - 1] = true);
        Self { data, mask, done: false }
    }
}

impl<T> Iterator for Permutator<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let len = self.data.len();
        for i in 0..len - 1 {
            let i = len - i - 1;
            println!("{i}");
        }
        Some(vec![])
    }
}

#[test]
fn permutator_test() {
    let mut perm = Permutator::new(vec![1, 2, 3], 2);
    perm.next();
    println!("GOT HERE -> {perm:?}");
    assert!(false)
}
