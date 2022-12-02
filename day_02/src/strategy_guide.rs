use lazy_static::lazy_static;
use regex::Regex;

use crate::rockpaperscissors::RockPaperScissors;

pub struct StrategyGuide {}

lazy_static! {
    static ref PAIR: Regex = Regex::new(r"(?P<first>[ABC])\s(?P<second>[XYZ])").unwrap();
}

impl StrategyGuide {
    pub fn selection_pair(line: &str) -> (RockPaperScissors, RockPaperScissors) {
        let capture = PAIR.captures(line).unwrap();
        let first = StrategyGuide::decode(&capture["first"]);
        let second = StrategyGuide::decode(&capture["second"]);
        // reverse order because 'we' are the second column
        (second, first)
    }

    pub fn decode(value: &str) -> RockPaperScissors {
        match value {
            "A" | "X" => RockPaperScissors::Rock,
            "B" | "Y" => RockPaperScissors::Paper,
            "C" | "Z" => RockPaperScissors::Scissors,
            _ => panic!("Unexpected value '{}' - must be A, B, C, X, Y, Z", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!((RockPaperScissors::Paper, RockPaperScissors::Rock), StrategyGuide::selection_pair("A Y"));
        assert_eq!((RockPaperScissors::Rock, RockPaperScissors::Paper), StrategyGuide::selection_pair("B X"));
        assert_eq!((RockPaperScissors::Scissors, RockPaperScissors::Scissors), StrategyGuide::selection_pair("C Z"));
    }
}