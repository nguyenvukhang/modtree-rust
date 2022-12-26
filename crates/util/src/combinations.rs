/// An iterator to iterate through all the `n`-length combinations in an
/// iterator.
pub struct Combinations<T> {
    n: usize,
    indices: Vec<usize>,
    pool: Vec<T>,
    first: bool,
}

/// Create a new `Combinations` from a clonable iterator.
pub fn combinations<T>(pool: Vec<T>, n: usize) -> Combinations<T>
where
    T: Sized,
{
    let mut indices: Vec<usize> = Vec::with_capacity(n);
    for i in 0..n {
        indices.push(i);
    }
    Combinations { n: n, indices: indices, pool, first: true }
}

impl<T> Iterator for Combinations<T>
where
    T: Clone,
{
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let pool_len = self.pool.len();
        if pool_len == 0 || self.n > pool_len {
            return None;
        }

        if self.first {
            self.first = false;
        } else if self.n == 0 {
            return None;
        } else {
            // Scan from the end, looking for an index to increment
            let mut i: usize = self.n - 1;

            while self.indices[i] == i + pool_len - self.n {
                if i > 0 {
                    i -= 1;
                } else {
                    // Reached the last combination
                    return None;
                }
            }

            // Increment index, and reset the ones to its right
            self.indices[i] += 1;
            let mut j = i + 1;
            while j < self.n {
                self.indices[j] = self.indices[j - 1] + 1;
                j += 1;
            }
        }

        // Create result vector based on the indices
        let mut result = Vec::with_capacity(self.n);
        for i in self.indices.iter() {
            result.push(self.pool[*i].clone());
        }
        Some(result)
    }
}

#[test]
fn combinations_test() {
    let mut c = combinations(vec![1, 2, 3, 4, 5], 3);
    while let Some(k) = c.next() {
        println!("{k:?}");
    }
    assert!(false)
}
