/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let s = sentence.to_ascii_lowercase();
    ('a'..='z').all(|x| s.contains(x))
}
