use std::fs;

use data_stream::DataStream;

mod data_stream;

fn main() -> std::io::Result<()> {
    let data = fs::read_to_string("./day_06/input.txt")?;
    let stream = DataStream::new(&data.trim());

    println!("{}", stream.find_packet_marker().unwrap());
    Ok(())
}
