#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1);
    }
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }
    if let Some(c) = string_digits.chars().find(|&c| !c.is_numeric()) {
        return Err(Error::InvalidDigit(c));
    }
    Ok(sub(string_digits, span))
}

pub fn sub(string_digits: &str, span: usize) -> u64 {
    if let Some((s1, s2)) = string_digits.split_once('0') {
        let r1 = sub(s1, span);
        let r2 = sub(s2, span);
        return r1.max(r2);
    }

    let v = string_digits.chars().collect::<Vec<_>>();
    v.windows(span)
        .map(|x| x.iter().map(|&x| (x as u8 - b'0') as u64).product::<u64>())
        .max()
        .unwrap_or(0)
}
