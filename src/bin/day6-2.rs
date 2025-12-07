use std::io::{self, BufRead};
#[derive(PartialEq, Eq, Debug)]
enum Op {
	Mac,
	Sum,
}

impl TryFrom<char> for Op {
	type Error = ();
	fn try_from(value: char) -> Result<Self, Self::Error> {
		match value {
			'*' => Ok(Op::Mac),
			'+' => Ok(Op::Sum),
			_ => Err(()),
		}
	}
}

fn main() {
	let mut lines: Vec<Vec<char>> = io::stdin()
		.lock()
		.lines()
		.map(|v| v.expect("Could not read line").chars().collect())
		.collect();
	let ops = parse_ops(lines.pop().expect("No last line for ops"));
	let mut accumulates = Vec::new();
	for (i, op) in ops.iter().enumerate() {
		let mut acc = if op.0 == Op::Mac { 1 } else { 0 };
		for col in ops[i].1
			..ops
				.get(i + 1)
				.map(|(_, idx)| idx - 1)
				.unwrap_or(lines[0].len())
		{
			let col_val = parse_col(col, &lines);
			if op.0 == Op::Mac {
				acc *= col_val;
			} else {
				acc += col_val;
			}
		}
		accumulates.push(acc);
	}
	println!("{}", accumulates.iter().sum::<usize>());
}

fn parse_col(col: usize, lines: &Vec<Vec<char>>) -> usize {
	lines
		.iter()
		.map(|line| line[col])
		.collect::<String>()
		.trim()
		.parse::<usize>()
		.expect("Could not parse column")
}

// Returns operation and char index
fn parse_ops(ops: Vec<char>) -> Vec<(Op, usize)> {
	ops.iter()
		.enumerate()
		.filter_map(|(idx, op)| {
			if let Ok(op) = Op::try_from(*op) {
				Some((op, idx))
			} else {
				None
			}
		})
		.collect()
}
#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn parse_ops_test() {
		assert_eq!(
			parse_ops("*   +   *   + ".chars().collect()),
			vec![(Op::Mac, 0), (Op::Sum, 4), (Op::Mac, 8), (Op::Sum, 12)]
		);
	}

	#[test]
	fn parse_col_test() {
		let lines = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314"
			.lines()
			.map(|v| v.chars().collect())
			.collect();

		assert_eq!(parse_col(0, &lines), 1);
		assert_eq!(parse_col(1, &lines), 24);
		assert_eq!(parse_col(2, &lines), 356);

		assert_eq!(parse_col(4, &lines), 369);
		assert_eq!(parse_col(5, &lines), 248);
		assert_eq!(parse_col(6, &lines), 8);

		assert_eq!(parse_col(8, &lines), 32);
		assert_eq!(parse_col(9, &lines), 581);
		assert_eq!(parse_col(10, &lines), 175);

		assert_eq!(parse_col(12, &lines), 623);
		assert_eq!(parse_col(13, &lines), 431);
		assert_eq!(parse_col(14, &lines), 4);
	}
}
