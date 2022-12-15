use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut hs = HashMap::new();
    for w in words
        .replace(|c| char::is_ascii_punctuation(&c) && c != '\'', " ")
        .split(|c: char| c.is_ascii_whitespace() || c == '\n')
        .map(|s| {
            s.trim_matches(|c| char::is_whitespace(c) || c == '\'')
                .to_ascii_lowercase()
        })
        .filter(|s| !s.is_empty())
    {
        *hs.entry(w).or_insert(0u32) += 1u32;
    }

    hs
}
