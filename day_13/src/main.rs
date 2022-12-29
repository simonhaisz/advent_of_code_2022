mod packet;
mod signal;

use std::fs;

use packet::Packet;
use signal::validate_packets;
use util::Timer;

fn main() -> std::io::Result<()> {
    let _timer = Timer::new();

    let data = fs::read_to_string("./day_13/input.txt")?;

    run_part_two(data);

    Ok(())
}

#[allow(dead_code)]
fn run_part_one(data: String) {

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
}

fn run_part_two(data: String) {
    let mut packets = data.split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| Packet::from(l))
        .collect::<Vec<Packet>>();

    let first_divider = Packet::from("[[2]]");
    let second_divider = Packet::from("[[6]]");
    packets.push(first_divider.clone());
    packets.push(second_divider.clone());

    packets.sort();

    let mut first_divider_index = None;
    let mut second_divider_index = None;

    for (i, p) in packets.iter().enumerate() {
        if *p == first_divider {
            assert!(first_divider_index.is_none());
            first_divider_index = Some(i + 1);
        } else if *p == second_divider {
            assert!(second_divider_index.is_none());
            second_divider_index = Some(i + 1);
        }
    }

    let first_divider_index = first_divider_index.unwrap();
    let second_divider_index = second_divider_index.unwrap();

    let key = first_divider_index * second_divider_index;

    println!("{}", key);
}