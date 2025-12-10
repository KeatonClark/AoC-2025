use std::io::{self, BufRead};

#[derive(Debug, Copy, Clone)]
struct Point {
	x: i64,
	y: i64,
}

impl Point {
	fn new(x: i64, y: i64) -> Self { Self { x, y } }
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

#[derive(Debug, Copy, Clone)]
struct Edge(Point, Point);
impl Edge {
	fn from_points(a: Point, b: Point) -> Self {
		Self(a, b)
	}
	fn intersects(&self, other: &Edge) -> bool {
		let a = (self.1.x - self.0.x) * (other.0.y - self.0.y)
			- (self.1.y - self.0.y) * (other.0.x - self.0.x);
		let b = (self.1.x - self.0.x) * (other.1.y - self.0.y)
			- (self.1.y - self.0.y) * (other.1.x - self.0.x);
		let c = (other.1.x - other.0.x) * (self.0.y - other.0.y)
			- (other.1.y - other.0.y) * (self.0.x - other.0.x);
		let d = (other.1.x - other.0.x) * (self.1.y - other.0.y)
			- (other.1.y - other.0.y) * (self.1.x - other.0.x);
		((a > 0 && b < 0) || (a < 0 && b > 0)) && ((c > 0 && d < 0) || (c < 0 && d > 0))
	}
	fn contains_point(&self, point: &Point) -> bool {
		if (self.0.x.min(self.1.x) <= point.x && point.x <= self.0.x.max(self.1.x)) &&
			(self.0.y.min(self.1.y) <= point.y && point.y <= self.0.y.max(self.1.y)) {
			return 0 == (point.x - self.0.x) * (self.1.y - self.0.y) - (point.y - self.0.y) * (self.1.x - self.0.x);
		}
		false
	 }
}

#[derive(Debug)]
struct Polygon(Vec<Point>, Vec<Edge>);

impl Polygon {
	fn from_points(points: Vec<Point>) -> Self {
		let mut edges = Vec::new();
		for i in 0..points.len() {
			edges.push(Edge::from_points(points[i], points[(i+1) % points.len()]));
		}
		Self(points, edges)
	}
	fn contains_point(&self, point: &Point) -> bool {
		let mut ret = false;
		for i in 0..self.0.len() {
			let pa = self.0[i];
			let pb = self.0[(i + self.0.len() - 1) % self.0.len()];
			if Edge::from_points(pa, pb).contains_point(&point) {
				return true;
			}
			if (pa.y > point.y) != (pb.y > point.y) {
				if point.x < (pb.x - pa.x) * (point.y - pa.y) / (pb.y - pa.y) + pa.x {
					ret = !ret;
				}
			}
		}
		ret
	}
	fn edges(&self) -> impl Iterator<Item = &Edge> {
		self.1.iter()
	}
	fn points(&self) -> impl Iterator<Item = &Point> {
		self.0.iter()
	}
	fn intersects(&self, polygon: &Polygon) -> bool {
		polygon.edges().any(|inner_edge| self.edges().any(|outer_edge| {
			outer_edge.intersects(inner_edge)
		}))
	}
	fn contains_poly(&self, polygon: &Polygon) -> bool {
		if self.intersects(polygon) {
			return false;
		}
		polygon.points().all(|point| self.contains_point(point))
	}
}

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
	let outer = Polygon::from_points(points.clone());
	let mut max = 0;
	for i in 0..points.len() {
		for j in i + 1..points.len() {
			let rect = Polygon::from_points(vec![
				Point::new(points[i].x.min(points[j].x), points[i].y.min(points[j].y)),
				Point::new(points[i].x.max(points[j].x), points[i].y.min(points[j].y)),
				Point::new(points[i].x.min(points[j].x), points[i].y.max(points[j].y)),
				Point::new(points[i].x.max(points[j].x), points[i].y.max(points[j].y)),
			]);
			if !outer.contains_poly(&rect) {
				continue;
			}
			max = max.max(
				((points[i].x - points[j].x).abs() + 1) *
				((points[i].y - points[j].y).abs() + 1));
			
		}
	}
	println!("{}", max);
}
#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn contains_test() {
		let outer = Polygon::from_points(vec![
			Point::new(7,1),
			Point::new(11,1),
			Point::new(11,7),
			Point::new(9,7),
			Point::new(9,5),
			Point::new(2,5),
			Point::new(2,3),
			Point::new(7,3)
		]);
		assert!(outer.contains_poly(&Polygon::from_points(vec![
					Point::new(9,5),
					Point::new(2,5),
					Point::new(2,3),
					Point::new(9,3),
		])));
		assert!(outer.contains_poly(&Polygon::from_points(vec![
					Point::new(9,7),
					Point::new(9,5),
					Point::new(9,7),
					Point::new(9,5),
		])));
		assert!(outer.contains_poly(&Polygon::from_points(vec![
					Point::new(11,1),
					Point::new(11,3),
					Point::new(7,3),
					Point::new(7,1),
		])));
		assert!(!outer.contains_poly(&Polygon::from_points(vec![
					Point::new(2,5),
					Point::new(2,1),
					Point::new(11,1),
					Point::new(11,5),
		])));

	}
}
