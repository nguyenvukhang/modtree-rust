use std::hash::Hash;
use types::{Error, Result};

/// Period(<year>, <semester>)
#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Period(i32, i32);

impl Period {
    pub fn new(year: i32, sem: i32) -> Result<Self> {
        if !(1 <= sem && sem <= 4) {
            Err(Error::InvalidSemester)
        } else {
            Ok(Self(year, sem))
        }
    }
    pub fn year(&self) -> i32 {
        self.0
    }
    pub fn acad_year(&self) -> String {
        format!("{}/{}", self.0, self.0 + 1)
    }
    pub fn sem(&self) -> i32 {
        self.1
    }
}
