use std::{
	cmp::Reverse,
	collections::{BinaryHeap, HashSet},
	io::{self, BufRead},
	ops::Sub,
};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Point {
	x: i64,
	y: i64,
	z: i64,
}

#[derive(PartialEq, Debug)]
struct Edge {
	edge: f64,
	points: (Point, Point),
}

impl Eq for Edge {}
impl From<(Point, Point)> for Edge {
	fn from(points: (Point, Point)) -> Self {
		Self {
			edge: points.0 - points.1,
			points,
		}
	}
}
impl Ord for Edge {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.edge.partial_cmp(&other.edge).unwrap()
	}
}
impl PartialOrd for Edge {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.edge.partial_cmp(&other.edge)
	}
}

// Point - Point = Euclidean distance
impl Sub for Point {
	type Output = f64;
	fn sub(self, rhs: Self) -> Self::Output {
		f64::sqrt(
			((self.x - rhs.x) as f64).powi(2)
				+ ((self.y - rhs.y) as f64).powi(2)
				+ ((self.z - rhs.z) as f64).powi(2),
		)
	}
}

impl TryFrom<&str> for Point {
	type Error = ();
	fn try_from(value: &str) -> Result<Self, Self::Error> {
		let mut iter = value.split(',');
		let x = iter.next().ok_or(())?.parse().map_err(|_| ())?;
		let y = iter.next().ok_or(())?.parse().map_err(|_| ())?;
		let z = iter.next().ok_or(())?.parse().map_err(|_| ())?;
		Ok(Self { x, y, z })
	}
}

fn parse_lines(lines: impl Iterator<Item = String>) -> (Vec<Point>, BinaryHeap<Reverse<Edge>>) {
	let points: Vec<Point> = lines
		.map(|line| Point::try_from(line.as_str()).expect("Could not read line"))
		.collect();
	let mut edges: BinaryHeap<Reverse<Edge>> = BinaryHeap::new();
	for i in 0..points.len() {
		for j in 1 + i..points.len() {
			edges.push(Reverse(Edge::from((points[i], points[j]))));
		}
	}
	(points, edges)
}

fn calculate(mut edges: BinaryHeap<Reverse<Edge>>, points: Vec<Point>) -> i64 {
	let mut groups: Vec<HashSet<Point>> = points
		.iter()
		.map(|v| {
			let mut ret = HashSet::new();
			ret.insert(*v);
			ret
		})
		.collect();
	loop {
		if let Some(Reverse(edge)) = edges.pop() {
			let (a, _) = groups
				.iter()
				.enumerate()
				.find(|v| v.1.contains(&edge.points.0))
				.expect("We lost one of the points");
			let (b, _) = groups
				.iter()
				.enumerate()
				.find(|v| v.1.contains(&edge.points.1))
				.expect("We lost one of the points");
			if a != b {
				// These two have not been grouped yet
				let merge = groups.remove(a.max(b));
				groups.get_mut(a.min(b)).unwrap().extend(merge.iter());
				if groups.len() == 1 {
					return edge.points.0.x * edge.points.1.x;
				}
			}
		} else {
			panic!("No more edges");
		}
	}
}

fn main() {
	let (points, edges) = parse_lines(
		io::stdin()
			.lock()
			.lines()
			.into_iter()
			.map(|line| line.expect("Could not read line")),
	);
	let ret = calculate(edges, points);
	println!("{}", ret);
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn calculate_test() {
		let (points, edges) = parse_lines(
			"\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
				.lines()
				.map(|v| v.to_string()),
		);
		let ret = calculate(edges, points);
		assert_eq!(ret, 25272);
	}
	#[test]
	fn parse_lines_test() {
		let (points, edges) = parse_lines(
			"\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
				.lines()
				.map(|v| v.to_string()),
		);
		assert_eq!(
			points,
			vec![
				Point {
					x: 162,
					y: 817,
					z: 812
				},
				Point {
					x: 57,
					y: 618,
					z: 57
				},
				Point {
					x: 906,
					y: 360,
					z: 560
				},
				Point {
					x: 592,
					y: 479,
					z: 940
				},
				Point {
					x: 352,
					y: 342,
					z: 300
				},
				Point {
					x: 466,
					y: 668,
					z: 158
				},
				Point {
					x: 542,
					y: 29,
					z: 236
				},
				Point {
					x: 431,
					y: 825,
					z: 988
				},
				Point {
					x: 739,
					y: 650,
					z: 466
				},
				Point {
					x: 52,
					y: 470,
					z: 668
				},
				Point {
					x: 216,
					y: 146,
					z: 977
				},
				Point {
					x: 819,
					y: 987,
					z: 18
				},
				Point {
					x: 117,
					y: 168,
					z: 530
				},
				Point {
					x: 805,
					y: 96,
					z: 715
				},
				Point {
					x: 346,
					y: 949,
					z: 466
				},
				Point {
					x: 970,
					y: 615,
					z: 88
				},
				Point {
					x: 941,
					y: 993,
					z: 340
				},
				Point {
					x: 862,
					y: 61,
					z: 35
				},
				Point {
					x: 984,
					y: 92,
					z: 344
				},
				Point {
					x: 425,
					y: 690,
					z: 689
				}
			]
		);

		assert_eq!(
			*edges.peek().expect("No edges"),
			Reverse(Edge {
				edge: 316.90219311326956,
				points: (
					Point {
						x: 162,
						y: 817,
						z: 812
					},
					Point {
						x: 425,
						y: 690,
						z: 689
					}
				)
			})
		);
	}
}
