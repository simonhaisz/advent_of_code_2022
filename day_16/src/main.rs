mod valve;
mod network;

use std::fs;

use util::Timer;

fn main() -> std::io::Result<()> {
    let _timer = Timer::new();

    let input = fs::read_to_string("./day_13/input.txt")?;

    run_part_1(input);

    Ok(())
}

fn run_part_1(input: String) {

}