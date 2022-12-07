mod rucksack;

use std::{fs::File, io::{BufReader, BufRead, Lines}};

use rucksack::ElfGroup;
use util::Timer;

use crate::rucksack::Rucksack;

fn main() -> std::io::Result<()> {
    let _timer: Timer;
    
    let file = File::open("./day_03/input.txt")?;
    let lines = BufReader::new(file).lines();

    run_part_2(lines);

    Ok(())
}

#[allow(dead_code)]
fn run_part_1(lines: Lines<BufReader<File>>) {
    let total_priority = lines
        .map(Result::unwrap)
        .map(|l| Rucksack::from(&l))
        .map(|r| r.common_priority())
        .sum::<u32>();

    println!("The sum of the common priorities in all rucksacks is {}", total_priority);
}

fn run_part_2(lines: Lines<BufReader<File>>) {
    let mut total_priority = 0;
    let mut elf_group = ElfGroup::new();
    for line in lines {
        let line = line.unwrap();
        elf_group.add(line);
        if elf_group.len() == 3 {
            total_priority += elf_group.priority();
            elf_group.reset();
        }
    }

    println!("The sum of the common badge priorities in all three-elf groups is {}", total_priority);
}