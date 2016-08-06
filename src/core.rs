/* vim: set noexpandtab : */

use std::f64::INFINITY;
use std::ops::{Add,Sub,Mul,Div,Neg};
use std::clone::Clone;
use std::cmp::{Ord,Ordering,PartialOrd};
use std::fmt::{Debug,Display};
use std::fmt;
use std::str::FromStr;

extern crate num;
use num::rational::BigRational;
use num::ToPrimitive;

#[derive(Debug,Clone,PartialEq,PartialOrd)]
pub struct Point<N: Num> {
	pub x: N,
	pub y: N,
}

#[derive(Debug,Clone,PartialEq)]
pub struct Line<N: Num> {
	pub p1: Point<N>,
	pub p2: Point<N>
}

#[derive(Debug,Clone,PartialEq)]
pub struct Polygon<N: Num> {
	is_hole: bool,
	square: bool,
	area: f64,
	corners: Vec<(Line<N>, Line<N>)>,
	pub points: Vec<Point<N>>,
}

#[derive(Debug,Clone)]
pub struct Shape<N: Num> {
	pub polys: Vec<Polygon<N>>,
}

#[derive(Debug,Clone)]
pub struct Skeleton<N: Num> {
	pub lines: Vec<Line<N>>,
}

//This assumes l0 -> l1 is clockwise, and l0.p2==l1.p1
fn dot<N: Num>(l0: &Line<N>, l1: &Line<N>) -> N {
	let d1 = &l0.p2 - &l0.p1;
	let d2 = &l1.p2 - &l0.p2;

	d1.x*d2.y - d1.y*d2.x
}

pub fn is_convex<N: Num>(l0: &Line<N>, l1: &Line<N>) -> bool {
	dot(l0,l1) > N::zero()
}

pub fn p_distance<N: Num>(p1: &Point<N>, p2: &Point<N>) -> f64 {
	let d = p1 - p2;
	return (d.x.to_f64().powi(2) + d.y.to_f64().powi(2)).sqrt();
}

pub fn v_distance<N: Num>(p: &Point<N>) -> f64 {
	return (p.x.to_f64().powi(2) + p.y.to_f64().powi(2)).sqrt();
}


pub fn normalize_line<N:Num>(start: &Point<N>, dir: &Point<N>) -> Point<N> {
	let ratio = N::from_f64(1.0 / v_distance(dir));
	let scaled = dir.scale(ratio);
	start + &scaled
}

// l0.p2 and l1.p1 are the same since this is where the lines join
// l0 and l1 must be perpendicular
pub fn square_from_corner<N:Num>(line0: &Line<N>, line1: &Line<N>) -> Polygon<N> {
	let line0sw = Line{p1: line0.p2.clone(), p2: line0.p1.clone()};
	let line1sw = Line{p1: line1.p2.clone(), p2: line1.p1.clone()};
	let l0 = if line0.p1 == line1.p2 { &line0sw } else { line0 };
	let l1 = if line0.p1 == line1.p2 { &line1sw } else { line1 };
	if l0.p2 != l1.p1 {
		panic!("Lines must join {}, {}; {}, {}", l0.p1, l0.p2, l1.p1, l1.p2);
	}

	let p1 = normalize_line(&l0.p2, &(&l0.p1-&l0.p2));
	let p2 = normalize_line(&p1, &(&l1.p2-&l1.p1));
	let poly = Polygon::new(vec!(
		l0.p2.clone(),
		p1,
		p2,
		normalize_line(&l0.p2, &(&l1.p2-&l0.p2))));

	poly
}

impl<N: Num> Point<N> {
	pub fn to_f64(&self) -> Point<f64> {
		Point{x: self.x.to_f64(), y: self.y.to_f64()}
	}

	pub fn scale(&self, alpha: N) -> Point<N> {
		Point{x: self.x.clone() * alpha.clone(), y: self.y.clone() * alpha}
	}
}

impl<N: Num> Polygon<N> {
	pub fn new(points: Vec<Point<N>>) -> Polygon<N> {
		let (clockwise, area, square, corners) = orient_area(&points);
		Polygon{points: points, area: area, square: square, is_hole: clockwise, corners: corners}
	}

	pub fn is_hole(&self) -> bool {
		self.is_hole
	}

