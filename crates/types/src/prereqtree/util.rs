/// Compares vectors up to and including length of 200.
pub fn vec_eq<T, F: Fn(&T, &T) -> bool>(
    a: &Vec<T>,
    b: &Vec<T>,
    cmp: F,
) -> bool {
    if a.len() > 200 || b.len() > 200 || a.len() != b.len() {
        return false;
    }
    let (a, b, len) = (a.as_slice(), b.as_slice(), a.len());
    let (mut ok_a, mut ok_b) = ([false; 200], [false; 200]);
    for idx_a in 0..len {
        if ok_a[idx_a] {
            continue;
        }
        for idx_b in 0..len {
            if ok_b[idx_b] {
                continue;
            }
            if cmp(&a[idx_a], &b[idx_b]) {
                ok_a[idx_a] = true;
                ok_b[idx_b] = true;
            }
        }
    }
    (0..len).fold(true, |a, x| a && ok_a[x] && ok_b[x])
}

#[test]
fn test_vec_eq() {
    macro_rules! test {
        ($a:expr, $b:expr, $expected:expr) => {
            let [a, b]: [Vec<u8>; 2] = [$a, $b];
            assert!(!$expected ^ vec_eq(&a, &b, |a, b| a.eq(b)));
        };
    }
    test!(vec![1, 2, 3, 4], vec![4, 3, 2, 1], true);
    test!(vec![2, 3, 4], vec![4, 3, 1], false);
    test!(vec![2, 3, 4], vec![4, 3, 1], false);
    test!(vec![], vec![1], false);
}

/// Counter that allows a different base for each digit.
/// For example, with bases [5, 3, 2], the count goes
/// 0 -> 1 -> 10 -> 11 -> 20 -> 21 -> 100
struct Counter {
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
    fn new(bases: Vec<usize>) -> Self {
        Self { count: vec![0; bases.len()], bases, done: false }
    }
    /// Returns false only upon reset
    fn _increment(&mut self, place: Option<usize>) -> bool {
        let place = match place {
            None => return false,
            Some(v) if v >= self.bases.len() => return false,
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
    fn increment(&mut self) {
        if !self._increment(self.bases.len().checked_sub(1)) {
            self.done = true
        }
    }
}
