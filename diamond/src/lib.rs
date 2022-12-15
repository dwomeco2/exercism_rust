pub fn get_diamond(c: char) -> Vec<String> {
    let size = (c as u8 - 65 + 1) * 2 - 1;
    let mid = size / 2;
    (0..)
        .take(size as usize)
        .map(|i| match i {
            _ if i <= mid => i,
            _ => mid - (i - mid),
        })
        .map(|line| format!("{inner:^size$}", inner = inner(line), size = size as usize))
        .collect()
}

fn inner(l: u8) -> String {
    match l {
        0 => "A".to_string(),
        _ => format!(
            "{ch}{sp:>len$}{ch}",
            ch = (l + 65) as char,
            sp = ' ',
            len = l as usize * 2 - 1
        ),
    }
}
