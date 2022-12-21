use std::collections::HashSet;
use std::mem;
use types::Module;

/// In a graph representation of all possible module plans, each node represents
/// a path. This is that node
#[derive(Clone)]
pub struct Path {
    record: Vec<Vec<String>>,
    /// Modules done before the current semester.
    done: HashSet<String>,
    /// Semester number of the `doing` field
    global_sem: usize,
    /// Modules done in current semester.
    /// Always in increasing lexicographical order.
    doing: Vec<String>,
}

impl Path {
    pub fn new() -> Self {
        Self {
            done: HashSet::new(),
            global_sem: 0,
            doing: vec![],
            record: vec![],
        }
    }

    /// Increments the semester and moves all `doing` to `done`.
    pub fn next_sem(&mut self) {
        let doing = mem::take(&mut self.doing);
        self.done.extend(doing.clone());
        self.record.push(doing);
        self.global_sem += 1;
    }

    /// Mark a module as done during this semester.
    pub fn mark(&mut self, module_code: &str) {
        self.doing.push(module_code.to_string());
    }

    pub fn mod_count(&self) -> usize {
        self.record.iter().fold(0, |a, v| a + v.len()) + self.doing_count()
    }

    /// Checks if path has all the modules desired.
    pub fn is_done(&self, required: &Vec<String>) -> bool {
        required.iter().all(|r| self.done.contains(r) || self.doing.contains(r))
    }

    /// Checks if path has all the modules desired.
    /// TODO: This is a debug function
    pub fn is_doing(&self, code: &String) -> bool {
        self.doing.contains(code)
    }

    /// Gets the actual semester: a value in [1, 4]
    fn sem(&self) -> usize {
        self.global_sem % 4 + 1
    }

    /// Get number of modules currently doing.
    pub fn doing_count(&self) -> usize {
        self.doing.len()
    }

    /// Number of sems that the path takes
    pub fn len(&self) -> usize {
        self.global_sem
    }

    /// Get a list of possible next modules to take.
    /// 1. Must be offered in this sem.
    /// 2. Must have prerequisites fulfilled by modules `done`.
    /// 3. Must be lexicographically after the last module in `doing`.
    pub fn choices<'a>(
        &self,
        sample_space: &'a Vec<Module>,
    ) -> Vec<&'a String> {
        let last = self.doing.last();
        sample_space
            .iter()
            // 1. Must be offered in this sem.
            .filter(|m| m.semesters().contains(&self.sem()))
            // 2. Must have prerequisites fulfilled by modules `done`.
            .filter(|m| m.prereqtree().satisfied_by(&self.done))
            // 3. Must be lexicographically after the last module in `doing`.
            .filter(|m| last.map_or(true, |v| m.code().cmp(v).is_gt()))
            .map(|m| m.code())
            .collect()
    }
}

// for BinaryHeap implementation (min heap)
use std::cmp::{Ord, Ordering, PartialOrd};
impl PartialEq for Path {
    fn eq(&self, rhs: &Path) -> bool {
        self.len() == rhs.len()
    }
}
impl Eq for Path {}
impl PartialOrd for Path {
    fn partial_cmp(&self, rhs: &Path) -> Option<Ordering> {
        match rhs.len().cmp(&self.len()) {
            Ordering::Equal => None,
            v => Some(v),
        }
    }
}
impl Ord for Path {
    fn cmp(&self, rhs: &Path) -> Ordering {
        rhs.len().cmp(&self.len())
    }
}

use std::fmt;
impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Path")
            .field("record", &self.record)
            .field("global_sem", &self.global_sem)
            .finish()
    }
}

#[test]
fn test() {
    let mut p = Path::new();
    p.mark("CS1010");
    p.next_sem();
    p.next_sem();
    println!("{p:?}");
    println!("{:?}", "a".cmp("A"));
    // assert!(false)
}
