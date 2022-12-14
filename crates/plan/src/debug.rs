use crate::structs::*;
use crate::plan::Plan;
use std::fmt;

impl fmt::Debug for Period {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let year = self.year();
        let sem = match self.sem() {
            1 => "Semester 1",
            2 => "Semester 2",
            3 => "Special Term 1",
            4 => "Special Term 2",
            _ => "[invalid semester]",
        };
        write!(f, "{year}, {sem}")
    }
}

impl fmt::Debug for Plan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[plan]\nMatric: {:?}\n[semesters]\n{:?}", self.matric(), self.semesters())
    }
}
