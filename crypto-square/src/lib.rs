pub fn encrypt(input: &str) -> String {
    let input = Vec::from(normalize(input));
    let x = (input.len() as f32).sqrt();
    let (rows_len, chunk_length) = (x.ceil() as usize, x.floor() as usize);
    (0..rows_len)
        .map(|j| {
            (0..chunk_length)
                .filter_map(|i| {
                    (&input.get(i * rows_len + j)).map_or(Some(' '), |&c| Some(c as char))
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn normalize(input: &str) -> String {
    input
        .chars()
        .filter(|x| x.is_alphanumeric())
        .map(|x| x.to_ascii_lowercase())
        .collect::<String>()
}
