use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

#[derive(Debug, Default)]
pub struct Forth {
    stack: Vec<Value>,
    word_dict: HashMap<String, String>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Default::default()
    }

    pub fn stack(&self) -> &[Value] {
        self.stack.as_slice()
    }

    pub fn eval(&mut self, input: &str) -> Result {
        if input.match_indices(|c: char| c == ':' || c == ';').count() % 2 != 0 {
            return Err(Error::InvalidWord);
        }

        let mut v: Vec<&str> = vec![];
        let s: Vec<&str> = input.trim().split(' ').collect();
        for x in s {
            match x {
                ":" => v.push(x),
                ";" => {
                    v.remove(0);
                    let (name, last) = v.split_at(1);
                    let name = name[0].to_lowercase();
                    if name.parse::<i32>().is_ok() {
                        return Err(Error::InvalidWord);
                    }
                    if let Some(old) = self.word_dict.insert(name.to_string(), last.join(" ")) {
                        self.replace(&name, &old);
                    }
                    v = vec![];
                }
                _ if v.first() == Some(&":") => v.push(x),
                _ => self.handle_ops(x)?,
            }
        }

        Ok(())
    }

    fn handle_ops(&mut self, op: &str) -> Result {
        let opp = &*op.to_lowercase();
        for haha in opp.split(' ') {
            if self.word_dict.contains_key(haha) {
                return self.handle_ops(&self.word_dict.get(haha).unwrap().to_owned());
            }
            match haha {
                e if e.parse::<Value>().is_ok() => self.stack.push(e.parse::<Value>().unwrap()),
                o @ ("+" | "-" | "*" | "/") => {
                    let n = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    let n1 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    match o {
                        "+" => self.stack.push(n + n1),
                        "-" => self.stack.push(n1 - n),
                        "*" => self.stack.push(n * n1),
                        "/" => {
                            let result = n1.checked_div(n).ok_or(Error::DivisionByZero)?;
                            self.stack.push(result);
                        }
                        _ => unreachable!(),
                    }
                }
                "dup" => {
                    let n = self.stack.last().ok_or(Error::StackUnderflow)?.clone();
                    self.stack.push(n);
                }
                "drop" => {
                    self.stack.pop().ok_or(Error::StackUnderflow)?;
                }
                "swap" => {
                    let n = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    let n1 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    self.stack.push(n);
                    self.stack.push(n1);
                }
                "over" => {
                    let n = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    let n1 = self.stack.last().ok_or(Error::StackUnderflow)?.clone();
                    self.stack.push(n);
                    self.stack.push(n1);
                }
                _ => Err(Error::UnknownWord)?,
            }
        }

        Ok(())
    }

    fn replace(&mut self, word: &str, existing: &str) {
        self.word_dict.clone().iter().for_each(|(name, def)| {
            if def.split_whitespace().any(|term| term == word) {
                let def = def
                    .split_whitespace()
                    .map(|term| match term.to_string() {
                        t if t == word => existing.to_owned(),

                        t => t,
                    })
                    .collect::<Vec<String>>()
                    .join(" ");

                self.word_dict.insert(name.clone(), def);
            }
        });
    }
}
