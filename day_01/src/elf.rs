#[derive(Clone, Debug, Default)]
pub struct Elf {
	snacks: Vec<u64>,
}

impl Elf {
	pub fn new() -> Self {
		Elf {
			snacks: vec![],
		}
	}

	pub fn add_snack(&mut self, calories: u64) {
		self.snacks.push(calories);
	}

	pub fn calories(&self) -> u64 {
		self.snacks.iter().sum()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn empty() {
		let elf = Elf::new();

		assert_eq!(0, elf.calories());
	}

	#[test]
	fn single() {
		let mut elf = Elf::new();
		elf.add_snack(100);

		assert_eq!(100, elf.calories());
	}

	#[test]
	fn second_breakfast() {
		let mut elf = Elf::new();
		elf.add_snack(100);
		elf.add_snack(200);
		elf.add_snack(300);
		elf.add_snack(400);

		assert_eq!(1000, elf.calories());
	}
}
