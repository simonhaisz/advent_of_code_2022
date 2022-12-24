mod monkey;
mod keep_away;

use std::fs;

use keep_away::{keep_away, monkey_business_level};
use monkey::{parse_monkeys, Monkey};
use util::Timer;

fn main() -> std::io::Result<()> {
    let _timer = Timer::new();
    
    let data = fs::read_to_string("./day_11/input.txt")?;

    let mut monkeys = parse_monkeys(&data);

    run_part_one(&mut monkeys);

    Ok(())
}

fn run_part_one(monkeys: &mut Vec<Monkey>) {
    keep_away(monkeys, 20);

    let monkey_business = monkey_business_level(monkeys);

    println!("{}", monkey_business);
}