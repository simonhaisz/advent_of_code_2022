use lazy_static::lazy_static;
use regex::Regex;

type Crate = char;

#[derive(Default)]
pub struct Crates {
    stacks: Vec<Vec<Crate>>,
}

lazy_static! {
    static ref HORIZONTAL_CRATES: Regex = Regex::new(r"(\[(?P<crate>[A-Z])\]|(\s{3}))\s?").unwrap();
}

impl Crates {
    pub fn from(lines: &[String]) -> Self {
        let mut stacks = vec![];
        for line in lines {
            let captures = HORIZONTAL_CRATES.captures_iter(line);

            for (column, capture) in captures.enumerate() {
                if stacks.len() == column {
                    stacks.push(vec![]);
                }
                match capture.name("crate") {
                    Some(m) => {
                        let name = m.as_str().chars().nth(0).unwrap();
                        // stacks start at the bottom but the lines start at the top
                        // always insert new crates to the bottom of the stack
                        stacks[column].insert(0, name);
                    },
                    None => {},
                }
            }
        }

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

    impl Crates {
        pub fn from_stacks(stacks: Vec<Vec<Crate>>) -> Self {
            Crates { stacks }
        }
    }

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