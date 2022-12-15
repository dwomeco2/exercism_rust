use std::{cmp::Ordering, fmt::Display};

use itertools::Itertools;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
enum Ranking {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    FiveOfAKind,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone)]
pub enum Value {
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    V10,
    VJ,
    VQ,
    VK,
    VA,
}

impl From<&str> for Value {
    fn from(a: &str) -> Self {
        match a {
            "2" => Value::V2,
            "3" => Value::V3,
            "4" => Value::V4,
            "5" => Value::V5,
            "6" => Value::V6,
            "7" => Value::V7,
            "8" => Value::V8,
            "9" => Value::V9,
            "10" => Value::V10,
            "J" => Value::VJ,
            "Q" => Value::VQ,
            "K" => Value::VK,
            "A" => Value::VA,
            _ => panic!("No Such Value"),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = match self {
            Value::V2 => "2",
            Value::V3 => "3",
            Value::V4 => "4",
            Value::V5 => "5",
            Value::V6 => "6",
            Value::V7 => "7",
            Value::V8 => "8",
            Value::V9 => "9",
            Value::V10 => "10",
            Value::VJ => "J",
            Value::VQ => "Q",
            Value::VK => "K",
            Value::VA => "A",
        };
        write!(f, "{}", x)
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone)]
enum Suit {
    Diamonds,
    Clubs,
    Hearts,
    Spades,
}

impl From<&str> for Suit {
    fn from(s: &str) -> Self {
        match s {
            "D" => Suit::Diamonds,
            "C" => Suit::Clubs,
            "H" => Suit::Hearts,
            "S" => Suit::Spades,
            _ => panic!("No Such Suit"),
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Suit::Diamonds => write!(f, "{}", "D"),
            Suit::Clubs => write!(f, "{}", "C"),
            Suit::Hearts => write!(f, "{}", "H"),
            Suit::Spades => write!(f, "{}", "S"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, Eq)]
struct Card(Value, Suit);

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

#[derive(Eq, PartialEq, PartialOrd, Debug)]
pub struct Hand {
    cards: [Card; 5],
    category: Ranking,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let order = self.category.cmp(&other.category);
        if order.is_ne() {
            order
        } else {
            let v = self.sorted_cards();
            let v2 = other.sorted_cards();
            match self.category {
                Ranking::FiveOfAKind => self.cards[0].cmp(&other.cards[0]),
                Ranking::FourOfAKind | Ranking::FullHouse => {
                    let order = v[0].cmp(&v2[0]);
                    if order.is_eq() {
                        v[1].cmp(&v2[1])
                    } else {
                        order
                    }
                }
                Ranking::ThreeOfAKind => {
                    let order = v[0].cmp(&v2[0]);
                    if order.is_eq() {
                        self.compare_slice(&v[1..], &v2[1..])
                    } else {
                        order
                    }
                }
                Ranking::TwoPair => {
                    let order = v[0].cmp(&v2[0]);
                    if order.is_eq() {
                        let order = v[1].cmp(&v2[1]);
                        if order.is_eq() {
                            return v[2].cmp(&v2[2]);
                        }
                        return order;
                    }
                    order
                }
                Ranking::OnePair => {
                    let order = v[0].cmp(&v2[0]);
                    if order.is_eq() {
                        self.compare_slice(&v[1..], &v2[1..])
                    } else {
                        order
                    }
                }
                Ranking::StraightFlush | Ranking::Straight => {
                    let mut new_v = &v[..];
                    let mut new_v2 = &v2[..];
                    if let Some(_) = self.cards.iter().find(|x| x.0 == Value::VA) {
                        if let Some(_) = self.cards.iter().find(|x| x.0 == Value::V5) {
                            new_v = &v[1..];
                        }
                    }

                    if let Some(_) = other.cards.iter().find(|x| x.0 == Value::VA) {
                        if let Some(_) = other.cards.iter().find(|x| x.0 == Value::V5) {
                            new_v2 = &v2[1..];
                        }
                    }
                    self.compare_slice(&new_v, &new_v2)
                }
                Ranking::Flush | Ranking::HighCard => self.compare_slice(&v, &v2),
            }
        }
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cards.iter().join(" "))
    }
}

impl Hand {
    pub fn new(hand: &str) -> Hand {
        let v: [Card; 5] = hand
            .split_whitespace()
            .map(|cstr| {
                let l = cstr.len();
                Card(cstr[0..l - 1].into(), cstr[l - 1..l].into())
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let mut h = Hand {
            cards: v,
            category: Ranking::HighCard,
        };
        h.category = h.decide_category();
        h
    }

    fn decide_category(&self) -> Ranking {
        let v = self.sorted_cards();
        if v.len() == 1 {
            return Ranking::FiveOfAKind;
        } else if v.len() == 2 {
            if v[0].0 == 4 {
                return Ranking::FourOfAKind;
            } else {
                return Ranking::FullHouse;
            }
        } else if v.len() == 3 {
            if v[0].0 == 3 {
                return Ranking::ThreeOfAKind;
            } else {
                return Ranking::TwoPair;
            }
        } else if v.len() == 4 {
            return Ranking::OnePair;
        } else {
            // staright flush, flush, straight, highCard
            let is_flush = self.is_flush();
            let is_straight = self.is_straight();

            if is_flush && is_straight {
                return Ranking::StraightFlush;
            } else if is_flush {
                return Ranking::Flush;
            } else if is_straight {
                return Ranking::Straight;
            }
            return Ranking::HighCard;
        }
    }

    fn is_flush(&self) -> bool {
        // all same suit
        self.cards.iter().all(|&x| x.1 == self.cards[0].1)
    }

    fn is_straight(&self) -> bool {
        // snake
        let mut x = self
            .cards
            .iter()
            .sorted_by(|x1, x2| Ord::cmp(&x1.0, &x2.0))
            .collect::<Vec<_>>();
        // A 5 4 3 2    = is straight
        if x[4].0 == Value::VA && x[3].0 == Value::V5 {
            x.pop();
            return x.windows(2).all(|x| x[1].0 as u8 - x[0].0 as u8 == 1);
        }
        // A K Q J 10   = not straight
        x.windows(2).all(|x| x[1].0 as u8 - x[0].0 as u8 == 1) && x[0].0 != Value::VA
    }

    fn compare_slice(&self, this: &[(usize, Card)], other: &[(usize, Card)]) -> std::cmp::Ordering {
        let map_card = |x: &[(usize, Card)]| x.iter().map(|(_, c)| *c).collect::<Vec<_>>();
        let this = map_card(this);
        let other = map_card(other);
        this.cmp(&other)
    }

    fn sorted_cards(&self) -> Vec<(usize, Card)> {
        self.cards
            .into_iter()
            .sorted_by(|x1, x2| Ord::cmp(&x2.0, &x1.0))
            .dedup_by_with_count(|x1, x2| x1.0 == x2.0)
            .sorted_by(|x1, x2| Ord::cmp(&x2.0, &x1.0))
            .collect::<Vec<_>>()
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let result = hands
        .iter()
        .map(|h| Hand::new(&h))
        .sorted_by(|a, b| b.cmp(&a))
        .collect::<Vec<_>>();

    let mut v: Vec<&Hand> = vec![];
    for r in result.iter() {
        if v.len() == 0 {
            v.push(&r);
        } else if v[0].cmp(&r) == Ordering::Equal {
            v.push(&r);
        }
    }

    let gg = v.iter().map(|x| format!("{}", x)).collect::<Vec<_>>();

    let mut ggg = vec![];
    for h in hands {
        if gg.contains(&h.to_string()) {
            ggg.push(*h);
        }
    }
    ggg
}
