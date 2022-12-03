use std::collections::HashSet;

pub struct Rucksack<'a> {
    contents: Vec<&'a str>,
    common: HashSet<char>,
}

impl <'a> Rucksack<'a> {
    pub fn from(items: &'a str) -> Rucksack {
        // each rucksack must be equally splitable across two compartments
        assert!(items.len() % 2 == 0);
        let midpoint = items.len() / 2;
        let contents = vec![
            &items[0..midpoint],
            &items[midpoint..],
        ];
        let common = find_common_characters(contents[0], contents[1]);
        // there should only be one matching item between compartments
        assert_eq!(1, common.len());
        Rucksack { contents, common }
    }

    pub fn common_priority(&self) -> u32 {
        priority(*self.common.iter().next().unwrap())
    }
}

fn find_common_characters(left: &str, right: &str) -> HashSet<char> {
    let mut common_characters = HashSet::new();
    for left_c in left.chars() {
        for right_c in right.chars() {
            if left_c == right_c {
                common_characters.insert(left_c);
            }
        }
    }
    common_characters
}

fn priority(character: char) -> u32 {
    let code = character as u32;
    match code {
        // A-Z have priority 27-52
        65..=90 => code - 38,
        // a-z have priority 1-26
        97..=122 => code - 96,
        _ => panic!("Character '{}' has a code {} outside the valid range", character, code),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rucksack() {
        let rucksack = Rucksack::from("vJrwpWtwJgWrhcsFMMfFFhFp");
        assert_eq!(vec!["vJrwpWtwJgWr", "hcsFMMfFFhFp"], rucksack.contents);
        assert_eq!(vec!['p'], rucksack.common.into_iter().collect::<Vec<char>>());

        let rucksack = Rucksack::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
        assert_eq!(vec!["jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"], rucksack.contents);
        assert_eq!(vec!['L'], rucksack.common.into_iter().collect::<Vec<char>>());

        let rucksack = Rucksack::from("PmmdzqPrVvPwwTWBwg");
        assert_eq!(vec!["PmmdzqPrV", "vPwwTWBwg"], rucksack.contents);
        assert_eq!(vec!['P'], rucksack.common.into_iter().collect::<Vec<char>>());
    }

    #[test]
    fn priority() {
        assert_eq!(16, super::priority('p'));
        assert_eq!(38, super::priority('L'));
        assert_eq!(42, super::priority('P'));
        assert_eq!(22, super::priority('v'));
        assert_eq!(20, super::priority('t'));
        assert_eq!(19, super::priority('s'));
    }
}