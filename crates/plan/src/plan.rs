use crate::semester::Semester;
use std::collections::HashSet;
use std::fmt;

/// Plans can only be built chronologically. New modules can only be
/// added to the `current_year` at the `current_sem`, and these values
/// will only increment forwards.

#[derive(Clone)]
pub struct Plan {
    years: usize,
    done: HashSet<String>,
    current_year: usize,
    current_sem: usize,
}

impl Plan {
    pub fn new(years: usize) -> Self {
        Self { years, done: HashSet::new(), current_sem: 1, current_year: 1 }
    }
}

impl fmt::Debug for Plan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Plan")
            .field("Y/S", &(&self.current_year, &self.current_sem))
            .field("mods", &self.done)
            .finish()
    }
}
