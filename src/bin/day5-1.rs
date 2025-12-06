use std::io::{self, Read};
fn main() {
	let mut buf = String::new();
	io::stdin()
		.lock()
		.read_to_string(&mut buf)
		.expect("Could not read input");
	let (ranges, ids) = buf.split_once("\n\n").expect("Could not find input split");
	let ranges = split_ranges(ranges);
	let ret = ids
		.lines()
		.filter(|id| {
			ranges
				.iter()
				.any(|range| range.contains(&id.parse::<usize>().expect("Could not parse id")))
		})
		.count();
	println!("{ret}");
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
}
