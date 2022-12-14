use crate::plan::Plan;
use crate::structs::*;
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

fn sem(x: i32) -> String {
    match x {
        1 => "Sem 1",
        2 => "Sem 2",
        3 => "S.T.1",
        4 => "S.T.2",
        _ => "[invalid semester]",
    }
    .to_string()
}

impl fmt::Debug for Plan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sems = self.semesters();
        let sems = sems
            .iter()
            .enumerate()
            .map(|(i, list)| {
                format!("Y{}:{} -> {list:?}", i / 4 + 1, sem(i as i32 % 4 + 1))
            })
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "[plan]\nMatric: {:?}\n[semesters]\n{sems}", self.matric())
    }
}
