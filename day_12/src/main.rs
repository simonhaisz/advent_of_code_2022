mod map;
mod orienteering;

use std::fs;

use map::{Map, Location};
use orienteering::{find_easiest_route, find_easiest_route_from_easiest_start};
use util::Timer;

use crate::orienteering::print_route;

fn main() -> std::io::Result<()> {
    let _timer = Timer::new();

    let data = fs::read_to_string("./day_12/input.txt")?;

    let (map, _start, end) = Map::from(&data);

    run_part_2(&map, &end);

    Ok(())
}

#[allow(dead_code)]
fn run_part_1(map: &Map, start: &Location, end: &Location) {
    let easy = find_easiest_route(map, start, end).unwrap();

    println!("{}", easy.len());

    println!("{}", print_route(&map, &easy));
}

fn run_part_2(map: &Map, end: &Location) {
    let easy = find_easiest_route_from_easiest_start(map, end);

    println!("{}", easy.len());

    println!("{}", print_route(&map, &easy));
}