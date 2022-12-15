/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    } else if s1 == s2 {
        return Some(0);
    }
    return Some(
        s1.chars()
            .zip(s2.chars())
            .fold(0, |acc, (c1, c2)| match c1 != c2 {
                true => acc + 1,
                false => acc,
            }),
    );
}
