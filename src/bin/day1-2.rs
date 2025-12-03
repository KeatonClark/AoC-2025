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
		#[allow(unused_assignments)]
		let mut count = 0;
		(dial_pos, count) = move_dial(
			parse_input(&input.expect("Could not read from stdin")).expect("Could not parse input"),
			dial_pos,
		);
		ret += count;
	}
	println!("{}", ret);
}

fn parse_input(input: &str) -> Result<Direction, &str> {
	let input = input.trim();
	match input {
		l if input.starts_with("L") => {
			Ok(Direction::Left(l[1..].parse::<usize>().map_err(|_| input)?))
		}
		r if input.starts_with("R") => Ok(Direction::Right(
			r[1..].parse::<usize>().map_err(|_| input)?,
		)),
		_ => Err(input),
	}
}

fn move_dial(dir: Direction, pos: usize) -> (usize, usize) {
	match dir {
		Direction::Left(count) => {
			let new = pos as i64 - count as i64;
			let out = new.rem_euclid(100) as usize;
			(
				out,
				new.div_euclid(100).abs() as usize - if pos == 0 { 1 } else { 0 }
					+ if out == 0 { 1 } else { 0 },
			)
		}
		Direction::Right(count) => {
			let new = pos + count;
			(new.rem_euclid(100), new.div_euclid(100))
		}
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
		assert_eq!(move_dial(Direction::Left(68), 50), (82, 1));
		assert_eq!(move_dial(Direction::Left(30), 82), (52, 0));
		assert_eq!(move_dial(Direction::Right(48), 52), (0, 1));
		assert_eq!(move_dial(Direction::Left(5), 0), (95, 0));
		assert_eq!(move_dial(Direction::Right(60), 95), (55, 1));
		assert_eq!(move_dial(Direction::Left(55), 55), (0, 1));
		assert_eq!(move_dial(Direction::Left(1), 0), (99, 0));
		assert_eq!(move_dial(Direction::Left(99), 99), (0, 1));
		assert_eq!(move_dial(Direction::Right(14), 0), (14, 0));
		assert_eq!(move_dial(Direction::Left(82), 14), (32, 1));
	}
}
