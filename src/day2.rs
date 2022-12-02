use crate::get_input;
use anyhow::Result;

pub fn run() -> Result<()> {
	let puzzle_input = get_input(2, 1)?;
	println!("part 1: {}", part_one(&puzzle_input));
	println!("part 2: {}", part_two(&puzzle_input));
	Ok(())
}

fn part_one(input: &str) -> i32 {
	let mut score = 0;
	for line in input.lines() {
		let round = line.split_once(' ').unwrap();
		let opponent = Move::from_opponent(round.0.chars().next().unwrap()).unwrap();
		let player = Move::from_player(round.1.chars().next().unwrap()).unwrap();
		score += score_round(opponent, player);
	}
	score
}

fn part_two(input: &str) -> i32 {
	let mut score = 0;
	for line in input.lines() {
		let round = line.split_once(' ').unwrap();
		let opponent = Move::from_opponent(round.0.chars().next().unwrap()).unwrap();
		let strategy = Outcome::from_strategy(round.1.chars().next().unwrap()).unwrap();
		let player = Move::for_strategy(opponent, strategy);
		score += score_round(opponent, player);
	}
	score
}

fn score_round(opponent: Move, player: Move) -> i32 {
	let bonus = Outcome::from_match(opponent, player).score();
	player.score() + bonus
}

#[derive(Clone, Copy)]
enum Outcome {
	Loss,
	Draw,
	Win,
}

impl Outcome {
	fn from_match(opponent: Move, player: Move) -> Self {
		use Move::{Paper, Rock, Scissors};
		use Outcome::{Draw, Loss, Win};
		match opponent {
			Rock => match player {
				Rock => Draw,
				Paper => Win,
				Scissors => Loss,
			},
			Paper => match player {
				Rock => Loss,
				Paper => Draw,
				Scissors => Win,
			},
			Scissors => match player {
				Rock => Win,
				Paper => Loss,
				Scissors => Draw,
			},
		}
	}

	fn from_strategy(c: char) -> Option<Self> {
		match c {
			'X' => Some(Outcome::Loss),
			'Y' => Some(Outcome::Draw),
			'Z' => Some(Outcome::Win),
			_ => None,
		}
	}

	fn score(self) -> i32 {
		match self {
			Outcome::Loss => 0,
			Outcome::Draw => 3,
			Outcome::Win => 6,
		}
	}
}

#[derive(Clone, Copy, PartialEq)]
enum Move {
	Rock,
	Paper,
	Scissors,
}

impl Move {
	fn for_strategy(opponent: Move, strategy: Outcome) -> Self {
		use Move::{Paper, Rock, Scissors};
		use Outcome::{Draw, Loss, Win};
		match opponent {
			Rock => match strategy {
				Loss => Scissors,
				Draw => Rock,
				Win => Paper,
			},
			Paper => match strategy {
				Loss => Rock,
				Draw => Paper,
				Win => Scissors,
			},
			Scissors => match strategy {
				Loss => Paper,
				Draw => Scissors,
				Win => Rock,
			},
		}
	}

	fn from_opponent(c: char) -> Option<Move> {
		match c {
			'A' => Some(Move::Rock),
			'B' => Some(Move::Paper),
			'C' => Some(Move::Scissors),
			_ => None,
		}
	}

	fn from_player(c: char) -> Option<Move> {
		match c {
			'X' => Some(Move::Rock),
			'Y' => Some(Move::Paper),
			'Z' => Some(Move::Scissors),
			_ => None,
		}
	}

	fn score(self) -> i32 {
		match self {
			Move::Rock => 1,
			Move::Paper => 2,
			Move::Scissors => 3,
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use indoc::indoc;
	use pretty_assertions::assert_eq;
	#[test]
	fn test_part_one() {
		let test_input = indoc! {"
			A Y
			B X
			C Z
		"};
		assert_eq!(part_one(test_input), 15);
	}

	#[test]
	fn test_part_two() {
		let test_input = indoc! {"
			A Y
			B X
			C Z
		"};
		assert_eq!(part_two(test_input), 12);
	}
}
