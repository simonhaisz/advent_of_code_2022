pub struct Map {
    grid: Vec<u8>,
    width: usize,
    height: usize,
}

pub type Location = (i64, i64);

trait GridLocation {
    type Item;

    fn up(&self) -> Self::Item;
    fn right(&self) -> Self::Item;
    fn down(&self) -> Self::Item;
    fn left(&self) -> Self::Item;
}

impl GridLocation for Location {
    type Item = Self;

    fn up(&self) -> Self {
        (self.0, self.1 - 1)
    }

    fn right(&self) -> Self {
        (self.0 + 1, self.1)
    }

    fn down(&self) -> Self {
        (self.0, self.1 + 1)
    }

    fn left(&self) -> Self {
        (self.0 - 1, self.1)
    }
}

impl Map {
    pub fn from(input: &str) -> (Self, Location, Location) {
        let lines = input.split('\n');
        let mut width: Option<usize> = None;
        let mut height = 0;
        let mut start: Option<usize> = None;
        let mut end: Option<usize> = None;
        let mut grid = vec![];

        for line in lines {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            height += 1;
            if width.is_none() {
                width.replace(line.len());
            } else {
                assert_eq!(width.unwrap(), line.len());
            }
            for c in line.chars() {
                let elevation = match c {
                    'S' => {
                        if start.is_none() {
                            start.replace(grid.len());
                        } else {
                            panic!("Found a second 'S' grid reference");
                        }
                        b'a'
                    },
                    'E' => {
                        if end.is_none() {
                            end.replace(grid.len());
                        } else {
                            panic!("Fround a second 'E' grid reference");
                        }
                        b'z'
                    },
                    c => c as u8,
                };
                grid.push(elevation);
            }
        }

        let width = width.unwrap();
        let start = start.unwrap();
        let end = end.unwrap();

        let map = Self { grid, width, height, };
        let start = map.location(start);
        let end = map.location(end);

        (map, start, end)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn location(&self, index: usize) -> Location {
        let x = index % self.width;
        let y = index / self.width;

        (x as i64, y as i64)
    }

    pub fn neighbors(&self, location: &Location) -> Vec<Location> {
        let up = location.up();
        let right = location.right();
        let down = location.down();
        let left = location.left();

        let current = self.index(location);

        let valid = |l: &Location| {
            if (l.0 < 0 || l.1 < 0) || (l.0 >= self.width as i64 || l.1 >= self.height as i64) {
                false
            } else {
                let next = self.index(l);

                // neighbor can be at most 1 elevation higher in order to travel
                let current = self.grid[current];
                let next = self.grid[next];

                current + 1 >= next
            }
        };

        let mut neighbors = vec![];

        for l in [up, right, down, left] {
            if valid(&l) {
                neighbors.push(l);
            }
        }

        neighbors
    }

    fn index(&self, location: &Location) -> usize {
        assert!(location.0 > -1);
        assert!(location.1 > -1);
        location.0 as usize + location.1 as usize * self.width
    }

    pub fn all_start_locations(&self) -> Vec<Location> {
        self.grid.iter().enumerate()
            .filter(|(_, c)| **c as char == 'a')
            .map(|(i, _)| self.location(i))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from() {
        let input = "
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
        ";

        let (map, start, end) = Map::from(input);

        assert_eq!(8, map.width);
        assert_eq!(5, map.height);
        assert_eq!((0,0), start);
        assert_eq!((5,2), end);
    }

    #[test]
    fn all_starts() {
        let input = "
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
        ";

        let (map, _, _) = Map::from(input);

        let all_starts = map.all_start_locations();

        assert_eq!(6, all_starts.len());
    }
}