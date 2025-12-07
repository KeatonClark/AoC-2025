use std::io::{self, BufRead};
#[derive(Debug, PartialEq, Eq)]
enum Token {
	Mac,
	Sum,
	Val(usize),
}
impl From<&str> for Token {
	fn from(value: &str) -> Self {
		match value {
			"+" => Self::Sum,
			"*" => Self::Mac,
			val => Self::Val(val.parse::<usize>().expect("Not a value")),
		}
	}
}

fn main() {
	let mut lines: Vec<Vec<Token>> = io::stdin()
		.lock()
		.lines()
		.map(|v| parse_line(&v.expect("Could not read line")))
		.collect();
	println!("{}", accumulate(&mut lines));
}

fn accumulate(toks: &mut Vec<Vec<Token>>) -> usize {
	let mut acc = vec![0; toks[0].len()];
	let ops = toks.pop().expect("Empty lines");
	for (idx, op) in ops.iter().enumerate() {
		match op {
			Token::Mac => {
				acc[idx] = 1;
				for i in 0..toks.len() {
					if let Token::Val(val) = toks[i][idx] {
						acc[idx] *= val;
					} else {
						panic!("Op in value line");
					}
				}
			}
			Token::Sum => {
				for i in 0..toks.len() {
					if let Token::Val(val) = toks[i][idx] {
						acc[idx] += val;
					} else {
						panic!("Op in value line");
					}
				}
			}
			_ => panic!("Value in ops line"),
		}
	}
	acc.iter().sum()
}

fn parse_line(line: &str) -> Vec<Token> {
	line.split_whitespace().map(|s| s.into()).collect()
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn parse_line_test() {
		assert_eq!(
			parse_line("123 328  51 64 "),
			vec![
				Token::Val(123),
				Token::Val(328),
				Token::Val(51),
				Token::Val(64)
			]
		);
		assert_eq!(
			parse_line(" 45 64  387 23 "),
			vec![
				Token::Val(45),
				Token::Val(64),
				Token::Val(387),
				Token::Val(23)
			]
		);
		assert_eq!(
			parse_line("  6 98  215 314"),
			vec![
				Token::Val(6),
				Token::Val(98),
				Token::Val(215),
				Token::Val(314)
			]
		);
		assert_eq!(
			parse_line("*   +   *   +  "),
			vec![Token::Mac, Token::Sum, Token::Mac, Token::Sum]
		);
	}

	#[test]
	fn accumulate_test() {
		assert_eq!(
			4277556,
			accumulate(&mut vec![
				vec![
					Token::Val(123),
					Token::Val(328),
					Token::Val(51),
					Token::Val(64),
				],
				vec![
					Token::Val(45),
					Token::Val(64),
					Token::Val(387),
					Token::Val(23),
				],
				vec![
					Token::Val(6),
					Token::Val(98),
					Token::Val(215),
					Token::Val(314),
				],
				vec![Token::Mac, Token::Sum, Token::Mac, Token::Sum],
			])
		);
	}
}