	pub fn square(&self) -> bool {
		self.square
	}

	pub fn area(&self) -> f64 {
		self.area
	}

	pub fn corners(&self) -> Vec<(Line<N>, Line<N>)> {
		self.corners.clone()
	}

	// Returns the longest edge of this polygon
	pub fn longest_edge(self) -> (Point<N>, Point<N>) {
		let mut max: f64 = p_distance(&self.points.last().unwrap(), &self.points[0]);
		let mut longest: (Point<N>, Point<N>) = (self.points.last().unwrap().clone(), self.points[0].clone());
		for edge in self.points.windows(2) {
			let distance = p_distance(&edge[0], &edge[1]);
			if distance > max {
				max = distance;
				longest = (Point{x: edge[0].x.clone(), y: edge[0].y.clone()}, Point{x: edge[1].x.clone(), y: edge[1].y.clone()});
			}
		}

		return longest;
	}

	// Return the first vertex where this polygon departs the unit square - the
	// vertex closest to 0,0 that lies on the unit square.
	//
	// The polygon must be convex (no holes)
	//
	// If some element of the polygon lies outside the unit square, we'll still
	// find the vertex closest to 0,0.
	//
	// If no vertex of the polygon lies on the unit square, return None.
	pub fn lowest_unit_vertex(self) -> Option<Point<f64>> {
		let xx = Line::new(Point{x: 0.0, y: 0.0}, Point{x: 1.0, y: 0.0});
		let yx = Line::new(Point{x: 1.0, y: 0.0}, Point{x: 1.0, y: 1.0});
		let xy = Line::new(Point{x: 1.0, y: 1.0}, Point{x: 0.0, y: 1.0});
		let yy = Line::new(Point{x: 0.0, y: 1.0}, Point{x: 0.0, y: 0.0});

		let mut candidates = Vec::new();

		// Search the axes in order of close-ness to 0,0
		for boundary in [xx, yy, yx, xy].iter() {
			// Build a list of points coincident to this axis
			for point in self.points.clone() {
				if boundary.coincident(point.to_f64()) {
					candidates.push(point.to_f64());
				}
			}

			// If we found any points, don't try any other axes
			if candidates.len() > 0 {
				break;
			}
		}

		// No verticies coincident with the unit square
		if candidates.len() == 0 {
			return None;
		}

		// Pick the closest point from the candidates
		let origin = Point{x: 0.0, y: 0.0};
		let mut min = candidates[0].clone();
		for point in candidates {
			if p_distance(&origin, &point.to_f64()) < p_distance(&origin, &min) {
				min = point;
			}
		}

		// I can't get this to work --blinken
		//return candidates.min_by_key(|point| p_distance(&origin, *point));

		Some(min)
	}
}

impl<N: Num> Shape<N> {
	pub fn new(polys: Vec<Polygon<N>>) -> Shape<N> {
		Shape{polys: polys}
	}

	pub fn area(self) -> f64 {
		let mut a = 0.0;
		for p in self.polys {
			let sgn = if p.is_hole() { -1.0 } else { 1.0 };
			a += sgn * p.area();
		}
		a
	}

	// Runs lowest_unit_vertex on the first polygon in this shape - see above
	pub fn lowest_unit_vertex(self) -> Option<Point<f64>> {
		self.polys[0].clone().lowest_unit_vertex()
	}
}

impl<N: Num> Line<N> {
	pub fn new(p1: Point<N>, p2: Point<N>) -> Line<N> {
		return Line{p1: p1, p2: p2};
	}

	// Returns the length of this line
	pub fn len(&self) -> f64 {
		return p_distance(&self.p1, &self.p2);
	}

	// True if point lies on this line
	pub fn coincident(&self, point: Point<N>) -> bool {
		return p_distance(&self.p1, &point) + p_distance(&point, &self.p2) == self.len();
	}

	// Returns a point along this line. 0 <= alpha <= 1, else you're extrapolating bro
	pub fn interpolate(&self, alpha: N) -> Point<N> {
		&self.p1 + &(&self.p2 - &self.p1).scale(alpha)
	}

	// Splits this line into two at the specified position along it.
	pub fn split(&self, alpha: N) -> (Line<N>, Line<N>) {
		let mid = self.interpolate(alpha);
		let l1 = Line::new(self.p1.clone(), mid.clone());
		let l2 = Line::new(mid, self.p2.clone());
		(l1, l2)
	}
}

