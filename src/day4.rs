use std::ops::RangeInclusive;

use crate::get_input;
use anyhow::Result;

pub fn run() -> Result<()> {
	let puzzle_input = get_input(4, 1)?;
	println!("part one: {}", part_one(&puzzle_input));
	println!("part two: {}", part_two(&puzzle_input));
	Ok(())
}

fn part_one(input: &str) -> i32 {
	find_pairs(input, encompasses)
}

fn part_two(input: &str) -> i32 {
	find_pairs(input, overlaps)
}

fn find_pairs<F>(input: &str, f: F) -> i32
where
	F: Fn(RangeInclusive<i32>, RangeInclusive<i32>) -> bool,
{
	let mut total = 0;
	for line in input.lines() {
		let (elf_one, elf_two) = line.split_once(',').unwrap();
		let elf_one = make_inclusive_range(elf_one);
		let elf_two = make_inclusive_range(elf_two);
		if f(elf_one, elf_two) {
			total += 1;
		}
	}
	total
}

fn make_inclusive_range(s: &str) -> RangeInclusive<i32> {
	let (start, end) = s.split_once('-').unwrap();
	RangeInclusive::new(start.parse().unwrap(), end.parse().unwrap())
}

fn encompasses(range_one: RangeInclusive<i32>, range_two: RangeInclusive<i32>) -> bool {
	range_one.start() <= range_two.start() && range_one.end() >= range_two.end()
		|| range_two.start() <= range_one.start() && range_two.end() >= range_one.end()
}

fn overlaps(range_one: RangeInclusive<i32>, range_two: RangeInclusive<i32>) -> bool {
	for x in range_one {
		if range_two.contains(&x) {
			return true;
		}
	}
	false
}

#[cfg(test)]
mod test {
	use super::*;
	use indoc::indoc;
	use pretty_assertions::assert_eq;
	#[test]
	fn test_part_one() {
		let test_input = indoc! {"
				2-4,6-8
				2-3,4-5
				5-7,7-9
				2-8,3-7
				6-6,4-6
				2-6,4-8
		"};
		assert_eq!(part_one(test_input), 2);
	}
	#[test]
	fn test_part_two() {
		let test_input = indoc! {"
				2-4,6-8
				2-3,4-5
				5-7,7-9
				2-8,3-7
				6-6,4-6
				2-6,4-8
		"};
		assert_eq!(part_two(test_input), 4);
	}
}
