use crate::rockpaperscissors::RockPaperScissors;

pub struct Tournament {
    score: u32,
}

impl Tournament {
    pub fn new() -> Tournament {
        Tournament { score: 0 }
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn game(&mut self, a: RockPaperScissors, b: RockPaperScissors) {
        let outcome = RockPaperScissors::play(&a, &b);

        self.score += a.score() + outcome.score();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let empty = Tournament::new();

        assert_eq!(0, empty.score());
    }

    #[test]
    fn small() {
        let mut small = Tournament::new();
        small.game(RockPaperScissors::Paper, RockPaperScissors::Rock);
        small.game(RockPaperScissors::Rock, RockPaperScissors::Paper);
        small.game(RockPaperScissors::Scissors, RockPaperScissors::Scissors);

        assert_eq!(15, small.score());
    }
}