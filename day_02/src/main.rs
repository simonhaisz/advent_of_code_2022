mod rockpaperscissors;
mod tournament;
mod strategy_guide;

use std::{fs::File, io::{BufReader, BufRead, Lines}};

use strategy_guide::{StrategyGuide, WrongStrategyGuide, CorrectStrategyGuide};
use tournament::Tournament;



fn main() -> std::io::Result<()> {
    let file = File::open("./day_02/input.txt")?;
    let lines = BufReader::new(file).lines();

    run_part_2(lines);
    Ok(())
}

#[allow(dead_code)]
fn run_part_1(lines: Lines<BufReader<File>>) {
    let total_score = lines
        .map(|l| WrongStrategyGuide::selection_pair(&l.unwrap()))
        .map(|(a, b)| Tournament::score_game(a, b))
        .sum::<u32>();

    println!("Following the wrong strategy guide should give me a score of {}", total_score);
}

fn run_part_2(lines: Lines<BufReader<File>>) {
    let total_score = lines
        .map(|l| CorrectStrategyGuide::selection_pair(&l.unwrap()))
        .map(|(a, b)| Tournament::score_game(a, b))
        .sum::<u32>();

    println!("Following the correct strategy guide should give me a score of {}", total_score);
}