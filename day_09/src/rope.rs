use std::collections::HashSet;

pub type Position = (i32, i32);

#[derive(Debug)]
pub struct Rope {
	knots: Vec<Position>,
	tail_trace: HashSet<Position>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Move {
	direction: Direction,
	steps: u8,
}

impl Move {
	pub fn from(line: &str) -> Self {
		let mut pair = line.split(' ');
		let dir = pair.next().unwrap();
		
		let direction = match dir {
			"U" => Direction::Up,
			"D"	=> Direction::Down,
			"R" => Direction::Right,
			"L" => Direction::Left,
			_ => panic!("")
		};
		let steps = pair.next().unwrap().parse::<u8>().unwrap();
		Self { direction, steps }
	}
}

pub trait Moveable {
	type Item;

	fn mv(&mut self, direction: &Direction);
}

impl Moveable for Position {
    type Item = Self;

    fn mv(&mut self, direction: &Direction) {
		match direction {
			Direction::Up => self.1 += 1,
			Direction::Down => self.1 -= 1,
			Direction::Left => self.0 -= 1,
			Direction::Right => self.0 += 1,
		}
    }
}

impl Rope {
	pub fn new(knot_count: usize) -> Self {
		let knots = vec![(0, 0); knot_count];
		let mut tail_trace = HashSet::new();
		tail_trace.insert((0, 0));
		Self {
			knots,
			tail_trace,
		}
	}

	pub fn head_mut(&mut self) -> &mut Position {
		self.knots.first_mut().unwrap()
	}

	pub fn tail(&self) -> &Position {
		self.knots.last().unwrap()
	}

	pub fn move_head(&mut self, mv: &Move) {
		for _ in 0..mv.steps {
			self.mv(&mv.direction);
			self.tail_trace.insert(*self.tail());
		}
	}

	pub fn tail_trace_count(&self) -> usize {
		self.tail_trace.len()
	}
}

impl Moveable for Rope {
    type Item = Self;

