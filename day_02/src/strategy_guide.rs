use lazy_static::lazy_static;
use regex::Regex;

use crate::rockpaperscissors::{RockPaperScissors, GameOutcome};

pub struct WrongStrategyGuide {}

lazy_static! {
    static ref PAIR: Regex = Regex::new(r"(?P<first>[ABC])\s(?P<second>[XYZ])").unwrap();
}

impl WrongStrategyGuide {
    pub fn selection_pair(line: &str) -> (RockPaperScissors, RockPaperScissors) {
        let capture = PAIR.captures(line).unwrap();
        let first = WrongStrategyGuide::decode(&capture["first"]);
        let second = WrongStrategyGuide::decode(&capture["second"]);
        // reverse order because 'we' are the second column
        (second, first)
    }

    fn decode(value: &str) -> RockPaperScissors {
        match value {
            "A" | "X" => RockPaperScissors::Rock,
            "B" | "Y" => RockPaperScissors::Paper,
            "C" | "Z" => RockPaperScissors::Scissors,
            _ => panic!("Unexpected value '{}' - must be in (A, B, C, X, Y, Z)", value),
        }
    }
}

pub struct CorrectStrategyGuide {}

impl CorrectStrategyGuide {
    pub fn selection_pair(line: &str) -> (RockPaperScissors, RockPaperScissors) {
        let capture = PAIR.captures(line).unwrap();
        let first = CorrectStrategyGuide::decode_selection(&capture["first"]);
        let outcome = CorrectStrategyGuide::decode_outcome(&capture["second"]);

        let second = RockPaperScissors::rig(&first, &outcome);
        
        // reverse order because 'they' are the first column
        (second, first)
    }

    fn decode_selection(value: &str) -> RockPaperScissors {
        match value {
            "A" => RockPaperScissors::Rock,
            "B" => RockPaperScissors::Paper,
            "C" => RockPaperScissors::Scissors,
            _ => panic!("Unexpected value '{}' - must be in (A, B, C)", value),
        }
    }

    fn decode_outcome(value: &str) -> GameOutcome {
        match value {
            "X" => GameOutcome::Loss,
            "Y" => GameOutcome::Draw,
            "Z" => GameOutcome::Win,
            _ => panic!("Unexpected value '{}' - must be in (X, Y, Z)", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrong() {
        assert_eq!((RockPaperScissors::Paper, RockPaperScissors::Rock), WrongStrategyGuide::selection_pair("A Y"));
        assert_eq!((RockPaperScissors::Rock, RockPaperScissors::Paper), WrongStrategyGuide::selection_pair("B X"));
        assert_eq!((RockPaperScissors::Scissors, RockPaperScissors::Scissors), WrongStrategyGuide::selection_pair("C Z"));
    }

    #[test]
    fn correct() {
        assert_eq!((RockPaperScissors::Rock, RockPaperScissors::Rock), CorrectStrategyGuide::selection_pair("A Y"));
        assert_eq!((RockPaperScissors::Rock, RockPaperScissors::Paper), CorrectStrategyGuide::selection_pair("B X"));
        assert_eq!((RockPaperScissors::Rock, RockPaperScissors::Scissors), CorrectStrategyGuide::selection_pair("C Z"));
    }
}