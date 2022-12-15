use std::iter;

#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut vec: Vec<u8> = vec![];

    for &num in values.iter().rev() {
        let mut remain = num;
        let mut qlv_bit = iter::once(0).chain(iter::repeat(1)).peekable();

        while remain > 0 || qlv_bit.peek().unwrap() == &0 {
            vec.push(((remain % 128) | qlv_bit.next().unwrap() << 7) as u8);
            remain = remain / 128;
        }
    }
    vec.reverse();
    vec
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut res = Vec::new();
    let mut iter = bytes.iter().peekable();
    let mut number = 0u32;

    while let Some(byte) = iter.next() {
        number = if number.leading_zeros() >= 7 {
            (number << 7) | (byte & 0x7F) as u32
        } else {
            return Err(Error::Overflow);
        };

        if byte & 0x80 == 0 {
            res.push(number);
            number = 0;
            continue;
        } else if iter.peek().is_none() {
            return Err(Error::IncompleteNumber);
        }
    }

    Ok(res)
}
