mod cpu;

use std::fs;

use cpu::{Program, Operation};
use util::Timer;

fn main() -> std::io::Result<()> {
    let _timer = Timer::new();
    
    let data = fs::read_to_string("./day_10/input.txt")?;

    let mut program = Program::new();

    let ops = Operation::series_from(&data);

    for op in ops {
        program.run(op);
    }

    run_part_2(&program);

    Ok(())
}

#[allow(dead_code)]
fn run_part_1(program: &Program) {
    println!("{}", program.signal_strength_sum(&[20, 60, 100, 140, 180, 220]));
}

fn run_part_2(program: &Program) {
    println!("{}", program.draw_pixels());
}