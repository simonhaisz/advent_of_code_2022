use std::ops::RangeInclusive;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PAIR: Regex = Regex::new(r"(?P<first_start>\d+)-(?P<first_end>\d+),(?P<second_start>\d+)-(?P<second_end>\d+)").unwrap();
}
pub fn parse_assignments(line: &str) -> (RangeInclusive<u8>, RangeInclusive<u8>) {
    let capture = PAIR.captures(line)
        .unwrap_or_else(|| panic!("Line '{}' should match the regex", line));
    let parse_id = |v: &str| v.parse::<u8>().unwrap();

    let ids = vec!["first_start", "first_end", "second_start", "second_end"]
        .iter()
        .map(|n| &capture[*n])
        .map(parse_id)
        .collect::<Vec<u8>>();
    
    (ids[0]..=ids[1], ids[2]..=ids[3])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assignments() {
        let (first, second) = parse_assignments("2-4,6-8");
        assert_eq!(2..=4, first);
        assert_eq!(6..=8, second);
    }
}