use crate::get_input;
use anyhow::Result;
use std::collections::HashSet;

pub fn run() -> Result<()> {
	let puzzle_input = get_input(3, 1)?;
	println!("part one: {}", part_one(&puzzle_input));
	println!("part two: {}", part_two(&puzzle_input));
	Ok(())
}

fn part_one(input: &str) -> u32 {
	let mut total = 0;
	for line in input.lines() {
		let mut seen = HashSet::new();
		let len = line.len();
		let half = len / 2;
		let compartment_one = &line[0..half];
		let compartment_two = &line[half..len];
		for c in compartment_one.chars() {
			if compartment_two.contains(c) {
				if !seen.contains(&c) {
					total += priority(c);
				}
				seen.insert(c);
			}
		}
	}
	total
}

fn part_two(input: &str) -> u32 {
	let mut total = 0;
	let mut lines = input.lines();
	while let Some(one) = lines.next() {
		let mut seen = HashSet::new();
		let two = lines.next().unwrap();
		let three = lines.next().unwrap();
		for c in one.chars() {
			if !seen.contains(&c) && two.contains(c) && three.contains(c) {
				total += priority(c);
			}
			seen.insert(c);
		}
	}
	total
}

fn priority(c: char) -> u32 {
	assert!(c.is_ascii_alphabetic());
	let val = c as u8;
	let ret = if c.is_ascii_lowercase() {
		val - b'a' + 1
	} else {
		val - b'A' + 27
	};
	u32::from(ret)
}

#[cfg(test)]
mod test {
	use super::*;
	use indoc::indoc;
	use pretty_assertions::assert_eq;
	#[test]
	fn test_part_one() {
		let test_input = indoc! {"
			vJrwpWtwJgWrhcsFMMfFFhFp
			jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
			PmmdzqPrVvPwwTWBwg
			wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
			ttgJtRGJQctTZtZT
			CrZsJsPPZsGzwwsLwLmpwMDw
		"};
		assert_eq!(part_one(test_input), 157);
	}
	#[test]
	fn test_part_two() {
		let test_input = indoc! {"
			vJrwpWtwJgWrhcsFMMfFFhFp
			jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
			PmmdzqPrVvPwwTWBwg
			wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
			ttgJtRGJQctTZtZT
			CrZsJsPPZsGzwwsLwLmpwMDw
		"};
		assert_eq!(part_two(test_input), 70);
	}
}
