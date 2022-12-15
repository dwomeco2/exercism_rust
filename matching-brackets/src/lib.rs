fn is_match(open: char, close: char) -> bool {
    match open {
        '[' => close == ']',
        '{' => close == '}',
        '(' => close == ')',
        _ => false,
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = vec![];
    string.chars().all(|x| {
        match x {
            '[' | '{' | '(' => stack.push(x),
            ']' | '}' | ')' => {
                if let Some(open) = stack.pop() {
                    return is_match(open, x);
                } else {
                    return false;
                }
            }
            _ => return true,
        }
        return true;
    }) && stack.len() == 0
}
