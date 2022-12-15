use std::collections::HashMap;

use std::collections::HashSet;

use itertools::Itertools;

struct Token {
    chars: Vec<char>,
}

impl Token {
    fn new(string: &str) -> Self {
        let mut chars: Vec<char> = Vec::new();

        for c in string.chars() {
            chars.push(c)
        }

        Token { chars }
    }

    fn to_number(&self, mapping: &HashMap<char, u8>) -> u64 {
        let result: u64 = self
            .chars
            .iter()
            .rev()
            .enumerate()
            .map(|(i, c)| {
                let v = mapping.get(c).unwrap();

                10_u64.pow(i.try_into().unwrap()) * (*v as u64)
            })
            .sum();

        result
    }
}

fn parse_tokens(input: &str) -> (Vec<Token>, Token) {
    let equal_index = input.split_whitespace();

    let mut to_sum_tokens: Vec<Token> = Vec::new();

    for token in equal_index {
        if "+" == token || token.contains('=') {
            continue;
        }

        to_sum_tokens.push(Token::new(token));
    }

    let sum_token: Token = to_sum_tokens.pop().unwrap();

    return (to_sum_tokens, sum_token);
}

fn collect_first_chars(sum_tokens: &Vec<Token>, sum: &Token) -> HashSet<char> {
    let mut first_chars: HashSet<char> = HashSet::new();

    for t in sum_tokens {
        let c = t.chars.get(0).unwrap();

        first_chars.insert(*c);
    }

    first_chars.insert(*sum.chars.get(0).unwrap());

    first_chars
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // Let'ss start with a brute force solution:

    //      Get set of symbols

    //      Iterate over all combinations

    let (sum_tokens, sum) = parse_tokens(input);

    let chars: Vec<char> = input
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<_>>();

    let char_set: HashSet<char> = HashSet::from_iter(chars);

    let first_chars = collect_first_chars(&sum_tokens, &sum);

    let n_chars = char_set.iter().count();

    let perms = (0..10).permutations(n_chars);

    let mut mapping: HashMap<char, u8>;

    for permutation in perms {
        mapping = HashMap::new();

        for (i, char) in char_set.iter().enumerate() {
            mapping.insert(char.clone(), *permutation.get(i).unwrap());
        }

        if first_chars.iter().any(|c| *mapping.get(c).unwrap() == 0) {
            continue;
        }

        let hyp_result: u64 = sum_tokens.iter().map(|t| t.to_number(&mapping)).sum();

        if hyp_result == sum.to_number(&mapping) {
            return Some(mapping);
        };
    }

    return None;
}
