mod elf;
mod expedition;

use std::fs::File;
use std::io::{BufReader, BufRead};

use elf::Elf;
use expedition::Expedition;

fn main() -> std::io::Result<()> {
    let file = File::open("./day_01/input.txt")?;
    let lines = BufReader::new(file).lines();

    let mut expedition = Expedition::new();
    let mut current_elf: Option<Elf> = None;

    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            assert!(current_elf.is_some(), "Found an empty input line with no elf to add");
            expedition.add_elf(current_elf.take().unwrap());
        } else {
            if current_elf.is_none() {
                current_elf.replace(Elf::new());
            }
            let calories = line.parse().unwrap();
            current_elf.as_mut().unwrap().add_snack(calories);
        }
    }

    run_part_2(&expedition);

    Ok(())
}

#[allow(dead_code)]
fn run_part_1(expedition: &Expedition) {
    let (index, calories) = expedition.find_snack_source().unwrap();

    println!("The best source of snacks is elf {} with {} total calories", index, calories);
}

fn run_part_2(expedition: &Expedition) {
    let top_elf_snack_total = expedition.find_top_snack_sources(3);

    println!("The snack calory total form the top 3 elves is {}", top_elf_snack_total);
}
