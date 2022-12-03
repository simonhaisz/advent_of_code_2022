mod rucksack;

use std::{fs::File, io::{BufReader, BufRead, Lines}};

use crate::rucksack::Rucksack;

fn main() -> std::io::Result<()> {
    let file = File::open("./day_03/input.txt")?;
    let lines = BufReader::new(file).lines();

    run_part_1(lines);

    Ok(())
}

fn run_part_1(lines: Lines<BufReader<File>>) {
    let mut total_priority = 0;

    for line in lines {
        let line = line.unwrap();
        let rucksack = Rucksack::from(&line);

        total_priority += rucksack.common_priority();
    }

    println!("The sum of the common priorities in all rucksacks is {}", total_priority);
}