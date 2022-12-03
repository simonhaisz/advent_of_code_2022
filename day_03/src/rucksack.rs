use std::collections::HashSet;

pub struct Rucksack {
    common: HashSet<char>,
}

impl Rucksack {
    pub fn from(items: &str) -> Rucksack {
        // each rucksack must be equally splitable across two compartments
        assert!(items.len() % 2 == 0);
        let midpoint = items.len() / 2;
        let contents = vec![
            &items[0..midpoint],
            &items[midpoint..],
        ];
        let common = find_common_characters_pair(contents[0], contents[1]);
        // there should only be one matching item between compartments
        assert_eq!(1, common.len());
        Rucksack { common }
    }

    pub fn common_priority(&self) -> u32 {
        priority(*self.common.iter().next().unwrap())
    }
}

fn find_common_characters_pair(left: &str, right: &str) -> HashSet<char> {
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

pub struct ElfGroup {
    rucksacks: Vec<String>,
}

impl ElfGroup {
    pub fn new() -> ElfGroup {
        ElfGroup { rucksacks: vec![] }
    }

    pub fn add(&mut self, rucksack: String) {
        self.rucksacks.push(rucksack);
    }

    pub fn len(&self) -> usize {
        self.rucksacks.len()
    }

    pub fn priority(&self) -> u32 {
        let common = find_common_characters_set(&self.rucksacks);
        assert_eq!(1, common.len());
        priority(*common.iter().next().unwrap())
    }

    pub fn reset(&mut self) {
        self.rucksacks.clear();
    }
}

fn find_common_characters_set(strings: &Vec<String>) -> HashSet<char> {
    assert!(strings.len() > 1);
    let mut common_characters = HashSet::new();
    let first = &strings[0];
    let others = &strings[1..];
    for first_c in first.chars() {
        let mut match_found = false;
        for other in others.iter() {
            match_found = false;
            for other_c in other.chars() {
                if first_c == other_c {
                    match_found = true;
                }
            }
            if !match_found {
                break;
            }
        }
        if match_found {
            common_characters.insert(first_c);
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
        assert_eq!(vec!['p'], rucksack.common.into_iter().collect::<Vec<char>>());

        let rucksack = Rucksack::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
        assert_eq!(vec!['L'], rucksack.common.into_iter().collect::<Vec<char>>());

        let rucksack = Rucksack::from("PmmdzqPrVvPwwTWBwg");
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

    #[test]
    fn rucksack_sets() {
        let rucksacks = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_owned(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_owned(),
            "PmmdzqPrVvPwwTWBwg".to_owned(),
        ];

        let common_set = find_common_characters_set(&rucksacks);
        assert_eq!(1, common_set.len());
        assert_eq!('r', *common_set.iter().next().unwrap());

        let rucksacks = vec![
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_owned(),
            "ttgJtRGJQctTZtZT".to_owned(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_owned(),
        ];

        let common_set = find_common_characters_set(&rucksacks);
        assert_eq!(1, common_set.len());
        assert_eq!('Z', *common_set.iter().next().unwrap());
    }
}