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
                if let Some(m) = capture.name("crate") {
                    let name = m.as_str().chars().next().unwrap();
                    // stacks start at the bottom but the lines start at the top
                    // always insert new crates to the bottom of the stack
                    stacks[column].insert(0, name);
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

    pub fn move_crates(&mut self, count: usize, from: usize, to: usize) {
        let crates = self.take_crates(count, from - 1);
        self.give_crates(crates, to - 1);
    }

    fn take_crates(&mut self, count: usize, index: usize) -> Vec<char> {
        let stack = self.stacks.get_mut(index).unwrap();
        let range_start = stack.len() - count;
        let range_end = stack.len();
        stack.splice(range_start..range_end, []).collect::<Vec<char>>()
    }

    fn give_crates(&mut self, crates: Vec<char>, index: usize) {
        let stack = self.stacks.get_mut(index).unwrap();
        stack.extend(crates);

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