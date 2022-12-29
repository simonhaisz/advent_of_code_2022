mod packet;
mod signal;

use std::fs;

use packet::Packet;
use signal::validate_packets;
use util::Timer;

fn main() -> std::io::Result<()> {
    let _timer = Timer::new();

    let data = fs::read_to_string("./day_13/input.txt")?;

    let mut i = 1;
    let mut valid_sum = 0;

    let mut line_iter = data.split("\n");
    loop {
        let first = line_iter.next();
        if first.is_none() {
            // input is pairs of lines with a whitespace line in between
            break;
        }
        let a = Packet::from(first.unwrap());
        let b = Packet::from(line_iter.next().unwrap());
        line_iter.next().unwrap();

        if validate_packets(&a, &b) {
            valid_sum += i;
        }
        i += 1;
    }

    println!("{}", valid_sum);

    Ok(())
}
