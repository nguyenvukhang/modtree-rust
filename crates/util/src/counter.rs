/// Counter that allows a different base for each digit.
/// For example, with bases [5, 3, 2], the count goes
/// 0 -> 1 -> 10 -> 11 -> 20 -> 21 -> 100
pub struct Counter {
    count: Vec<usize>, // stores count in reverse order
    bases: Vec<usize>,
    done: bool,
}

impl Iterator for Counter {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.done {
            true => return None,
            false => self.count.clone(),
        };
        self.increment();
        Some(result)
    }
}

impl Counter {
    pub fn new(bases: Vec<usize>) -> Self {
        Self { count: vec![0; bases.len()], bases, done: false }
    }
    /// Returns false only upon reset
    fn _increment(&mut self, place: Option<usize>) -> bool {
        let place = match place {
            None => return false,
            Some(v) if v >= self.bases.len() => return false, // RESET
            Some(v) => v,
        };
        if self.count[place] + 1 == self.bases[place] {
            self.count[place] = 0;
            self._increment(place.checked_sub(1))
        } else {
            self.count[place] += 1;
            true
        }
    }
    pub fn increment(&mut self) {
        if !self._increment(self.bases.len().checked_sub(1)) {
            self.done = true
        }
    }
}
