use std::ops::Deref;

pub struct WordProblem;

pub fn answer(command: &str) -> Option<i32> {
    let mut v = command
        .strip_suffix("?")
        .unwrap()
        .split_ascii_whitespace()
        .filter_map(|s| match s {
            n if n.parse::<i32>().is_ok() => Some(s.to_string()),
            n if n.ends_with("th") && n.strip_suffix("th").unwrap().parse::<i32>().is_ok() => {
                Some(s.strip_suffix("th").unwrap().to_string())
            }
            "plus" | "minus" | "multiplied" | "divided" | "cubed" | "raised" => Some(s.to_string()),
            _ => None,
        })
        .rev()
        .collect::<Vec<String>>();
    let mut result: Option<i32> = None;
    while v.len() > 0 && (v.len() - 1) % 2 == 0 {
        result = match [v.pop(), v.pop(), v.pop()] {
            [Some(n), None, None] if n.parse::<i32>().is_ok() => Some(t32(n)),
            [Some(n1), Some(op), Some(n2)]
                if n1.parse::<i32>().is_ok() && n2.parse::<i32>().is_ok() =>
            {
                match op.deref() {
                    "plus" => Some(t32(n1) + t32(n2)),
                    "minus" => Some(t32(n1) - t32(n2)),
                    "multiplied" => Some(t32(n1) * t32(n2)),
                    "divided" => t32(n1).checked_div(t32(n2)),
                    "raised" => Some(t32(n1).pow(t32(n2) as u32)),
                    _ => break,
                }
                .and_then(|x| {
                    v.push(x.clone().to_string());
                    Some(x)
                })
            }
            _ => break,
        };
    }

    result
}

fn t32(s: String) -> i32 {
    s.parse::<i32>().unwrap()
}
