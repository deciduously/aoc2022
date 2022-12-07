#![warn(clippy::pedantic)]

use anyhow::Result;
use std::{io::prelude::*, path::PathBuf};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn get_input(day: u8, part: u8) -> Result<String> {
	let filename = format!("inputs/{}-{}.txt", day, part);
	let mut f = std::fs::File::open(PathBuf::from(filename))?;
	let mut contents = String::new();
	f.read_to_string(&mut contents)?;
	Ok(contents)
}

fn main() -> Result<()> {
	let args = std::env::args().collect::<Vec<String>>();
	if args.len() != 2 {
		eprintln!("usage: aoc2022 <DAY>");
		std::process::exit(1);
	}
	let day: i32 = args[1].parse()?;
	match day {
		1 => day1::run()?,
		2 => day2::run()?,
		3 => day3::run()?,
		4 => day4::run()?,
		5 => day5::run()?,
		_ => {
			eprintln!("Day not implemented");
			std::process::exit(1);
		}
	}
	Ok(())
}
