use crate::get_input;
use anyhow::Result;

pub fn run() -> Result<()> {
	let puzzle_input = get_input(5, 1)?;
	println!("part one: {}", part_one(&puzzle_input));
	// println!("part two: {}", part_two(&puzzle_input));
	Ok(())
}

fn part_one(input: &str) -> String {
	let (stacks, instructions) = parse_input(input);
	todo!()
}

// fn part_two(input: &str) -> i32 {
// 	find_pairs(input, overlaps)
// }

fn parse_input(input: &str) -> (Stacks, Moves) {
	// The two sections are separated by a blank line
	let (stack_input, move_input) = input.split_once('\n').unwrap();
	let stacks = parse_stacks(stack_input);
	let moves = parse_moves(move_input);
	(stacks, moves)
}

fn parse_stacks(input: &str) -> Stacks {
	let mut stacks = vec![];
	for line in input.lines() {
		let mut chars = line.chars();
		// Entries come in chunks of 4 characters.
		let mut idx = 0;
		loop {
			// [
			let open = chars.next().unwrap();
			// If the first char is a number, we're done.
			if open.is_numeric() {
				break;
			}
			// content
			let c = chars.next().unwrap();
			dbg!(&c);
			if stacks.get(idx).is_none() {
				stacks.push(vec![c]);
			} else {
				stacks[idx].push(c);
			}
			// ]
			let _ = chars.next().unwrap();
			// space or newline
			let terminator = chars.next();
			dbg!(&terminator);
			if terminator != Some(' ') {
				continue;
			}
			idx += 1;
		}
	}
	Stacks { stacks }
}

fn parse_moves(input: &str) -> Moves {
	let mut moves = vec![];
	Moves { moves }
}

#[derive(Debug)]
struct Stacks {
	stacks: Vec<Vec<char>>,
}

struct Moves {
	moves: Vec<Move>,
}

struct Move {
	amount: usize,
	from: usize,
	to: usize,
}

#[cfg(test)]
mod test {
	use super::*;
	use indoc::indoc;
	use pretty_assertions::assert_eq;
	#[test]
	fn test_part_one() {
		let test_input = indoc! {"
			    [D]
			[N] [C]
			[Z] [M] [P]
			1   2   3

			move 1 from 2 to 1
			move 3 from 1 to 3
			move 2 from 2 to 1
			move 1 from 1 to 2
		"};
		let (stacks, _) = parse_input(&test_input);
		dbg!(stacks);
		// assert_eq!(part_one(test_input), 2);
	}
	#[test]
	fn test_part_two() {
		let test_input = indoc! {"
		"};
	}
}
