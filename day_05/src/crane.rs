use lazy_static::lazy_static;
use regex::Regex;

use crate::crates::Crates;

#[derive(Debug, PartialEq, Eq)]
pub struct Move {
    moves: u8,
    from: u8,
    to: u8,
}

lazy_static! {
    static ref MOVE_COMMAND: Regex = Regex::new(r"move\s(?P<moves>\d+)\sfrom\s(?P<from>\d+)\sto\s(?P<to>\d+)").unwrap();
}

impl Move {
    pub fn from(line: &str) -> Self {
        let captures = MOVE_COMMAND.captures(line).unwrap();
        let moves = captures["moves"].parse::<u8>().unwrap();
        let from = captures["from"].parse::<u8>().unwrap();
        let to = captures["to"].parse::<u8>().unwrap();

        Move { moves, from, to }
    }
}

pub struct Crane {}

impl Crane {
    pub fn move_crates(crates: &mut Crates, m: Move) {
        for _ in 0..m.moves {
            crates.move_crate(m.from as usize, m.to as usize);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Move {
        pub fn new(moves: u8, from: u8, to: u8) -> Self {
            Move { moves, from, to }
        }
    }

    #[test]
    fn move_crates() {
        let mut crates = Crates::from_stacks(vec![
            vec!['Z', 'N'],
            vec!['M', 'C', 'D'],
            vec!['P'],
        ]);
        
        Crane::move_crates(&mut crates, Move::new(1, 2, 1));
        Crane::move_crates(&mut crates, Move::new(3, 1, 3));
        Crane::move_crates(&mut crates, Move::new(2, 2, 1));
        Crane::move_crates(&mut crates, Move::new(1, 1, 2));

        assert_eq!("CMZ", crates.top_crates());
    }

    #[test]
    fn parse() {
        assert_eq!(Move::new(1, 2, 1), Move::from("move 1 from 2 to 1"));
        assert_eq!(Move::new(3, 1, 3), Move::from("move 3 from 1 to 3"));
        assert_eq!(Move::new(2, 2, 1), Move::from("move 2 from 2 to 1"));
        assert_eq!(Move::new(1, 1, 2), Move::from("move 1 from 1 to 2"));
    }
}