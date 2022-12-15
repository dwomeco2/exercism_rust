use std::{collections::HashSet, sync::Mutex};

use rand::Rng;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref ROBOT_NAMES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            name: Self::generate_name(),
        }
    }

    fn generate_name() -> String {
        let mut rng = rand::thread_rng();
        loop {
            let n: u32 = rng.gen_range(0..1000);
            let a1: char = rng.gen_range(b'A'..b'Z') as char;
            let a2: char = rng.gen_range(b'A'..b'Z') as char;
            let new_name = format!("{}{}{:03}", a1, a2, n);

            if ROBOT_NAMES.lock().unwrap().insert(new_name.clone()) {
                return new_name;
            }
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = Robot::generate_name();
    }
}