impl<N: Num> Skeleton<N> {
	pub fn new(lines: Vec<Line<N>>) -> Skeleton<N> {
		return Skeleton{lines: lines};
	}

	pub fn clone(self) -> Skeleton<N> {
		return Skeleton{lines: self.lines.clone()};
	}

	pub fn push(self, line: Line<N>) -> Skeleton<N> {
		let mut lines: Vec<Line<N>> = self.lines.clone();
		lines.push(line);
		return Skeleton{lines: lines};
	}

	pub fn lines(self) -> Vec<Line<N>> {
		return self.lines.clone();
	}

	// Returns the number of lines composing this skeleton
	pub fn len(self) -> usize {
		return self.lines.len();
	}

	// Returns the longest edge in this skeleton
	pub fn longest_edge(self) -> Line<N> {
		let mut longest: Line<N> = self.lines[0].clone();
		for line in self.lines {
			if line.len() > longest.len() {
				longest = line;
			}
		}

		return longest.clone();
	}
}

pub fn angle<'a, N: Num>(p0: &'a Point<N>, p1: &'a Point<N>) -> f64 {
	let d = p1 - p0;
	return d.x.to_f64().atan2(d.y.to_f64());
}

fn half_tri_area<'a, N: Num>(p0: &'a Point<N>, p1: &'a Point<N>) -> N {
	(p1.x.clone() - p0.x.clone()) * (p1.y.clone() + p0.y.clone())
}

/* returns a tuple where the first element is true if the poly points are in clockwise order,
** and the second element is the area contained within. thx to:
** http://stackoverflow.com/questions/1165647/how-to-determine-if-a-list-of-polygon-points-are-in-clockwise-order */
fn orient_area<N: Num>(points: &Vec<Point<N>>) -> (bool, f64, bool, Vec<(Line<N>, Line<N>)>) {
	let mut corners: Vec<(Line<N>, Line<N>)> = Vec::new();
	let n = points.len();
	let mut square = n == 4;
	// first case
	let mut sum = half_tri_area(&points[n-1], &points[0]);
	let mut edge1 = (&points[n-1], &points[0]);
	let cornerangle = (angle(edge1.0, edge1.1) - angle(&points[n-2], &points[n-1])).abs();
	if cornerangle % 90.0_f64.to_radians() > 0.00001 {
		square = false;
	} else {
		corners.push((
			Line{p1: (*edge1.0).clone(), p2: (*edge1.1).clone()},
			Line{p1: points[n-2].clone(), p2: points[n-1].clone()}
		));
	}
	// rest of polygon
	for segment in points.windows(2) {
		let edge = (&segment[0], &segment[1]);
		let cornerangle = (angle(edge1.0, edge1.1) - angle(edge.0, edge.1)).abs();
		if cornerangle % 90.0_f64.to_radians() > 0.000001 {
			square = false;
		} else {
			corners.push((
				Line{p1: (*edge1.0).clone(), p2: (*edge1.1).clone()},
				Line{p1: (*edge.0).clone(), p2: (*edge.1).clone()}
			));
		}
		edge1 = edge;
		sum = sum + half_tri_area(&segment[0], &segment[1]);
	}
	let f = sum.to_f64();
	return (f >= 0.0, f.abs() / 2.0, square, corners)
}

/*pub fn mirror<N: Num>(shapes: &Vec<Polygon<N>>, axis: Line<N>) -> Vec<Polygon<N>> {
		let mut results: Vec<Polygon<N>> = Vec::new();
		for shape in shapes {
				let new_shape = (*shape).clone();
				results.push(new_shape);
		}
		results
}*/



pub trait SuperLegit {
	fn to_f64(&self) -> f64;
	fn from_f64(f64) -> Self;
	fn zero() -> Self;
	fn one() -> Self;
}

impl SuperLegit for i32 {
	fn to_f64(&self) -> f64 { *self as f64 }
	fn from_f64(f: f64) -> Self { f as i32 }
	fn zero() -> Self { 0 }
	fn one() -> Self { 1 }
}

impl SuperLegit for f64 {
	fn to_f64(&self) -> f64 { *self }
	fn from_f64(f: f64) -> Self { f }
	fn zero() -> Self { 0.0 }
	fn one() -> Self { 1.0 }
}

