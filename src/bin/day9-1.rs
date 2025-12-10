use std::io::{self, BufRead};

fn main() {
	let points: Vec<Point> = io::stdin()
		.lock()
		.lines()
		.into_iter()
		.map(|line| {
			Point::try_from(line.expect("Could not read line").as_str())
				.expect("Could not parse line")
		})
		.collect();
	let mut max = 0;
	for i in 0..points.len() {
		for j in i + 1..points.len() {
			max = max.max(
				(1 + (points[i].x - points[j].x).abs()) * (1 + (points[i].y - points[j].y).abs()),
			);
		}
	}
	println!("{}", max);
}
#[derive(Debug)]
struct Point {
	x: i64,
	y: i64,
}
impl TryFrom<&str> for Point {
	type Error = ();
	fn try_from(value: &str) -> Result<Self, Self::Error> {
		let mut iter = value.split(',');
		let x = iter.next().ok_or(())?.parse().map_err(|_| ())?;
		let y = iter.next().ok_or(())?.parse().map_err(|_| ())?;
		Ok(Self { x, y })
	}
}
