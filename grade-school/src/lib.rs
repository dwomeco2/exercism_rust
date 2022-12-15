use std::collections::{HashMap, HashSet};

// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
pub struct School {
    grades: HashSet<u32>,
    students: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            grades: HashSet::new(),
            students: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.grades.insert(grade);
        self.students
            .entry(grade)
            .or_insert(vec![student.into()])
            .push(student.into());
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut v: Vec<u32> = self.grades.iter().copied().collect();
        v.sort();
        v
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut v: Vec<String> = self
            .students
            .get(&grade)
            .or(Some(&Vec::new()))
            .unwrap()
            .to_owned();
        v.sort();
        v.dedup();
        v
    }
}
