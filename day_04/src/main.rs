mod assignment;
mod parser;

use std::{fs::File, io::{BufReader, BufRead}};

use assignment::{assignments_redundant, assignments_have_waste};
use parser::parse_assignments;
use util::Timer;

fn main() -> std::io::Result<()> {
    let _timer: Timer;
    
    let file = File::open("./day_04/input.txt")?;

    run_part_2(&file);

    Ok(())
}

#[allow(dead_code)]
fn run_part_1(file: &File) {
    let count = BufReader::new(file).lines()
        .map(Result::unwrap)
        .map(|v| parse_assignments(&v))
        .map(|(a, b)| assignments_redundant(a, b))
        .filter(|b| *b)
        .count();
    
    println!("There are {} assignment pairs where work is completly duplicated", count);
}

fn run_part_2(file: &File) {
    let count = BufReader::new(file).lines()
        .map(Result::unwrap)
        .map(|v| parse_assignments(&v))
        .map(|(a, b)| assignments_have_waste(a, b))
        .filter(|b| *b)
        .count();
    
        println!("There are {} assignment pairs where some work is wasted", count);
}