impl SuperLegit for BigRational {
	fn to_f64(&self) -> f64 {
		// BUG converts very large negatives to positive infinity
		self.numer().to_f64().unwrap_or(INFINITY) / self.denom().to_f64().unwrap_or(1.0)
	}

	fn from_f64(f: f64) -> Self { BigRational::from_float(f).unwrap() }

	fn zero() -> Self { num::zero::<BigRational>() }
	fn one() -> Self { num::one::<BigRational>() }
}

pub trait Num: Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> + Neg<Output=Self> + Sized + FromStr + Debug + Display + PartialOrd + PartialEq + Clone + SuperLegit {}
impl<N> Num for N where N: Add<Output=N> + Sub<Output=N> + Mul<Output=N> + Div<Output=N> + Neg<Output=N> + Sized + FromStr + Debug + Display + PartialOrd + PartialEq + Clone + SuperLegit {}

impl<N: Num> Add for Point<N> {
	type Output=Self;
	fn add(self, other: Point<N>) -> Self {
		Point{x: self.x + other.x, y: self.y + other.y}
	}
}

impl<'a, 'b, N: Num> Add<&'b Point<N>> for &'a Point<N> {
	type Output=Point<N>;
	fn add(self, other: &'b Point<N>) -> Point<N> {
		Point{x: self.x.clone() + other.x.clone(), y: self.y.clone() + other.y.clone()}
	}
}

impl<N: Num> Sub for Point<N> {
	type Output=Self;
	fn sub(self, other: Point<N>) -> Self {
		Point{x: self.x - other.x, y: self.y - other.y}
	}
}

impl<'a, 'b, N: Num> Sub<&'b Point<N>> for &'a Point<N> {
	type Output=Point<N>;
	fn sub(self, other: &'b Point<N>) -> Point<N> {
		Point{x: self.x.clone() - other.x.clone(), y: self.y.clone() - other.y.clone()}
	}
}

impl<N: Num> Display for Point<N> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{},{}", self.x, self.y)
	}
}

/* can't derive(Eq) because we support Point<f64> and f64 doesn't provide a total ordering
 * (╯°□°)╯ sᴎɐᴎ */
impl<N: Num> Eq for Point<N> {
	// apparently this is allowed to be empty. cool?
}

