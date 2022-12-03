use std::{fs::File, io::{BufReader, BufRead, Lines}};

use strategy_guide::{WrongStrategyGuide, CorrectStrategyGuide};
use tournament::Tournament;

mod rockpaperscissors;
mod tournament;
mod strategy_guide;

fn main() -> std::io::Result<()> {
    let file = File::open("./day_02/input.txt")?;
    let lines = BufReader::new(file).lines();

    let mut tournament = Tournament::new();

    run_part_2(lines, &mut tournament);
    Ok(())
}

#[allow(dead_code)]
fn run_part_1(lines: Lines<BufReader<File>>, tournament: &mut Tournament) {
    for line in lines {
        let (a, b) = WrongStrategyGuide::selection_pair(&line.unwrap());

        tournament.game(a, b);
    }

    let total_score = tournament.score();

    println!("Following the wrong strategy guide should give me a score of {}", total_score);
}

fn run_part_2(lines: Lines<BufReader<File>>, tournament: &mut Tournament) {
    for line in lines {
        let (a, b) = CorrectStrategyGuide::selection_pair(&line.unwrap());

        tournament.game(a, b);
    }

    let total_score = tournament.score();

    println!("Following the correct strategy guide should give me a score of {}", total_score);
}