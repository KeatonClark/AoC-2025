use std::io::{self, Read};
fn main() {
	let mut buf = String::new();
	io::stdin()
		.lock()
		.read_to_string(&mut buf)
		.expect("Could not read input");
	let (ranges, _) = buf.split_once("\n\n").expect("Could not find input split");
	let ranges = split_ranges(ranges);
	let ranges = merge_ranges(ranges);
	let ret: usize = ranges.iter().map(|range| range.clone().count()).sum();
	println!("{ret}");
}

fn merge_ranges(
	mut ranges: Vec<std::ops::RangeInclusive<usize>>,
) -> Vec<std::ops::RangeInclusive<usize>> {
	ranges.sort_by_key(|r| *r.start());
	let mut current = ranges[0].clone();
	let mut merged = Vec::new();
	for range in ranges.iter().skip(1) {
		if range.start() <= current.end() {
			if range.end() > current.end() {
				current = *current.start()..=*range.end();
			}
		} else {
			merged.push(current);
			current = range.clone();
		}
	}
	merged.push(current);
	merged
}

fn split_ranges(ranges: &str) -> Vec<std::ops::RangeInclusive<usize>> {
	ranges
		.lines()
		.map(|v| {
			let (l, r) = v.split_once('-').expect("Could not split range");
			l.parse::<usize>().expect("Could not parse left bound")
				..=r.parse::<usize>().expect("Could not parse right bound")
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn split_ranges_test() {
		assert_eq!(
			vec![3..=5, 10..=14, 16..=20, 12..=18],
			split_ranges("3-5\n10-14\n16-20\n12-18\n")
		);
	}

	#[test]
	fn merge_ranges_test() {
		assert_eq!(
			merge_ranges(vec![3..=5, 10..=14, 16..=20, 12..=18]),
			vec![3..=5, 10..=20]
		);
	}
}
