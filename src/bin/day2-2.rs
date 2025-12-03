use std::io::{self, Read};

fn main() {
	let mut input = String::new();
	let _ = io::stdin().lock().read_to_string(&mut input);
	let mut ret: usize = 0;
	for input in input.split(',') {
		let (lower, upper) = split_range(&input);
		ret += (lower..=upper).filter(|v| is_pattern(*v)).sum::<usize>();
	}
	println!("{}", ret);
}
fn split_range(range: &str) -> (usize, usize) {
	let (left, right) = range.split_at(range.find('-').expect("Could not find '-'"));
	let right = &right[1..].trim();
	(
		left.parse::<usize>().expect("Could not parse left bound"),
		right.parse::<usize>().expect("Could not parse right bound"),
	)
}
fn is_pattern(id: usize) -> bool {
	let id_str = id.to_string();
	for pattern_len in 1..=id_str.len() / 2 {
		if id_str.len() % pattern_len == 0 {
			let pattern = id_str[0..pattern_len].repeat(id_str.len() / pattern_len);
			if pattern == id_str {
				return true;
			}
		}
	}
	false
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn split_range_test() {
		assert!(matches!(split_range("11-22"), (11, 22)));
		assert!(matches!(split_range("95-115"), (95, 115)));
		assert!(matches!(split_range("998-1012"), (998, 1012)));
		assert!(matches!(
			split_range("1188511880-1188511890"),
			(1188511880, 1188511890)
		));
		assert!(matches!(split_range("222220-222224"), (222220, 222224)));
		assert!(matches!(split_range("1698522-1698528"), (1698522, 1698528)));
		assert!(matches!(split_range("446443-446449"), (446443, 446449)));
		assert!(matches!(
			split_range("38593856-38593862"),
			(38593856, 38593862)
		));
		assert!(matches!(split_range("565653-565659"), (565653, 565659)));
		assert!(matches!(
			split_range("824824821-824824827"),
			(824824821, 824824827)
		));
		assert!(matches!(
			split_range("2121212118-2121212124"),
			(2121212118, 2121212124)
		));
	}

	#[test]
	fn is_pattern_test() {
		for i in 11..=22 {
			if i == 11 || i == 22 {
				assert!(is_pattern(i));
			} else {
				assert!(!is_pattern(i));
			}
		}

		for i in 95..=115 {
			if i == 99 || i == 111 {
				assert!(is_pattern(i));
			} else {
				assert!(!is_pattern(i));
			}
		}

		for i in 998..=1012 {
			if i == 1010 || i == 999 {
				assert!(is_pattern(i));
			} else {
				assert!(!is_pattern(i));
			}
		}

		for i in 1188511880..=1188511890 {
			if i == 1188511885 {
				assert!(is_pattern(i));
			} else {
				assert!(!is_pattern(i));
			}
		}

		for i in 222220..=222224 {
			if i == 222222 {
				assert!(is_pattern(i));
			} else {
				assert!(!is_pattern(i));
			}
		}

		for i in 1698522..=1698528 {
			assert!(!is_pattern(i));
		}

		for i in 446443..=446449 {
			if i == 446446 {
				assert!(is_pattern(i));
			} else {
				assert!(!is_pattern(i));
			}
		}

		for i in 38593856..=38593862 {
			if i == 38593859 {
				assert!(is_pattern(i));
			} else {
				assert!(!is_pattern(i));
			}
		}

		for i in 565653..=565659 {
			if i == 565656 {
				assert!(is_pattern(i));
			} else {
				assert!(!is_pattern(i));
			}
		}

		for i in 824824821..=824824827 {
			if i == 824824824 {
				assert!(is_pattern(i));
			} else {
				assert!(!is_pattern(i));
			}
		}

		for i in 2121212118..=2121212124 {
			if i == 2121212121 {
				assert!(is_pattern(i));
			} else {
				assert!(!is_pattern(i));
			}
		}
	}
}
