use std::fmt::{Display, Formatter, Result};

pub struct Roman(u32);

const ROMAN_LETTER: &[&[&str]] = &[&["I", "V", "X"], &["X", "L", "C"], &["C", "D", "M"]];

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let n = self.0;

        let (_, mut s) = ROMAN_LETTER.iter().fold((0, String::new()), |(d, s), l| {
            let digit = (n % 10u32.pow(d + 1)) / 10u32.pow(d);
            (d + 1, format!("{}{s}", &p(digit, l[0], l[1], l[2])))
        });
        s.insert_str(0, "M".repeat((n as usize % 10000) / 1000).as_str());
        write!(f, "{s}")
    }
}

fn p(n: u32, s1: &str, s2: &str, s3: &str) -> String {
    match n {
        n if n == 1 => s1.to_string(),
        n if n < 4 => s1.repeat(n as usize),
        n if n == 4 => format!("{s1}{s2}"),
        n if n < 9 => format!("{s2}{}", s1.repeat(n as usize - 5)),
        n if n == 9 => format!("{s1}{s3}"),
        _ => unreachable!(),
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Roman(num)
    }
}
