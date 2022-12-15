pub fn number(user_number: &str) -> Option<String> {
    let num = user_number
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>();
    match num.len() {
        7 if is_valid_7(&num) => Some(num),
        10 if is_valid_10(&num) => Some(num),
        11 if is_valid_11(&num) => Some((&num[1..]).to_string()),
        _ => None,
    }
}

fn is_valid_7(num: &str) -> bool {
    num.starts_with(valid_digit_n)
}

fn is_valid_10(num: &str) -> bool {
    num.starts_with(valid_digit_n) && num.chars().nth(3).map_or(false, valid_digit_n)
}

fn valid_digit_n(c: char) -> bool {
    ('2'..='9').contains(&c)
}

fn is_valid_11(num: &str) -> bool {
    num.starts_with('1')
        && num.chars().nth(1).map_or(false, valid_digit_n)
        && num.chars().nth(4).map_or(false, valid_digit_n)
}
