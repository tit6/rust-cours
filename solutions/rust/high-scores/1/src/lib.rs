#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self {
            scores: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        return &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        if self.scores.is_empty() {
            return None
        } else {
             return Some(self.scores[self.scores.len() -1])   
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        if self.scores.is_empty() {
            return None
        } else {
            let max = Self::max_score(self.scores.clone());
            return Some(max)   
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top_three : Vec<u32> = Vec::new();
        let mut temp_max = 0;
        let mut copi_score = self.scores.clone();

        
        for _i in 1..4 {
            if copi_score.is_empty(){
                top_three.sort();
                top_three.reverse();
                return top_three
            }
            temp_max = Self::max_score(copi_score.clone());
            top_three.push(temp_max);
             if let Some(pos) = copi_score.iter().position(|&x| x == temp_max) {
                copi_score.remove(pos);
    }
            
        }

        top_three.sort();
        top_three.reverse();
        return top_three
        
    }

    pub fn max_score(score : Vec<u32>) -> u32 {
        let mut max = 0;
        for i in score.clone() {
            if max < i {
                max = i;
            }
        }
        return max
    }
}
