use crate::monkey::{Monkey, InspectionRelief};

pub fn common_factors(monkeys: &[Monkey]) -> u64 {
    monkeys.iter()
        .map(|m| m.divisor())
        .product()
}

pub fn keep_away(monkeys: &mut Vec<Monkey>, rounds: u32, relief: &InspectionRelief) {

    for _ in 0..rounds {
        for index in 0..monkeys.len() {

            let mut items_to_throw = vec![];
            {
                let throwing_monkey = monkeys.get_mut(index).unwrap();

                while let Some(throw) = throwing_monkey.inspect(relief) {
                    items_to_throw.push(throw);
                }
            }

            for (target, item) in items_to_throw {
                let other_monkey = monkeys.get_mut(target as usize).unwrap();
                other_monkey.catch(item);
            }
        }
    }
}

pub fn monkey_business_level(monkeys: &Vec<Monkey>) -> u64 {
    let mut item_inspections = monkeys.iter()
        .map(|m| m.inspection_count())
        .collect::<Vec<u64>>();
    
    item_inspections.sort();

    item_inspections.iter().rev().take(2).product()
}

#[cfg(test)]
mod tests {
    use crate::monkey::parse_monkeys;

    use super::*;

    #[test]
    fn sample_two_thirds() {
        let input = "Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3
    
    Monkey 1:
        Starting items: 54, 65, 75, 74
        Operation: new = old + 6
        Test: divisible by 19
            If true: throw to monkey 2
            If false: throw to monkey 0
    
    Monkey 2:
        Starting items: 79, 60, 97
        Operation: new = old * old
        Test: divisible by 13
            If true: throw to monkey 1
            If false: throw to monkey 3
    
    Monkey 3:
        Starting items: 74
        Operation: new = old + 3
        Test: divisible by 17
            If true: throw to monkey 0
            If false: throw to monkey 1";
        
        let mut monkeys = parse_monkeys(input);

        let relief = InspectionRelief::TwoThirds;

        keep_away(&mut monkeys, 1, &relief);

        assert_eq!(4, monkeys[0].items().len());
        assert_eq!(6, monkeys[1].items().len());
        assert_eq!(0, monkeys[2].items().len());
        assert_eq!(0, monkeys[3].items().len());

        assert_eq!(&vec![20, 23, 27, 26], monkeys[0].items());
        assert_eq!(&vec![2080, 25, 167, 207, 401, 1046], monkeys[1].items());

        // 1 + 9 = 10
        keep_away(&mut monkeys, 9, &relief);

        assert_eq!(4, monkeys[0].items().len());
        assert_eq!(6, monkeys[1].items().len());
        assert_eq!(0, monkeys[2].items().len());
        assert_eq!(0, monkeys[3].items().len());

        assert_eq!(&vec![91, 16, 20, 98], monkeys[0].items());
        assert_eq!(&vec![481, 245, 22, 26, 1092, 30], monkeys[1].items());

        // 10 + 10 = 20
        keep_away(&mut monkeys, 10, &relief);

        assert_eq!(5, monkeys[0].items().len());
        assert_eq!(5, monkeys[1].items().len());
        assert_eq!(0, monkeys[2].items().len());
        assert_eq!(0, monkeys[3].items().len());

        assert_eq!(&vec![10, 12, 14, 26, 34], monkeys[0].items());
        assert_eq!(&vec![245, 93, 53, 199, 115], monkeys[1].items());

        let monkey_business = monkey_business_level(&monkeys);

        assert_eq!(10605, monkey_business);
    }

    #[test]
    fn sample_none() {
        let input = "Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3
    
    Monkey 1:
        Starting items: 54, 65, 75, 74
        Operation: new = old + 6
        Test: divisible by 19
            If true: throw to monkey 2
            If false: throw to monkey 0
    
    Monkey 2:
        Starting items: 79, 60, 97
        Operation: new = old * old
        Test: divisible by 13
            If true: throw to monkey 1
            If false: throw to monkey 3
    
    Monkey 3:
        Starting items: 74
        Operation: new = old + 3
        Test: divisible by 17
            If true: throw to monkey 0
            If false: throw to monkey 1";
        
        let mut monkeys = parse_monkeys(input);
        
        let common_factors = common_factors(&monkeys);

        let relief = InspectionRelief::None(common_factors);

        keep_away(&mut monkeys, 1, &relief);

        assert_eq!(2, monkeys[0].inspection_count());
        assert_eq!(4, monkeys[1].inspection_count());
        assert_eq!(3, monkeys[2].inspection_count());
        assert_eq!(6, monkeys[3].inspection_count());

        // 1 + 19 = 20
        keep_away(&mut monkeys, 19, &relief);

        assert_eq!(99, monkeys[0].inspection_count());
        assert_eq!(97, monkeys[1].inspection_count());
        assert_eq!(8, monkeys[2].inspection_count());
        assert_eq!(103, monkeys[3].inspection_count());


        // 20 + 980 = 1000
        keep_away(&mut monkeys, 980, &relief);

        assert_eq!(5204, monkeys[0].inspection_count());
        assert_eq!(4792, monkeys[1].inspection_count());
        assert_eq!(199, monkeys[2].inspection_count());
        assert_eq!(5192, monkeys[3].inspection_count());

        // 1000 + 1000 = 2000
        keep_away(&mut monkeys, 1000, &relief);

        assert_eq!(10419, monkeys[0].inspection_count());
        assert_eq!(9577, monkeys[1].inspection_count());
        assert_eq!(392, monkeys[2].inspection_count());
        assert_eq!(10391, monkeys[3].inspection_count());

        // 1000 + 3000 = 5000
        keep_away(&mut monkeys, 3000, &relief);

        assert_eq!(26075, monkeys[0].inspection_count());
        assert_eq!(23921, monkeys[1].inspection_count());
        assert_eq!(974, monkeys[2].inspection_count());
        assert_eq!(26000, monkeys[3].inspection_count());

        // 5000 + 5000 = 10000
        keep_away(&mut monkeys, 5000, &relief);

        assert_eq!(52166, monkeys[0].inspection_count());
        assert_eq!(47830, monkeys[1].inspection_count());
        assert_eq!(1938, monkeys[2].inspection_count());
        assert_eq!(52013, monkeys[3].inspection_count());

        let monkey_business = monkey_business_level(&monkeys);

        assert_eq!(2713310158, monkey_business);
    }
}