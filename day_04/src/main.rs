mod assignment;
mod parser;

use std::{fs::File, io::{BufReader, BufRead}};

use assignment::assignments_redundant;
use parser::parse_assignments;

fn main() -> std::io::Result<()> {
    let file = File::open("./day_04/input.txt")?;
    let mut reader = BufReader::new(file);

    run_part_1(&mut reader);
    Ok(())
}

fn run_part_1(reader: &mut BufReader<File>) {
    let count = reader.lines()
        .map(Result::unwrap)
        .map(|v| parse_assignments(&v))
        .map(|(a, b)| assignments_redundant(a, b))
        .filter(|b| *b)
        .count();
    
    println!("There are {} assignment pairs where work is completly duplicated", count);
}