use lazy_static::lazy_static;
use regex::Regex;

use crate::rockpaperscissors::{
    RockPaperScissors, RockPaperScissors::Rock, RockPaperScissors::Paper, RockPaperScissors::Scissors,
    GameOutcome, GameOutcome::Loss, GameOutcome::Draw, GameOutcome::Win,
};


lazy_static! {
    static ref PAIR: Regex = Regex::new(r"(?P<first>[ABC])\s(?P<second>[XYZ])").unwrap();
}

pub trait StrategyGuide {
    fn selection_pair(line: &str) -> (RockPaperScissors, RockPaperScissors);
}

pub struct WrongStrategyGuide {}

impl StrategyGuide for WrongStrategyGuide {
    fn selection_pair(line: &str) -> (RockPaperScissors, RockPaperScissors) {
        let capture = PAIR.captures(line).unwrap();
        let first = WrongStrategyGuide::decode(&capture["first"]);
        let second = WrongStrategyGuide::decode(&capture["second"]);
        // reverse order because 'we' are the second column
        (second, first)
    }
}

impl WrongStrategyGuide {
    fn decode(value: &str) -> RockPaperScissors {
        match value {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => panic!("Unexpected value '{}' - must be in (A, B, C, X, Y, Z)", value),
        }
    }
}

pub struct CorrectStrategyGuide {}

impl StrategyGuide for CorrectStrategyGuide {
    fn selection_pair(line: &str) -> (RockPaperScissors, RockPaperScissors) {
        let capture = PAIR.captures(line).unwrap();
        let first = CorrectStrategyGuide::decode_selection(&capture["first"]);
        let outcome = CorrectStrategyGuide::decode_outcome(&capture["second"]);

        let second = RockPaperScissors::rig(&first, &outcome);
        
        // reverse order because 'they' are the first column
        (second, first)
    }
}

impl CorrectStrategyGuide {
    fn decode_selection(value: &str) -> RockPaperScissors {
        match value {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => panic!("Unexpected value '{}' - must be in (A, B, C)", value),
        }
    }

    fn decode_outcome(value: &str) -> GameOutcome {
        match value {
            "X" => Loss,
            "Y" => Draw,
            "Z" => Win,
            _ => panic!("Unexpected value '{}' - must be in (X, Y, Z)", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrong() {
        assert_eq!((Paper, Rock), WrongStrategyGuide::selection_pair("A Y"));
        assert_eq!((Rock, Paper), WrongStrategyGuide::selection_pair("B X"));
        assert_eq!((Scissors, Scissors), WrongStrategyGuide::selection_pair("C Z"));
    }

    #[test]
    fn correct() {
        assert_eq!((Rock, Rock), CorrectStrategyGuide::selection_pair("A Y"));
        assert_eq!((Rock, Paper), CorrectStrategyGuide::selection_pair("B X"));
        assert_eq!((Rock, Scissors), CorrectStrategyGuide::selection_pair("C Z"));
    }
}