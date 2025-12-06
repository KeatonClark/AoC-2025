use std::io::{self, BufRead};
fn main() {
	let mut ret = 0;
	for input in io::stdin().lock().lines() {
		let input = input.expect("Can't read input");
		let (v10, s10) = greedy_search(&input, true);
		let (v1, _) = greedy_search(s10, false);
		ret += (v10 * 10) + v1;
	}
	println!("{}", ret);
}

fn greedy_search(input: &str, not_last: bool) -> (u32, &str) {
	input
		.chars()
		.rev()
		.enumerate()
		.skip(if not_last { 1 } else { 0 })
		.max_by_key(|v| v.1)
		.map(|v| {
			(
				v.1.to_digit(10).expect("Not a digit"),
				&input[input.len() - v.0..],
			)
		})
		.expect("No max value")
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn greedy_search_test() {
		assert!(matches!(
			greedy_search("987654321111111", true),
			(9, "87654321111111")
		));
		assert!(matches!(
			greedy_search("87654321111111", false),
			(8, "7654321111111")
		));

		assert!(matches!(
			greedy_search("811111111111119", true),
			(8, "11111111111119")
		));
		assert!(matches!(greedy_search("11111111111119", false), (9, "")));

		assert!(matches!(greedy_search("234234234234278", true), (7, "8")));
		assert!(matches!(greedy_search("8", false), (8, "")));

		assert!(matches!(
			greedy_search("818181911112111", true),
			(9, "11112111")
		));
		assert!(matches!(greedy_search("11112111", false), (2, "111")));
	}
}
