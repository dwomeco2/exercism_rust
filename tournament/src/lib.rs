use std::{cmp::Ordering, collections::HashMap, fmt::Display};

enum MatchStatus {
    W,
    D,
    L,
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct Team {
    name: String,
    w: u8,
    d: u8,
    l: u8,
}

impl Team {
    pub fn new(name: &str) -> Self {
        Team {
            name: name.into(),
            w: 0,
            d: 0,
            l: 0,
        }
    }

    pub fn get_p(&self) -> u8 {
        self.w * 3 + self.d
    }

    pub fn get_mp(&self) -> u8 {
        self.w + self.d + self.l
    }
    pub fn update(&mut self, status: MatchStatus) {
        match status {
            MatchStatus::W => self.w += 1,
            MatchStatus::D => self.d += 1,
            MatchStatus::L => self.l += 1,
        }
    }
}

impl Ord for Team {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match other.get_p().cmp(&self.get_p()) {
            Ordering::Equal => self.name.cmp(&other.name),
            n => n,
        }
    }
}

impl Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:31}|{:3} |{:3} |{:3} |{:3} |{:3}",
            self.name,
            self.get_mp(),
            self.w,
            self.d,
            self.l,
            self.get_p()
        )
    }
}

pub fn tally(match_results: &str) -> String {
    let mut hm = HashMap::new();
    for x in match_results.split('\n').into_iter() {
        let x = x.split(';').collect::<Vec<&str>>();
        if let Some(&[team1, team2, status]) = x.get(0..=2) {
            let ha = match status {
                "win" => [(team1, MatchStatus::W), (team2, MatchStatus::L)],
                "loss" => [(team1, MatchStatus::L), (team2, MatchStatus::W)],
                "draw" => [(team1, MatchStatus::D), (team2, MatchStatus::D)],
                _ => unreachable!(),
            };
            for h in ha {
                let t = hm.entry(h.0).or_insert(Team::new(h.0));
                t.update(h.1);
            }
        }
    }

    let mut v = hm.iter().collect::<Vec<_>>();
    v.sort_by(|a, b| a.1.cmp(b.1));
    [
        "Team                           | MP |  W |  D |  L |  P",
        v.iter()
            .map(|t| t.1.to_string())
            .collect::<Vec<_>>()
            .join("\n")
            .as_str(),
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join("\n")
}
