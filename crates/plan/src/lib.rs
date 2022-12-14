use std::collections::{HashMap, HashSet};
use types::Module;

struct Semester(HashSet<Module>);
impl Semester {
    pub fn insert(&mut self, m: Module) -> bool {
        self.0.insert(m)
    }
    pub fn remove(&mut self, code: String) -> bool {
        // calculate ay using matric_year and year number
        // self.0.insert(m)
        false
    }
}
struct Plan {
    matric_year: u64,
    semesters: HashMap<[u8; 2], Semester>,
}

impl Plan {
    fn get_semester(&self, year: u8, sem: u8) -> Option<&Semester> {
        self.semesters.get(&[year, sem])
    }
    fn add_module(&mut self, year: u8, sem: u8, module: Module) {
        let sem = self.semesters.get_mut(&[year, sem]).unwrap();
        sem.insert(module);
    }
}

pub fn main() {
    println!("--- PLAN::main() ---");
    println!("GOT HERE");
}
