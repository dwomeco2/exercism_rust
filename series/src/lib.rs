pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec!["".to_string(); 6];
    }
    Vec::from(digits)
        .windows(len)
        .map(|c| std::str::from_utf8(c).unwrap().to_owned())
        .collect()
}
