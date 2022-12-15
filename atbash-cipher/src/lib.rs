/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let mut v = vec![];
    for c in plain.chars() {
        let x = match c as u8 {
            b'0'..=b'9' => c,
            b'a'..=b'z' | b'A'..=b'Z' => {
                let c = c.to_ascii_lowercase() as u8;
                let c = (26 - (c - 97) - 1) + 97;
                c as char
            }
            _ => continue,
        };
        v.push(x);
    }
    v.chunks(5)
        .into_iter()
        .map(|x| x.iter().collect::<String>() + " ")
        .collect::<String>()
        .trim_end()
        .to_string()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let mut v = vec![];
    for c in cipher.chars() {
        let x = match c as u8 {
            b'0'..=b'9' => c,
            b'a'..=b'z' => {
                let c = c as u8;
                let c = 26 - (c - 97 + 1) + 97;
                c as char
            }
            _ => continue,
        };
        v.push(x);
    }
    v.iter().collect()
}
