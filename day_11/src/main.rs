mod monkey;
mod keep_away;

use std::fs;

use keep_away::{keep_away, monkey_business_level};
use monkey::{parse_monkeys, Monkey};
use util::Timer;

use crate::{keep_away::common_factors, monkey::InspectionRelief};

fn main() -> std::io::Result<()> {
    let _timer = Timer::new();
    
    let data = fs::read_to_string("./day_11/input.txt")?;

    let mut monkeys = parse_monkeys(&data);

    run_part_two(&mut monkeys);

    Ok(())
}

#[allow(dead_code)]
fn run_part_one(monkeys: &mut Vec<Monkey>) {
    keep_away(monkeys, 20, &InspectionRelief::TwoThirds);

    let monkey_business = monkey_business_level(monkeys);

    println!("{}", monkey_business);
}

fn run_part_two(monkeys: &mut Vec<Monkey>) {
    let common_factors = common_factors(monkeys);
    keep_away(monkeys, 10000, &InspectionRelief::None(common_factors));

    let monkey_business = monkey_business_level(monkeys);

    println!("{}", monkey_business);
}