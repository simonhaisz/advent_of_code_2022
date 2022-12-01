use crate::elf::Elf;

pub struct Expedition {
	elves: Vec<Elf>,
}

impl Expedition {
	pub fn new() -> Expedition {
		Expedition { elves: vec![] }
	}

	pub fn add_elf(&mut self, elf: Elf) {
		self.elves.push(elf);
	}

	#[allow(dead_code)]
	pub fn find_snack_source(&self) -> Option<(usize, u64)> {
		let mut max_snacks_source = None;

		for (index, elf) in self.elves.iter().enumerate() {
			let calories = elf.calories();
			match max_snacks_source {
				None => max_snacks_source = Some((index, calories)),
				Some((_, c)) => {
					if calories > c {
						max_snacks_source = Some((index, calories));
					}
				},
			}
		}

		max_snacks_source
	}

	pub fn find_top_snack_sources(&self, top_elf_count: usize) -> u64 {
		if top_elf_count > self.elves.len() {
			panic!("{} is too many top snack sources! There are only {} elves in the expedition", top_elf_count, self.elves.len());
		}
		let mut sorted_elves = self.elves.to_vec();
		sorted_elves.sort_by(|a, b| b.calories().partial_cmp(&a.calories()).unwrap());

		let top_elves = &sorted_elves[0..top_elf_count];

		top_elves.iter()
			.map(|e| e.calories())
			.sum()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn empty() {
		let expedition = Expedition::new();

		assert_eq!(None, expedition.find_snack_source());
	}

	#[test]
	fn elf() {
		let mut elf = Elf::new();
		elf.add_snack(1337);

		let mut expedition = Expedition::new();
		expedition.add_elf(elf);

		assert_eq!(Some((0, 1337)), expedition.find_snack_source());
	}

	#[test]
	fn elves() {
		let mut galadriel = Elf::new();
		galadriel.add_snack(1);
		galadriel.add_snack(10);
		galadriel.add_snack(100);
		galadriel.add_snack(1000);

		let mut arwen = Elf::new();
		arwen.add_snack(333);

		let mut legolas = Elf::new();
		legolas.add_snack(1337);

		let mut elrond = Elf::new();
		elrond.add_snack(1);

		let mut expedition = Expedition::new();
		expedition.add_elf(galadriel);
		expedition.add_elf(arwen);
		expedition.add_elf(legolas);
		expedition.add_elf(elrond);

		assert_eq!(Some((2, 1337)), expedition.find_snack_source());
	}
}