#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores{scores}
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        match self.scores.len() {
            0 => None,
            n => Some(self.scores[n-1]),
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorted_scores = self.scores.to_vec();
        sorted_scores.sort_by(|a, b| b.cmp(a));
        sorted_scores.into_iter().take(3).collect()
    }
}
