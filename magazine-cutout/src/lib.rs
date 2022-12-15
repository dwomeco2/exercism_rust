// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut words: HashMap<&str, u8> = HashMap::new();

    for word in note.iter() {
        *words.entry(word).or_insert(0) += 1;
    }

    for word in magazine.iter() {
        if !words.contains_key(word) {
            continue;
        }
        match words.get_mut(word).unwrap() {
            0 => (),

            w => *w -= 1,
        }
    }

    words.values().sum::<u8>() == 0
}
