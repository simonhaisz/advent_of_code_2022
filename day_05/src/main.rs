mod crates;
mod crane;

use std::{fs::File, io::{BufReader, BufRead}};
use crane::{Move, Crane, CrateMover9001};
use crates::Crates;
use lazy_static::lazy_static;
use regex::Regex;
use util::Timer;

use crate::crane::CrateMover9000;

fn main() -> std::io::Result<()> {
    let _timer: Timer;

    let file = File::open("./day_05/input.txt")?;

    let mut state = InputReaderState::CrateStacks;

    let mut crate_stack_lines = vec![];
    let mut move_lines = vec![];

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();

        if line.is_empty() {
            continue;
        }

        match state {
            InputReaderState::CrateStacks => {
                if CRATE_STACK_NUMBER_LINE.is_match(&line) {
                    state = InputReaderState::Moves;
                } else {
                    crate_stack_lines.push(line);
                }
            },
            InputReaderState::Moves => {
                move_lines.push(line);
            },
        }
    }

    let mut crates = Crates::from(&crate_stack_lines);
    let moves = move_lines
        .iter()
        .map(|l| Move::from(l))
        .collect::<Vec<_>>();
    
    run_part_2(&mut crates, &moves);

    println!("After all of the moves the top crates are '{}'", crates.top_crates());

    Ok(())
}

enum InputReaderState {
    CrateStacks,
    Moves,
}

lazy_static! {
    static ref CRATE_STACK_NUMBER_LINE: Regex = Regex::new(r"^(\s+(\d+)\s+)+$").unwrap();
}

#[allow(dead_code)]
fn run_part_1(crates: &mut Crates, moves: &[Move]) {
    for m in moves {
        CrateMover9000::move_crates(crates, m);
    }
}

fn run_part_2(crates: &mut Crates, moves: &[Move]) {
    for m in moves {
        CrateMover9001::move_crates(crates, m);
    }
}