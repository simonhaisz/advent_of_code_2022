mod map;
mod orienteering;

use std::fs;

use map::Map;
use orienteering::find_easiest_route;
use util::Timer;

use crate::orienteering::print_route;

fn main() -> std::io::Result<()> {
    let _timer = Timer::new();

    let data = fs::read_to_string("./day_12/input.txt")?;

    let map = Map::from(&data);

    let easy = find_easiest_route(&map);

    println!("{}", easy.len());

    println!("{}", print_route(&map, &easy));
    Ok(())
}
