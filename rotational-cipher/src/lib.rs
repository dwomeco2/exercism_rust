pub fn rotate(input: &str, key: i8) -> String {
    let key: u8 = (key < 0)
        .then(|| ((26 + key) % 26) as u8)
        .unwrap_or(key as u8);
    input
        .chars()
        .map(|c| match c {
            c if c.is_ascii_lowercase() => (((c as u8 - b'a' + key) % 26) + b'a') as char,
            c if c.is_ascii_uppercase() => (((c as u8 - b'A' + key) % 26) + b'A') as char,
            _ => c,
        })
        .collect()
}