impl<N: Num> Ord for Point<N> {
	fn cmp(&self, other: &Point<N>) -> Ordering {
		if self < other { Ordering::Less }
		else if self > other { Ordering::Greater }
		else { Ordering::Equal }
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	extern crate num;
	use self::num::rational::BigRational;
	use num::Float;

	fn p(x: i32, y: i32) -> Point<i32> {
		Point{x: x, y: y}
	}

	fn p64(x: f64, y: f64) -> Point<f64> {
		Point{x: x, y: y}
	}

	#[test]
	fn square_from_corner_test(){
		let l1 = Line{ p1: p64(-1.0/2.0,1.0/2.0), p2: p64(0.0,0.0) };
		let l2 = Line{ p1: p64(0.0,0.0), p2: p64(1.0/2.0,1.0/2.0) };

		let poly = square_from_corner(&l1,&l2);

		println!("{:?}",poly);
	}

	#[test]
	fn test_is_convex_1(){
		let l1 = Line{ p1: p(1,1), p2: p(2,2) };
		let l2 = Line{ p1: p(2,2), p2: p(3,1) };

		assert!(is_convex(&l1,&l2)==false);
	}

	#[test]
	fn test_is_convex_2(){
		let l1 = Line{ p1: p(1,1), p2: p(2,2) };
		let l2 = Line{ p1: p(2,2), p2: p(3,4) };

		assert!(is_convex(&l1,&l2)==true);
	}

	#[test]
	fn test_ops() {
		assert_eq!(p(5, 7), p(2, 4) + p(3, 3));
		assert_eq!(p(-1, 1), p(2, 4) - p(3, 3));
	}

	#[test]
	fn test_angle() {
		assert_eq!(45.0.to_radians(), angle(&p(0, 0), &p(1, 1)));
		assert_eq!(-135.0.to_radians(), angle(&p(1, 1), &p(0, 0)));
	}

	#[test]
	fn test_clockwise() {
		assert!(!Polygon::new(vec!(p(0, 0), p(1, 0), p(1, 1), p(0, 1))).is_hole());
		assert!(Polygon::new(vec!(p(0, 0), p(0, 1), p(1, 1), p(1, 0))).is_hole());
		assert!(Polygon::new(vec!(p(1, 1), p(1, 2), p(2, 2), p(2, 1))).is_hole());
	}

	#[test]
	fn test_area() {
		assert_eq!(1.0, Polygon::new(vec!(p(0, 0), p(1, 0), p(1, 1), p(0, 1))).area());
		assert_eq!(1.0, Polygon::new(vec!(p(0, 0), p(0, 1), p(1, 1), p(1, 0))).area());
		let p22 = Polygon::new(vec!(p(0, 0), p(2, 0), p(2, 2), p(0, 2)));
		assert_eq!(4.0, p22.area());
		let p44 = Polygon::new(vec!(p(0, 0), p(4, 0), p(4, 4), p(0, 4)));
		let hole12 = Polygon::new(vec!(p(1, 1), p(1, 2), p(2, 2), p(2, 1)));
		assert!(hole12.is_hole());
		assert_eq!(15.0, Shape::new(vec!(p44, hole12)).area());
	}

	#[test]
	fn test_longest() {
		assert_eq!((p(1,0), p(2,2)), Polygon::new(vec!(p(0, 0), p(1, 0), p(2, 2), p(0, 1))).longest_edge());
		assert_eq!(2.0, Line::new(p(0, 0), p(2, 0)).len());
	}

	#[test]
	fn test_coincident() {
		assert!(Line::new(p(0,0), p(0,10)).coincident(p(0,5)));
		assert!(!Line::new(p(0,0), p(0,10)).coincident(p(1,5)));
		assert!(!Line::new(p(0,0), p(0,10)).coincident(p(0,11)));
	}

	#[test]
	fn test_float() {
		assert_eq!(0.5f64, "4328029871649615121465353437184/8656059743299229793415925725865".parse::<BigRational>().unwrap().to_f64());
		assert_eq!(0.25f64, "1/4".parse::<BigRational>().unwrap().to_f64());
		assert_eq!(1.1f64, "11/10".parse::<BigRational>().unwrap().to_f64());
	}

	#[test]
	fn test_lowest_unit_vertex() {
		let a = Polygon::new(vec!(p64(0.0, 0.0), p64(0.5, 0.0), p64(1.0, 0.5), p64(0.5, 0.5)));
		assert_eq!(p64(0.0,0.0), a.lowest_unit_vertex().unwrap());

		let b = Polygon::new(vec!(p64(0.0, 0.7), p64(0.5, 0.0), p64(1.0, 0.5), p64(0.5, 0.9)));
		assert_eq!(p64(0.5,0.0), b.lowest_unit_vertex().unwrap());

		let c = Polygon::new(vec!(p64(0.0, 2.0), p64(1.0, 2.0), p64(1.0, 3.0), p64(0.0, 3.0)));
		assert_eq!(None, c.lowest_unit_vertex());
	}

	#[test]
	fn test_interpolate() {
		let line1 = Line::new(p64(0.0, 0.0), p64(1.0, 3.0));
		assert_eq!(p64(0.5, 1.5), line1.interpolate(0.5));
		assert_eq!(p64(0.125, 0.375), line1.interpolate(0.125));
	}

	#[test]
	fn test_split() {
		let (start, mid, end) = (p64(1.0, 1.5), p64(1.125, 2.0), p64(1.25, 2.5));
		let line1 = Line::new(start.clone(), end.clone());
		assert_eq!((Line::new(start.clone(), mid.clone()), Line::new(mid.clone(), end.clone())), line1.split(0.5))
	}

	#[test]
	fn test_commutivity() {
		let (p1, p2) = (p64(1.0, 1.5), p64(1.25, 2.5));
		assert_eq!(p64(2.25, 4.0), &p1 + &p2);
		assert_eq!(p64(2.25, 4.0), &p2 + &p1);
		assert_eq!(p64(3.5, 6.5), &p1 + &(p2.scale(2.0)));
		assert_eq!(p64(3.25, 5.5), &(p1.scale(2.0)) + &p2);
	}
}
