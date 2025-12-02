use std::io::{self, BufRead};

#[derive(Debug)]
enum Direction {
	Right(usize),
	Left(usize),
}

fn main() {
	let mut dial_pos = 50;
	let mut ret = 0;
	for input in io::stdin().lock().lines() {
		dial_pos = move_dial(parse_input(&input.expect("Could not read from stdin")).expect("Could not parse input")
			, dial_pos);
		if dial_pos == 0 { ret += 1; }
	}
	println!("{}", ret);
}

fn parse_input(input: &str) -> Result<Direction, &str> {
	let input = input.trim();
	match input {
		l if input.starts_with("L") => Ok(Direction::Left(l[1..].parse::<usize>().map_err(|_| input)?)),
		r if input.starts_with("R") => Ok(Direction::Right(r[1..].parse::<usize>().map_err(|_| input)?)),
		_ => Err(input),
	}
}

fn move_dial(dir: Direction, pos: usize) -> usize {
	match dir {
		Direction::Left(count) => ((pos + 1000000) - count) % 100,
		Direction::Right(count) => (pos + count) % 100,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn parse_input_test() {
		assert!(matches!(parse_input("L68"), Ok(Direction::Left(68))));
		assert!(matches!(parse_input("L30"), Ok(Direction::Left(30))));
		assert!(matches!(parse_input("R48"), Ok(Direction::Right(48))));
		assert!(matches!(parse_input("L5"), Ok(Direction::Left(5))));
		assert!(matches!(parse_input("R60"), Ok(Direction::Right(60))));
		assert!(matches!(parse_input("L55"), Ok(Direction::Left(55))));
		assert!(matches!(parse_input("L1"), Ok(Direction::Left(1))));
		assert!(matches!(parse_input("L99"), Ok(Direction::Left(99))));
		assert!(matches!(parse_input("R14"), Ok(Direction::Right(14))));
		assert!(matches!(parse_input("L82"), Ok(Direction::Left(82))));
	}

	#[test]
	fn move_dial_test() {
		assert_eq!(move_dial(Direction::Left(68), 50), 82);
		assert_eq!(move_dial(Direction::Left(30), 82), 52);
		assert_eq!(move_dial(Direction::Right(48), 52), 0);
		assert_eq!(move_dial(Direction::Left(5), 0), 95);
		assert_eq!(move_dial(Direction::Right(60), 95), 55);
		assert_eq!(move_dial(Direction::Left(55), 55), 0);
		assert_eq!(move_dial(Direction::Left(1), 0), 99);
		assert_eq!(move_dial(Direction::Left(99), 99), 0);
		assert_eq!(move_dial(Direction::Right(14), 0), 14);
		assert_eq!(move_dial(Direction::Left(82), 14), 32);
	}
}
