use std::fs;

use data_stream::DataStream;
use util::Timer;

mod data_stream;

fn main() -> std::io::Result<()> {
    let _timer = Timer::new();
    
    let data = fs::read_to_string("./day_06/input.txt")?;
    let stream = DataStream::new(data.trim());

    println!("{}", run_part_2(&stream));

    Ok(())
}

#[allow(dead_code)]
fn run_part_1(stream: &DataStream) -> usize {
    stream.find_packet_marker().unwrap()
}

fn run_part_2(stream: &DataStream) -> usize {
    stream.find_message_marker().unwrap()
}