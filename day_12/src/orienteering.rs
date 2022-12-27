use std::collections::BinaryHeap;
use std::collections::HashMap;

use crate::map::Map;
use crate::map::Location;

struct PriorityLocation {
    location: Location,
    priority: u64,
}

impl PriorityLocation {
    fn new(location: Location, priority: u64) -> Self {
        PriorityLocation { location, priority }
    }
}

impl PartialEq for PriorityLocation {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for PriorityLocation {}

impl PartialOrd for PriorityLocation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PriorityLocation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.priority.cmp(&self.priority)
    }
}

pub fn find_easiest_route(map: &Map) -> Vec<Location> {
    let start = map.start_location();
    let end = map.end_location();
    let mut frontier = BinaryHeap::new();
    frontier.push(PriorityLocation::new(start, 0));

    let mut came_from: HashMap<Location, Location> = HashMap::new();
    let mut movement_so_far: HashMap<Location, u64> = HashMap::new();
    movement_so_far.insert(start, 0);

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();

        if current.location == end {
            break;
        }

        for next in map.neighbors(&current.location) {
            let new_travel = movement_so_far[&current.location] + 1;
            if !movement_so_far.contains_key(&next) || new_travel < movement_so_far[&next] {
                movement_so_far.insert(next, new_travel);
                frontier.push(PriorityLocation::new(next, new_travel));
                came_from.insert(next, current.location);
            }
        }
    }

    let mut path = vec![];
    let mut current = end;
    while current != start {
        let prev = came_from[&current];
        path.insert(0, prev);
        current = prev;
    }

    path
}

pub fn print_route(map: &Map, route: &[Location]) -> String {
    let mut printout = vec![];
    for _ in 0..map.height() {
        for _ in 0..map.width() {
            printout.push('.');
        }
        printout.push('\n');
    }

    let mut steps = route.iter();

    let mut current = steps.next();
    let mut next = steps.next();

    let step = |c: &Location, n: Option<&Location>| {
        if n.is_none() {
            return 'E';
        }
        let n = n.unwrap();
        let horizontal = n.0 - c.0;
        let vertical = n.1 - c.1;
        assert!(horizontal != 0 || vertical != 0);
        assert!(!(horizontal != 0 && vertical != 0));
        if horizontal != 0 {
            match horizontal {
                1 => '>',
                -1 => '<',
                _ => panic!("horizontal step can be at most 1 - found {}", horizontal),
            }
        } else if vertical != 0 {
            match vertical {
                1 => 'v',
                -1 => '^',
                _ => panic!("vertical step can be at most 1 - found {}", vertical),
            }
        } else {
            unimplemented!()
        }
    };

    while !(current.is_none() && next.is_none()) {
        assert!(current.is_some());
        let c = current.unwrap();
        let n = next;
        let s = step(c, n);

        let index = c.0 as usize + c.1 as usize * (map.width() + 1);
        printout[index] = s;

        current = next;
        next = steps.next();
    }

    String::from_iter(printout)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
        ";
        
        let map = Map::from(input);

        let easy = find_easiest_route(&map);

        assert_eq!(31, easy.len());

        let printout = print_route(&map, &easy);
        println!("{}", printout);
    }
}