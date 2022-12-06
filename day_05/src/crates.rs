use lazy_static::lazy_static;
use regex::Regex;

type Crate = char;

#[derive(Default)]
pub struct Crates {
    stacks: Vec<Vec<Crate>>,
}

lazy_static! {
    static ref HORIZONTAL_CRATES: Regex = Regex::new(r"(\[[A-Z]\]|\s{3})\s?").unwrap();
}

impl Crates {
    pub fn new() -> Self {
        Crates { stacks: vec![] }
    }

    pub fn from(lines: &[String]) -> Self {
        let mut stacks: Option<Vec<Vec<char>>> = None;
        for line in lines {
            let captures = HORIZONTAL_CRATES.captures(line).expect(&format!("Line '{}' should match the regex", line));

            if stacks.is_none() {
                stacks.replace(Vec::with_capacity(captures.len()));
            }
            for (column, c) in captures.iter().enumerate() {
                match c {
                    Some(m) => {
                        let text = m.as_str();
                        let mut chars = text.chars();
                        let one = chars.next().unwrap();
                        let two = chars.next().unwrap();
                        let three = chars.next().unwrap();
                        if two.is_alphabetic() {
                            stacks.as_mut().unwrap()[column].push(two);
                        }
                    },
                    None => panic!("No match from line '{}'", line)
                }
            }
        }

        let stacks = stacks.unwrap();

        Crates { stacks }
    }

    pub fn from_stacks(stacks: Vec<Vec<Crate>>) -> Self {
        Crates { stacks }
    }

    pub fn top_crates(&self) -> String {
        self.stacks
            .iter()
            .map(|s| s.last().unwrap())
            .collect::<String>()
    }

    pub fn move_crate(&mut self, from: usize, to: usize) {
        let c = self.stacks[from - 1].pop().unwrap();
        self.stacks[to - 1].push(c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top_crates() {
        let crates = Crates::from_stacks(vec![
            vec!['C'],
            vec!['M'],
            vec!['P', 'D', 'N', 'Z'],
        ]);

        assert_eq!("CMZ", crates.top_crates());
    }

    #[test]
    fn parse() {
        let lines = vec![
            "    [D]    ".to_owned(),
            "[N] [C]    ".to_owned(),
            "[Z] [M] [P]".to_owned(),
        ];

        let crates = Crates::from(&lines);

        assert_eq!("NDP", crates.top_crates());
    }
}