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
    
    let mut rope = Rope::new();
    for line in lines {
        let mv = Move::from(line);
        rope.move_head(&mv);
    }

    println!("{}", rope.tail_trace_count());
    Ok(())
}
