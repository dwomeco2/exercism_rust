use rand::Rng;

pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() {
        return None;
    }
    s.chars()
        .zip(key.chars().cycle())
        .map(|(c, k)| match c {
            _ if k.is_numeric() => None,
            _ if k.is_uppercase() => None,
            _ => Some((((c as u8 + (k as u8 - b'a') - b'a') % 26) + b'a') as char),
        })
        .collect::<Option<String>>()
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() {
        return None;
    }
    s.chars()
        .zip(key.chars().cycle())
        .map(|(c, k)| match c {
            _ if k.is_numeric() => None,
            _ if k.is_uppercase() => None,
            _ => Some((((c as u8 + (26 - (k as u8 - b'a')) - b'a') % 26) + b'a') as char),
        })
        .collect::<Option<String>>()
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = rand::thread_rng();
    let key = (0..100)
        .map(|_| rng.gen_range('a'..='z'))
        .collect::<String>();

    let encoded = encode(&key, s).unwrap();
    (key, encoded)
}
