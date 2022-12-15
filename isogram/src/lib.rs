use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let s = candidate
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<String>();
    let hs: HashSet<char> = s.to_ascii_lowercase().chars().collect();
    hs.len() == s.len()
}
