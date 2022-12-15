use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut hs = HashSet::new();
    let word_len = word.len();
    let f_ans = possible_anagrams
        .iter()
        .filter(|str| str.len() == word_len)
        .filter(|str| str.to_lowercase() != word.to_lowercase());
    let mut a: Vec<char> = word.to_lowercase().chars().collect();
    a.sort_unstable();
    for an in f_ans {
        let mut b: Vec<char> = an.to_lowercase().chars().collect();
        b.sort_unstable();
        if a == b {
            hs.insert(*an);
        }
    }
    hs
}
