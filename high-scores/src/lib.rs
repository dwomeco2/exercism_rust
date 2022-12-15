#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &'a [u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut s = self.scores.to_owned();
        s.sort();
        match s.as_slice() {
            [.., last1, last2, last3] => vec![*last3, *last2, *last1],
            [.., last2, last3] => vec![*last3, *last2],
            [.., last3] => vec![*last3],
            [] => vec![],
        }
    }
}
