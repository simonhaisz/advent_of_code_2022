use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "monkeys.pest"]
pub struct MonkeyParser;

enum Param {
    Old,
    Value(i32),
}

impl Param {
    fn from(input: &str) -> Self {
        if input == "old" {
            Param::Old
        } else {
            Param::Value(input.parse().unwrap())
        }
    }
}

enum Operator {
    Add,
    Multiply
}

impl Operator {
    fn from(input: &str) -> Self {
        match input {
            "+" => Operator::Add,
            "*" => Operator::Multiply,
            _ => panic!("Invalid operator '{}'", input)
        }
    }
}

struct Operation {
    a: Param,
    b: Param,
    op: Operator,
}

pub struct Monkey {
    id: u8,
    starting_items: Vec<i32>,
    operation: Operation,
    test_divisor: i32,
    test_pass_throw_target: u8,
    test_fail_throw_target: u8,
}

pub fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let parsed_monkeys = MonkeyParser::parse(Rule::monkeys, input)
    .expect("failed to parse monkeys")
    .next().unwrap();

    let mut monkeys = vec![];

    for parsed_monkey in parsed_monkeys.into_inner() {
        match parsed_monkey.as_rule() {
            Rule::monkey => {
                let mut id: Option<u8> = None;
                let mut starting_items: Option<Vec<i32>> = None;
                let mut operation: Option<Operation> = None;
                let mut test_divisor: Option<i32> = None;
                let mut test_pass_throw_target: Option<u8> = None;
                let mut test_fail_throw_target: Option<u8> = None;

                for parsed_monkey_inner in parsed_monkey.into_inner() {
                    match parsed_monkey_inner.as_rule() {
                        Rule::declaration => {
                            let parsed_id = parsed_monkey_inner.into_inner().next().unwrap().as_str()
                                .parse::<u8>().unwrap();

                            id.replace(parsed_id);
                        },
                        Rule::starting_items => {
                            let mut items = vec![];

                            for parsed_item in parsed_monkey_inner.into_inner() {
                                let item = parsed_item.as_str()
                                    .parse::<i32>().unwrap();

                                items.push(item);
                            }

                            starting_items.replace(items);
                        },
                        Rule::operation => {
                            let mut parsed_op_components = parsed_monkey_inner.into_inner();
                            let a = Param::from(parsed_op_components.next().unwrap().as_str());
                            let op = Operator::from(parsed_op_components.next().unwrap().as_str());
                            let b = Param::from(parsed_op_components.next().unwrap().as_str());

                            operation.replace(Operation { a, op, b });
                        },
                        Rule::test => {
                            let divisor = parsed_monkey_inner.into_inner().next().unwrap().as_str()
                                .parse::<i32>().unwrap();

                            test_divisor.replace(divisor);
                        },
                        Rule::if_true => {
                            let target = parsed_monkey_inner.into_inner().next().unwrap().as_str()
                                .parse::<u8>().unwrap();

                            test_pass_throw_target.replace(target);
                        },
                        Rule::if_false => {
                            let target = parsed_monkey_inner.into_inner().next().unwrap().as_str()
                            .parse::<u8>().unwrap();

                            test_fail_throw_target.replace(target);
                        },
                        _ => panic!("unexpected rule '{:?}': {}", parsed_monkey_inner.as_rule(), parsed_monkey_inner.as_str())
                    }
                }

                assert!(id.is_some());
                assert!(starting_items.is_some());
                assert!(operation.is_some());
                assert!(test_divisor.is_some());
                assert!(test_pass_throw_target.is_some());
                assert!(test_fail_throw_target.is_some());

                let monkey = Monkey {
                    id: id.unwrap(),
                    starting_items: starting_items.unwrap(),
                    operation: operation.unwrap(),
                    test_divisor: test_divisor.unwrap(),
                    test_pass_throw_target: test_pass_throw_target.unwrap(),
                    test_fail_throw_target: test_fail_throw_target.unwrap(),
                };
                monkeys.push(monkey);
            },
            _ => panic!("unexpected rule")
        }
    }

    monkeys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_parse() {
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
        
        let monkeys = parse_monkeys(input);

        assert_eq!(4, monkeys.len());
    }
}