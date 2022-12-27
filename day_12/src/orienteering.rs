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

pub fn find_easiest_route(map: &Map, start: &Location, end: &Location) -> Option<Vec<Location>> {
    let start = *start;
    let end = *end;
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

    if !came_from.contains_key(&end) {
        None
    } else {
        let mut path = vec![];
        let mut current = end;
        while current != start {
            if let Some(prev) = came_from.get(&current) {
                path.insert(0, *prev);
                current = *prev;
            } else {
                panic!("Cannot trace trail back to ({}, {})", current.0, current.1);
            }
        }
    
        Some(path)
    }
}

pub fn find_easiest_route_from_easiest_start(map: &Map, end: &Location) -> Vec<Location> {
    let all_starts = map.all_start_locations();

    let mut easiest_route: Option<Vec<Location>> = None;

    for start in all_starts {
        let route = find_easiest_route(map, &start, end);
        if let Some(route) = route {
            let replace = if let Some(easy) = easiest_route.as_ref() {
                route.len() < easy.len()
            } else {
                true
            };
            if replace {
                easiest_route.replace(route);
            }
        }
    }

    easiest_route.unwrap()
}

pub fn print_route(map: &Map, route: &[Location]) -> String {
    let mut printout = vec![];
    for _ in 0..map.height() {
        printout.extend(vec!['.'; map.width()]);
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
        
        let (map, start, end) = Map::from(input);

        let easy = find_easiest_route(&map, &start, &end).unwrap();

        assert_eq!(31, easy.len());

        let printout = print_route(&map, &easy);
        println!("{}", printout);
    }

    #[test]
    fn any_easy_start() {
        let input = "
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
        ";
        
        let (map, _, end) = Map::from(input);

        let easy = find_easiest_route_from_easiest_start(&map, &end);

        assert_eq!(29, easy.len());

        let printout = print_route(&map, &easy);
        println!("{}", printout);
    }
}