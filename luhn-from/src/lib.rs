pub struct Luhn {
    code: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let code = &self.code;
        let trimed_code = code.replace(' ', "");

        if trimed_code.len() <= 1 || trimed_code.chars().any(|c| !c.is_numeric()) {
            return false;
        }

        let sum = trimed_code
            .chars()
            .rev()
            .collect::<String>()
            .char_indices()
            .fold(0, |acc, x| {
                let num = x.1 as usize - '0' as usize;

                match x.0 % 2 {
                    0 => acc + num,
                    _ => {
                        if num * 2 > 9 {
                            acc + num * 2 - 9
                        } else {
                            acc + num * 2
                        }
                    }
                }
            });
        sum % 10 == 0
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T> From<T> for Luhn
where
    T: ToString,
{
    fn from(input: T) -> Self {
        Luhn {
            code: input.to_string(),
        }
    }
}
