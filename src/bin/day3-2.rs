use std::io::{self, BufRead};
fn main() {
	let mut ret = 0;
	for input in io::stdin().lock().lines() {
		let input = input.expect("Can't read input");
		ret += loop_search(&input);
	}
	println!("{}", ret);
}

fn greedy_search(input: &str, skip: usize) -> (u64, &str) {
	input
		.chars()
		.rev()
		.enumerate()
		.skip(skip)
		.max_by_key(|v| v.1)
		.map(|v| {
			(
				v.1.to_digit(10).expect("Not a digit") as u64,
				&input[input.len() - v.0..],
			)
		})
		.expect("No max value")
}
fn loop_search(mut input: &str) -> u64 {
	let mut ret = 0;
	#[allow(unused_assignments)]
	let mut v = 0;
	for i in (0..12).rev() {
		(v, input) = greedy_search(input, i);
		ret += v * 10_u64.pow(i as u32);
	}
	ret
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn greedy_search_test() {
		assert!(matches!(
			greedy_search("987654321111111", 1),
			(9, "87654321111111")
		));
		assert!(matches!(
			greedy_search("87654321111111", 0),
			(8, "7654321111111")
		));

		assert!(matches!(
			greedy_search("811111111111119", 1),
			(8, "11111111111119")
		));
		assert!(matches!(greedy_search("11111111111119", 0), (9, "")));

		assert!(matches!(greedy_search("234234234234278", 1), (7, "8")));
		assert!(matches!(greedy_search("8", 0), (8, "")));

		assert!(matches!(
			greedy_search("818181911112111", 1),
			(9, "11112111")
		));
		assert!(matches!(greedy_search("11112111", 0), (2, "111")));
	}
	#[test]
	fn loop_search_test() {
		assert_eq!(loop_search("987654321111111"), 987654321111);
		assert_eq!(loop_search("811111111111119"), 811111111119);
		assert_eq!(loop_search("234234234234278"), 434234234278);
		assert_eq!(loop_search("818181911112111"), 888911112111);
	}
}
