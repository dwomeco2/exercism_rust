/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn = isbn.replace('-', "");
    if isbn.len() != 10 {
        return false;
    }
    let (front, back) = isbn.split_at(9);
    if front.parse::<i32>().is_err() {
        return false;
    }
    if !(back.parse::<i32>().is_ok() || back == "X") {
        return false;
    }
    let mut a = front
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| c.to_digit(10).unwrap() * (i as u32 + 2))
        .sum::<u32>();
    a += back.parse::<u32>().or::<u32>(Ok(10)).unwrap();
    a % 11 == 0
}
