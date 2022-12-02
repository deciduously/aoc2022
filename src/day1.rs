use crate::get_input;
use anyhow::Result;
use std::collections::BinaryHeap;

pub fn run() -> Result<()> {
	// both parts use the same input
	let puzzle_input = read_nums_chunked(&get_input(1, 1)?)?;
	println!("part 1: {}", part_one(&puzzle_input));
	println!("part 2: {}", part_two(&puzzle_input));
	Ok(())
}

fn read_nums_chunked(input: &str) -> Result<Vec<Vec<i32>>> {
	let mut ret = Vec::new();
	let mut current_chunk = Vec::new();
	for line in input.lines() {
		let s = line;
		if s.is_empty() {
			ret.push(current_chunk.clone());
			current_chunk.clear();
		} else {
			current_chunk.push(s.parse()?);
		}
	}
	ret.push(current_chunk);
	Ok(ret)
}

fn part_one(chunks: &[Vec<i32>]) -> i32 {
	sums(chunks).pop().unwrap()
}

fn part_two(chunks: &[Vec<i32>]) -> i32 {
	let mut sums = sums(chunks);
	sums.pop().unwrap() + sums.pop().unwrap() + sums.pop().unwrap()
}

fn sums(chunks: &[Vec<i32>]) -> BinaryHeap<i32> {
	let mut heap = BinaryHeap::new();
	for chunk in chunks {
		heap.push(chunk.iter().sum());
	}
	heap
}

#[cfg(test)]
mod test {
	use super::*;
	use indoc::indoc;
	use pretty_assertions::assert_eq;
	#[test]
	fn test_part_one() {
		let test_input = indoc! {"
			1000
			2000
			3000

			4000

			5000
			6000

			7000
			8000
			9000

			10000
		"};
		let nums_groups = read_nums_chunked(test_input).unwrap();
		assert_eq!(part_one(&nums_groups), 24000);
	}
	#[test]
	fn test_part_two() {
		let test_input = indoc! {"
			1000
			2000
			3000

			4000

			5000
			6000

			7000
			8000
			9000

			10000
		"};
		let nums_groups = read_nums_chunked(test_input).unwrap();
		assert_eq!(part_two(&nums_groups), 45000);
	}
}
