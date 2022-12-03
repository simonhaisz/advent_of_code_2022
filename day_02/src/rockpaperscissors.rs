#[derive(Debug, PartialEq)]
pub enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

impl RockPaperScissors {
    pub fn score(&self) -> u32 {
        match self {
            RockPaperScissors::Rock => 1,
            RockPaperScissors::Paper => 2,
            RockPaperScissors::Scissors => 3,
        }
    }

    pub fn play(a: &RockPaperScissors, b: &RockPaperScissors) -> GameOutcome {
        match a {
            RockPaperScissors::Rock => match b {
                RockPaperScissors::Rock => GameOutcome::Draw,
                RockPaperScissors::Paper => GameOutcome::Loss,
                RockPaperScissors::Scissors => GameOutcome::Win,
            },
            RockPaperScissors::Paper => match b {
                RockPaperScissors::Rock => GameOutcome::Win,
                RockPaperScissors::Paper => GameOutcome::Draw,
                RockPaperScissors::Scissors => GameOutcome::Loss,
            },
            RockPaperScissors::Scissors => match b {
                RockPaperScissors::Rock => GameOutcome::Loss,
                RockPaperScissors::Paper => GameOutcome::Win,
                RockPaperScissors::Scissors => GameOutcome::Draw,
            },
        }
    }

    pub fn rig(opponent: &RockPaperScissors, outcome: &GameOutcome) -> RockPaperScissors {
        match opponent {
            RockPaperScissors::Rock => match outcome {
                GameOutcome::Loss => RockPaperScissors::Scissors,
                GameOutcome::Draw => RockPaperScissors::Rock,
                GameOutcome::Win => RockPaperScissors::Paper,
            },
            RockPaperScissors::Paper => match outcome {
                GameOutcome::Loss => RockPaperScissors::Rock,
                GameOutcome::Draw => RockPaperScissors::Paper,
                GameOutcome::Win => RockPaperScissors::Scissors,
            },
            RockPaperScissors::Scissors => match outcome {
                GameOutcome::Loss => RockPaperScissors::Paper,
                GameOutcome::Draw => RockPaperScissors::Scissors,
                GameOutcome::Win => RockPaperScissors::Rock,
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum GameOutcome {
    Loss,
    Draw,
    Win,
}

impl GameOutcome {
    pub fn score(&self) -> u32 {
        match self {
            GameOutcome::Loss => 0,
            GameOutcome::Draw => 3,
            GameOutcome::Win => 6,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shape_scores() {
        assert_eq!(1, RockPaperScissors::Rock.score());
        assert_eq!(2, RockPaperScissors::Paper.score());
        assert_eq!(3, RockPaperScissors::Scissors.score());
    }

    #[test]
    fn outcome_scores() {
        assert_eq!(0, GameOutcome::Loss.score());
        assert_eq!(3, GameOutcome::Draw.score());
        assert_eq!(6, GameOutcome::Win.score());
    }

    #[test]
    fn play_games() {
        assert_eq!(GameOutcome::Draw, RockPaperScissors::play(&RockPaperScissors::Rock, &RockPaperScissors::Rock));
        assert_eq!(GameOutcome::Loss, RockPaperScissors::play(&RockPaperScissors::Rock, &RockPaperScissors::Paper));
        assert_eq!(GameOutcome::Win, RockPaperScissors::play(&RockPaperScissors::Rock, &RockPaperScissors::Scissors));

        assert_eq!(GameOutcome::Win, RockPaperScissors::play(&RockPaperScissors::Paper, &RockPaperScissors::Rock));
        assert_eq!(GameOutcome::Draw, RockPaperScissors::play(&RockPaperScissors::Paper, &RockPaperScissors::Paper));
        assert_eq!(GameOutcome::Loss, RockPaperScissors::play(&RockPaperScissors::Paper, &RockPaperScissors::Scissors));

        assert_eq!(GameOutcome::Loss, RockPaperScissors::play(&RockPaperScissors::Scissors, &RockPaperScissors::Rock));
        assert_eq!(GameOutcome::Win, RockPaperScissors::play(&RockPaperScissors::Scissors, &RockPaperScissors::Paper));
        assert_eq!(GameOutcome::Draw, RockPaperScissors::play(&RockPaperScissors::Scissors, &RockPaperScissors::Scissors));
    }

    #[test]
    fn rig_games() {
        assert_eq!(RockPaperScissors::Rock, RockPaperScissors::rig(&RockPaperScissors::Rock, &GameOutcome::Draw));
        assert_eq!(RockPaperScissors::Rock, RockPaperScissors::rig(&RockPaperScissors::Paper, &GameOutcome::Loss));
        assert_eq!(RockPaperScissors::Rock, RockPaperScissors::rig(&RockPaperScissors::Scissors, &GameOutcome::Win));
    }
}