    fn mv(&mut self, direction: &Direction) {
		self.head_mut().mv(direction);

		for i in 0..self.knots.len() - 1 {
			let current = self.knots.get(i).unwrap();
			let next = self.knots.get(i + 1).unwrap();
			if let Some(delta) = position_delta(current, next) {
				for pull in pull(&delta) {
					self.knots.get_mut(i + 1).unwrap().mv(&pull);
				}
			}

		}
    }
}

fn position_delta(a: &Position, b: &Position) -> Option<Position> {
	let horizontal_delta = a.0 - b.0;
	let vertical_delta = a.1 - b.1;

	if horizontal_delta.abs() <= 1 && vertical_delta.abs() <= 1 {
		None
	} else {
		Some((horizontal_delta, vertical_delta))
	}
}

fn pull(delta: &Position) -> Vec<Direction> {
	let mut moves = vec![];

	let (horizontal_move, vertical_move) =
	if delta.0.abs() > 0 && delta.1.abs() > 0 && (delta.0.abs() > 1 || delta.1.abs() > 1) {
		(true, true)
	} else {
		(delta.0.abs() > 1, delta.1.abs() > 1)
	};

	if horizontal_move {
		if delta.0 > 0 {
			moves.push(Direction::Right);
		} else {
			moves.push(Direction::Left);
		}
	}
	if vertical_move {
		if delta.1 > 0 {
			moves.push(Direction::Up);
		} else {
			moves.push(Direction::Down);
		}
	}

	moves
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn position_deltas() {
		assert_eq!(None, position_delta(&(1, 1), &(1, 1)));
		assert_eq!(Some((5, 2)), position_delta(&(5, 2), &(0, 0)));
		assert_eq!(Some((-1, 6)), position_delta(&(2, 7), &(3, 1)));
	}

	#[test]
	fn pulls() {
		assert_eq!(Vec::<Direction>::new(), pull(&(0,0)));
		assert_eq!(Vec::<Direction>::new(), pull(&(1,0)));
		assert_eq!(Vec::<Direction>::new(), pull(&(0,-1)));
		assert_eq!(Vec::<Direction>::new(), pull(&(-1,1)));

		assert_eq!(vec![Direction::Right], pull(&(5,0)));
		assert_eq!(vec![Direction::Left], pull(&(-2,0)));
		assert_eq!(vec![Direction::Up], pull(&(0,3)));
		assert_eq!(vec![Direction::Down], pull(&(0,-2)));

		assert_eq!(vec![Direction::Right, Direction::Up], pull(&(5,2)));
		assert_eq!(vec![Direction::Right, Direction::Down], pull(&(5,-3)));
		assert_eq!(vec![Direction::Left, Direction::Down], pull(&(-3,-2)));
		assert_eq!(vec![Direction::Left, Direction::Up], pull(&(-3,5)));
	}

	#[test]
	fn sample_1_moves() {
		let mut rope = Rope::new(2);
		rope.move_head(&Move { direction: Direction::Right, steps: 4 });
		rope.move_head(&Move { direction: Direction::Up, steps: 4 });
		rope.move_head(&Move { direction: Direction::Left, steps: 3 });
		rope.move_head(&Move { direction: Direction::Down, steps: 1 });
		rope.move_head(&Move { direction: Direction::Right, steps: 4 });
		rope.move_head(&Move { direction: Direction::Down, steps: 1 });
		rope.move_head(&Move { direction: Direction::Left, steps: 5 });
		rope.move_head(&Move { direction: Direction::Right, steps: 2 });

		let mut expected_trace = HashSet::new();
		expected_trace.insert((0, 0));
		expected_trace.insert((1, 0));
		expected_trace.insert((2, 0));
		expected_trace.insert((3, 0));
		expected_trace.insert((4, 1));
		expected_trace.insert((1, 2));
		expected_trace.insert((2, 2));
		expected_trace.insert((3, 2));
		expected_trace.insert((4, 2));
		expected_trace.insert((3, 3));
		expected_trace.insert((4, 3));
		expected_trace.insert((2, 4));
		expected_trace.insert((3, 4));

		assert_eq!(expected_trace, rope.tail_trace);
	}

	#[test]
	fn sample_2_moves() {
		let mut rope = Rope::new(10);
		rope.move_head(&Move { direction: Direction::Right, steps: 5 });
		rope.move_head(&Move { direction: Direction::Up, steps: 8 });
		rope.move_head(&Move { direction: Direction::Left, steps: 8 });
		rope.move_head(&Move { direction: Direction::Down, steps: 3 });
		rope.move_head(&Move { direction: Direction::Right, steps: 17 });
		rope.move_head(&Move { direction: Direction::Down, steps: 10 });
		rope.move_head(&Move { direction: Direction::Left, steps: 25 });
		rope.move_head(&Move { direction: Direction::Up, steps: 20 });

		let mut expected_trace = HashSet::new();
		expected_trace.insert((0, 0));
		expected_trace.insert((1, 1));
		expected_trace.insert((2, 2));
		expected_trace.insert((1, 3));
		expected_trace.insert((2, 4));
		expected_trace.insert((3, 5));
		expected_trace.insert((4, 5));
		expected_trace.insert((5, 5));
		expected_trace.insert((6, 4));
		expected_trace.insert((7, 3));
		expected_trace.insert((8, 2));
		expected_trace.insert((9, 1));
		expected_trace.insert((10, 0));
		expected_trace.insert((9, -1));
		expected_trace.insert((8, -2));
		expected_trace.insert((7, -3));
		expected_trace.insert((6, -4));
		expected_trace.insert((5, -5));
		expected_trace.insert((4, -5));
		expected_trace.insert((3, -5));
		expected_trace.insert((2, -5));
		expected_trace.insert((1, -5));
		expected_trace.insert((0, -5));
		expected_trace.insert((-1, -5));
		expected_trace.insert((-2, -5));
		expected_trace.insert((-3, -4));
		expected_trace.insert((-4, -3));
		expected_trace.insert((-5, -2));
		expected_trace.insert((-6, -1));
		expected_trace.insert((-7, 0));
		expected_trace.insert((-8, 1));
		expected_trace.insert((-9, 2));
		expected_trace.insert((-10, 3));
		expected_trace.insert((-11, 4));
		expected_trace.insert((-11, 5));
		expected_trace.insert((-11, 6));

		assert_eq!(expected_trace, rope.tail_trace);
	}

	#[test]
	fn move_from() {
		let up_four = Move::from("U 4");
		assert_eq!(Move { direction: Direction::Up, steps: 4}, up_four);

		let down_d = Move::from("D 2");
		assert_eq!(Move { direction: Direction::Down, steps: 2}, down_d);

		let right_six = Move::from("R 6");
		assert_eq!(Move { direction: Direction::Right, steps: 6}, right_six);

		let left_one = Move::from("L 1");
		assert_eq!(Move { direction: Direction::Left, steps: 1}, left_one);
	}
}