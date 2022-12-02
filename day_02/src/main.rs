#[macro_use]
extern crate lazy_static;

use std::{fs::File, io::{BufReader, BufRead, Lines}};

use strategy_guide::StrategyGuide;
use tournament::Tournament;

mod rockpaperscissors;
mod tournament;
mod strategy_guide;

fn main() -> std::io::Result<()> {
    let file = File::open("./day_02/input.txt")?;
    let lines = BufReader::new(file).lines();

    let mut tournament = Tournament::new();

    run_part_1(lines, &mut tournament);
    Ok(())
}

fn run_part_1(lines: Lines<BufReader<File>>, tournament: &mut Tournament) {
    for line in lines {
        let (a, b) = StrategyGuide::selection_pair(&line.unwrap());

        tournament.game(a, b);
    }

    let total_score = tournament.score();

    println!("Following the strategy guide should give me a score of {}", total_score);
}