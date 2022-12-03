use crate::rockpaperscissors::RockPaperScissors;

pub struct Tournament {}

impl Tournament {
    pub fn score_game(a: &RockPaperScissors, b: &RockPaperScissors) -> u32 {
        let outcome = RockPaperScissors::play(a, b);
        a.score() + outcome.score()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rockpaperscissors::{RockPaperScissors::Rock, RockPaperScissors::Paper, RockPaperScissors::Scissors};

    #[test]
    fn small() {

        let mut total = 0;
        total += Tournament::score_game(&Paper, &Rock);
        total += Tournament::score_game(&Rock, &Paper);
        total += Tournament::score_game(&Scissors, &Scissors);
        assert_eq!(15, total);
    }
}