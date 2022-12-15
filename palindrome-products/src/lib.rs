/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    pub fn new(value: u64) -> Option<Palindrome> {
        let s = value.to_string();
        s.chars().eq(s.chars().rev()).then_some(Palindrome(value))
    }

    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if let Some(minp) = (min..=max)
        .take_while(|x| *x <= max)
        .flat_map(|n| (n..=max).filter_map(move |m| Palindrome::new(n * m)))
        .nth(0)
    {
        let maxp = (min..=max)
            .rev()
            .flat_map(|n| (min..=n).rev().map(move |m| n * m))
            .fold(minp, |p, x| {
                if x > p.into_inner() {
                    Palindrome::new(x).unwrap_or(p)
                } else {
                    p
                }
            });

        Some((minp, maxp))
    } else {
        None
    }
}
