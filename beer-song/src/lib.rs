fn num_w(n: u32) -> String {
    match n {
        0 => "99 bottles".to_string(),
        1 => "no more bottles".to_string(),
        2 => "1 bottle".to_string(),
        _ => format!("{n} bottles", n = n - 1),
    }
}

fn num(n: u32) -> String {
    match n {
        0 => "No more bottles".to_string(),
        1 => "1 bottle".to_string(),
        _ => format!("{n} bottles"),
    }
}

fn num_s(n: u32) -> String {
    match n {
        0 => "no more bottles".to_string(),
        _ => num(n),
    }
}
fn mid(n: u32) -> String {
    match n {
        0 => "Go to the store and buy some more".to_string(),
        1 => "Take it down and pass it around".to_string(),
        _ => format!("Take one down and pass it around"),
    }
}

pub fn verse(n: u32) -> String {
    format!(
        "{num} of beer on the wall, {num_s} of beer.\n{mid}, {num_w} of beer on the wall.\n",
        num = num(n),
        num_s = num_s(n),
        mid = mid(n),
        num_w = num_w(n)
    )
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .map(|n| verse(n))
        .rev()
        .collect::<Vec<_>>()
        .join("\n")
}
