/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let trimed_code = code.replace(" ", "");

    if trimed_code.len() <= 1 || trimed_code.chars().any(|c| !c.is_numeric()) {
        return false;
    }

    let sum = trimed_code
        .chars()
        .rev()
        .collect::<String>()
        .char_indices()
        .fold(0, |acc, x| {
            let num = x.1 as usize - '0' as usize;
            let acc = match x.0 % 2 {
                0 => acc + num,
                _ => {
                    if num * 2 > 9 {
                        acc + num * 2 - 9
                    } else {
                        acc + num * 2
                    }
                }
            };
            acc
        });
    return sum % 10 == 0;
}
