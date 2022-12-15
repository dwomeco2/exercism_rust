#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    if let Some(&n) = number.iter().find(|&&x| x >= from_base) {
        return Err(Error::InvalidDigit(n));
    }

    let number = number
        .iter()
        .skip_while(|&&x| x == 0)
        .copied()
        .collect::<Vec<_>>();

    if number.len() == 0 {
        return Ok(vec![0]);
    }

    let mut base10 = number
        .iter()
        .rev()
        .enumerate()
        .map(|(d, &n)| n * from_base.pow(d as u32))
        .sum::<u32>();

    let mut out = vec![];
    loop {
        out.push(base10 % to_base);
        base10 /= to_base;
        if base10 == 0 {
            break;
        }
    }
    out.reverse();
    Ok(out)
}
