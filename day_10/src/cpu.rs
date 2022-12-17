
pub struct Cpu {
	x: i64,
	cycles_completed: u64,
}

impl Cpu {
	pub fn new() -> Self {
		Self { x: 1, cycles_completed: 0 }
	}

	pub fn noop(&mut self) {
		self.cycles_completed += 1;
	}

	pub fn add(&mut self, n: i64) {
		self.x += n;
		self.cycles_completed += 2;
	}

	pub fn x(&self) -> i64 {
		self.x
	}
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu::new()
    }
}

pub enum Operation {
	Noop,
	AddX(i64)
}

impl Operation {
	pub fn from(line: &str) -> Self {
		if line == "noop" {
			Operation::Noop
		} else if line.starts_with("addx") {
			let mut pair = line.split(' ');
			pair.next();
			let n = pair.next().unwrap().parse::<i64>().unwrap();
			Operation::AddX(n)
		} else {
			panic!("")
		}
	}

	pub fn series_from(input: &str) -> Vec<Self> {
		let mut ops = vec![];
		for line in input.split('\n') {
			if line.is_empty() {
				continue;
			}
			let op = Operation::from(line);
			ops.push(op);
		}
		ops
	}
}

pub struct Program {
	cpu: Cpu,
	cycle_register: Vec<(u64, i64)>,
}

impl Program {
	pub fn new() -> Self {
		Self { cpu: Cpu::new(), cycle_register: vec![] }
	}

	pub fn run(&mut self, op: Operation) {
		match op {
			Operation::Noop => self.cpu.noop(),
			Operation::AddX(n) => self.cpu.add(n),
		}
		
		self.cycle_register.push((self.cpu.cycles_completed, self.cpu.x()));
	}

	pub fn signal_strength(&self, target_cycle: u64) -> i64 {
		let mut current_strength = 0;
		for (cycle, x) in self.cycle_register.iter() {
			if *cycle < target_cycle {
				current_strength = target_cycle as i64 * *x;
			} else {
				break;
			}
		}

		current_strength
	}

	pub fn signal_strength_sum(&self, target_cycles: &[u64]) -> i64 {
		target_cycles.iter()
			.map(|c| self.signal_strength(*c))
			.sum()
	}

	pub fn register_as_cycle(&self, target_cycle: u64) -> i64 {
		let mut current_value = 0;
		for (cycle, x) in self.cycle_register.iter() {
			if *cycle < target_cycle {
				current_value = *x;
			} else {
				break;
			}
		}

		current_value
	}

	pub fn draw_pixels(&self) -> String {
		let mut pixels = String::new();
		for c in 1..=240 {
			let x = self.register_as_cycle(c);
			let column = c as i64 % 40 - 1;
			let delta = x - column;
			if delta.abs() <= 1 {
				pixels += "#";
			} else {
				pixels += ".";
			}
			if c < 240 && c % 40 == 0 {
				pixels += "\n";
			}
		}

		pixels

	}
}

impl Default for Program {
	fn default() -> Self {
		Program::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sample() {
		let input = "
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";
		let mut program = Program::new();

		let ops = Operation::series_from(input);
		for op in ops {
			program.run(op);
		}
		
		assert_eq!(420, program.signal_strength(20));
		assert_eq!(1140, program.signal_strength(60));
		assert_eq!(1800, program.signal_strength(100));
		assert_eq!(2940, program.signal_strength(140));
		assert_eq!(2880, program.signal_strength(180));
		assert_eq!(3960, program.signal_strength(220));

		assert_eq!(13140, program.signal_strength_sum(&[20, 60, 100, 140, 180, 220]));

		let screen = program.draw_pixels();

		assert_eq!(
"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....",
			screen
		);
	}
}