use std::ops::RangeInclusive;

pub type Assignment = RangeInclusive<u8>;

pub fn assignments_redundant(a: Assignment, b: Assignment) -> bool {
    assert!(a.start() <= a.end());
    assert!(b.start() <= b.end());

    (a.start() <= b.start() && a.end() >= b.end())
    ||
    (b.start() <= a.start() && b.end() >= a.end())
}

pub fn assignments_have_waste(a: Assignment, b: Assignment) -> bool {
    assert!(a.start() <= a.end());
    assert!(b.start() <= b.end());

    (a.start() <= b.start() && a.end() >= b.end())
    ||
    (a.start() >= b.start() && a.start() <= b.end())
    ||
    (b.end() >= a.start() && b.end() <= a.end())
    ||
    (b.start() <= a.start() && b.end() >= a.end())
    ||
    (b.start() >= a.start() && b.start() <= a.end())
    ||
    (b.end() >= a.start() && b.end() <= a.end())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_overlap() {
        assert_eq!(false, assignments_redundant(11..=20, 21..=30));
        assert_eq!(false, assignments_redundant(1..=10, 11..=20));

        assert_eq!(false, assignments_have_waste(11..=20, 21..=30));
        assert_eq!(false, assignments_have_waste(1..=10, 11..=20));
    }

    #[test]
    fn some_overlap() {
        assert_eq!(false, assignments_redundant(11..=20, 16..=25));
        assert_eq!(false, assignments_redundant(6..=15, 11..=20));

        assert_eq!(false, assignments_redundant(11..=20, 20..=25));
        assert_eq!(false, assignments_redundant(5..=11, 11..=20));

        assert_eq!(true, assignments_have_waste(11..=20, 16..=25));
        assert_eq!(true, assignments_have_waste(6..=15, 11..=20));

        assert_eq!(true, assignments_have_waste(11..=20, 20..=25));
        assert_eq!(true, assignments_have_waste(5..=11, 11..=20));
    }

    #[test]
    fn full_overlap() {
        assert_eq!(true, assignments_redundant(1..=10, 2..=8));
        assert_eq!(true, assignments_redundant(2..=8, 1..=10));

        assert_eq!(true, assignments_redundant(1..=10, 10..=10));
        assert_eq!(true, assignments_redundant(1..=1, 1..=10));

        assert_eq!(true, assignments_have_waste(1..=10, 2..=8));
        assert_eq!(true, assignments_have_waste(2..=8, 1..=10));

        assert_eq!(true, assignments_have_waste(1..=10, 10..=10));
        assert_eq!(true, assignments_have_waste(1..=1, 1..=10));
    }
}