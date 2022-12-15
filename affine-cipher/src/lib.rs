/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

fn coprime(a: i32, b: i32) -> bool {
    match (a, b) {
        (0, 1) | (1, 0) => true,
        (0, _) | (_, 0) => false,
        _ if a == b => true,
        _ if a > b => coprime(a % b, b),
        _ if a < b => coprime(a, b % a),
        _ => unreachable!(),
    }
}

fn inverse(a: i32, b: i32) -> i32 {
    let (mut t, mut newt) = (0, 1);
    let (mut r, mut newr) = (b, a);

    while newr != 0 {
        let quot = r / newr;
        (t, newt) = (newt, t - quot * newt);
        (r, newr) = (newr, r - quot * newr);
    }
    if r > 1 {
        panic!()
    }
    (t % b + b) % b
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !coprime(a, 26) {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let mut v = vec![];
    for c in plaintext.chars() {
        let x = match c as u8 {
            b'0'..=b'9' => c,
            b'a'..=b'z' | b'A'..=b'Z' => {
                let x = c.to_ascii_lowercase() as u8 - 97;
                let ec = ((a * x as i32 + b) % 26) as u8 + 97;
                ec as char
            }
            _ => continue,
        };
        v.push(x);
    }
    (0..v.len() / 5).for_each(|i| v.insert((i + 1) * 5 + i, ' '));
    Ok(v.iter().collect::<String>().trim_end().to_string())
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !coprime(a, 26) {
        return Err(AffineCipherError::NotCoprime(a));
    }
    let mut v = vec![];
    let inv = inverse(a, 26);
    for c in ciphertext.chars() {
        let x = match c as u8 {
            b'0'..=b'9' => c,
            b'a'..=b'z' => {
                let y = c as u8;
                let de = ((inv * (y as i32 - 97 - b)).rem_euclid(26)) as u8 + 97;
                de as char
            }
            _ => continue,
        };
        v.push(x);
    }
    Ok(v.iter().collect())
}
