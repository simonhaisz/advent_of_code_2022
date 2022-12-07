use std::collections::{VecDeque, HashSet};

pub struct DataStream {
	data: String,
}

impl DataStream {
	pub fn new(d: &str) -> Self {
		DataStream { data: d.to_string() }
	}

	pub fn find_packet_marker(&self) -> Option<usize> {
		let mut data_slice = VecDeque::with_capacity(4);

		for (i, c) in self.data.chars().enumerate() {
			if data_slice.len() == 4 {
				data_slice.remove(0);
			}

			data_slice.push_back(c);

			if data_slice.len() == 4 {
				let unique_data = data_slice
					.iter()
					.collect::<HashSet<_>>();
				if unique_data.len() == 4 {
					return Some(i + 1);
				}
			}
		}

		None
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn samples() {
		let stream = DataStream::new("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
		assert_eq!(Some(7), stream.find_packet_marker());

		let stream = DataStream::new("bvwbjplbgvbhsrlpgdmjqwftvncz");
		assert_eq!(Some(5), stream.find_packet_marker());

		let stream = DataStream::new("nppdvjthqldpwncqszvftbrmjlhg");
		assert_eq!(Some(6), stream.find_packet_marker());

		let stream = DataStream::new("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
		assert_eq!(Some(10), stream.find_packet_marker());

		let stream = DataStream::new("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
		assert_eq!(Some(11), stream.find_packet_marker());
	}
}