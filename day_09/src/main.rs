mod rope;

use std::fs;

use rope::{Rope, Move};
use util::Timer;


fn main() -> std::io::Result<()> {
    let _timer = Timer::new();

    let data = fs::read_to_string("./day_09/input.txt")?;

    let lines = data
            .split('\n')
            .filter(|l| !l.is_empty())
            .collect::<Vec<&str>>();
    
    let mut rope = part_2_create();
    for line in lines {
        let mv = Move::from(line);
        rope.move_head(&mv);
    }

    println!("{}", rope.tail_trace_count());
    Ok(())
}

#[allow(dead_code)]
fn part_1_create() -> Rope {
    Rope::new(2)
}

fn part_2_create() -> Rope {
    Rope::new(10)
}