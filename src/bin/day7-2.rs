use std::{io::{self, BufRead}};

#[derive(Debug, Eq, PartialEq)]
enum Manifold {
	Seen(usize, usize),
	NotSeen(usize),
}

fn main() {
	let lines = io::stdin().lock().lines().into_iter().map(|line| line.expect("Could not read input"));
	let mut map = parse_lines(lines);
	if let Manifold::NotSeen(col) = map[0][0] {
		println!("{}", recurse(col, &mut map[1..]));
	}
}

fn recurse(col: usize, map: &mut [Vec<Manifold>]) -> usize {
	#[allow(irrefutable_let_patterns)]
	if map.len() > 0 && let (row, map) = map.split_at_mut(1) {
		if let Some(manifold) = row[0].iter_mut().find(|v| {
			match **v {
				Manifold::Seen(v, _) => v == col,
				Manifold::NotSeen(v) => v == col,
			}
		}) {
			match manifold {
				Manifold::Seen(_, ct) => *ct,
				Manifold::NotSeen(_) => {
					let ct = recurse(col - 1, map) + recurse(col + 1, map);
					*manifold = Manifold::Seen(col, ct);
					ct
				}
			}
		} else {
			recurse(col, &mut map[0..])
		}
	} else {
		1
	}
}

fn parse_lines(lines: impl Iterator<Item = String>) -> Vec<Vec<Manifold>> {
	lines
		.map(|line| {
			line.chars()
				.enumerate()
				.filter_map(|(col, c)| {
					if c == '^' || c == 'S' {
						Some(Manifold::NotSeen(col))
					} else {
						None
					}
				})
				.collect()
		})
		.filter(|line: &Vec<Manifold>| !line.is_empty())
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn parse_lines_test() {
		assert_eq!(
			vec![
				vec![Manifold::NotSeen(7)],
				vec![Manifold::NotSeen(7)],
				vec![Manifold::NotSeen(6), Manifold::NotSeen(8)],
				vec![Manifold::NotSeen(5), Manifold::NotSeen(7), Manifold::NotSeen(9)],
				vec![Manifold::NotSeen(4), Manifold::NotSeen(6), Manifold::NotSeen(10)],
				vec![Manifold::NotSeen(3), Manifold::NotSeen(5), Manifold::NotSeen(9), Manifold::NotSeen(11)],
				vec![Manifold::NotSeen(2), Manifold::NotSeen(6), Manifold::NotSeen(12)],
				vec![Manifold::NotSeen(1), Manifold::NotSeen(3), Manifold::NotSeen(5), Manifold::NotSeen(7), Manifold::NotSeen(9), Manifold::NotSeen(13)]
			],
			parse_lines(
				"\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
					.lines().map(|s| s.to_string())
			)
		);
	}

	#[test]
	fn recurse_test() {
		let mut map = vec![
			vec![Manifold::NotSeen(7)],
			vec![Manifold::NotSeen(6), Manifold::NotSeen(8)],
			vec![Manifold::NotSeen(5), Manifold::NotSeen(7), Manifold::NotSeen(9)],
			vec![Manifold::NotSeen(4), Manifold::NotSeen(6), Manifold::NotSeen(10)],
			vec![Manifold::NotSeen(3), Manifold::NotSeen(5), Manifold::NotSeen(9), Manifold::NotSeen(11)],
			vec![Manifold::NotSeen(2), Manifold::NotSeen(6), Manifold::NotSeen(12)],
			vec![Manifold::NotSeen(1), Manifold::NotSeen(3), Manifold::NotSeen(5), Manifold::NotSeen(7), Manifold::NotSeen(9), Manifold::NotSeen(13)]
		];
		let ret = recurse(7, &mut map);
		assert_eq!(40, ret);
	}
}
